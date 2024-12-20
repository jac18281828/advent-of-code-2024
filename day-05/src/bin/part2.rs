use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::process::exit;

use day_05::{build_page_order_table, check_rules_for_line, split_rule, split_rules_and_updates};

fn topological_sort(nodes: &HashSet<&str>, edges: &Vec<(&str, &str)>) -> Option<Vec<String>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut in_degree: HashMap<&str, usize> = HashMap::new();

    // Initialize in-degree for each node
    for &node in nodes {
        in_degree.insert(node, 0);
    }

    // Build graph and calculate in-degrees
    for &(u, v) in edges {
        graph.entry(u).or_default().push(v);
        *in_degree.entry(v).or_default() += 1;
    }

    // Queue for nodes with in-degree 0
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node);
        }
    }

    let mut sorted_order = Vec::new();

    while let Some(current) = queue.pop_front() {
        sorted_order.push(current);

        if let Some(neighbors) = graph.get(current) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    // Check if sorting was successful
    if sorted_order.len() == nodes.len() {
        let sorted_order: Vec<String> = sorted_order.iter().map(|s| s.to_string()).collect();
        Some(sorted_order)
    } else {
        // Identify nodes involved in cycles
        let remaining_nodes: HashSet<&str> = nodes
            .iter()
            .filter(|&node| !sorted_order.contains(&node))
            .cloned()
            .collect();
        eprintln!("Cycle detected involving nodes: {:?}", remaining_nodes);
        None // Cycle detected or invalid rules
    }
}

fn repair_update(update: Vec<&str>, rules: &Vec<&str>) -> Option<Vec<String>> {
    // Parse and trim the rules
    let edges: Vec<(&str, &str)> = rules
        .iter()
        .filter_map(|rule| {
            let parts: Vec<&str> = rule.split('|').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                Some((parts[0], parts[1]))
            } else {
                eprintln!("Invalid rule format: {}", rule);
                None
            }
        })
        .collect();

    // Collect all unique nodes
    let mut nodes: HashSet<&str> = HashSet::new();
    for &(u, v) in &edges {
        nodes.insert(u);
        nodes.insert(v);
    }

    // Perform topological sort
    if let Some(correct_order) = topological_sort(&nodes, &edges) {
        // Filter to include only nodes present in the update
        let filtered_order: Vec<String> = correct_order
            .into_iter()
            .filter(|node| update.iter().any(|&u| u.trim() == node))
            .collect();
        return Some(filtered_order);
    }
    None
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let (rules, updates) = split_rules_and_updates(&lines);

    let mut sum_of_valid_middle_page = 0;
    for update in updates.iter() {
        if !check_rules_for_line(update, &rules) {
            let page_order_table = build_page_order_table(update);

            let relevant_rule: Vec<String> = rules
                .iter()
                .map(|r| split_rule(r))
                .filter(|(a, b)| page_order_table.contains(*a) && page_order_table.contains(*b))
                .map(|(a, b)| [a.to_string(), b.to_string()].join("|"))
                .collect();

            let update_vec = update.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
            let rules_vec = relevant_rule
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();

            if let Some(repaired_update) = repair_update(update_vec, &rules_vec) {
                if repaired_update.is_empty() {
                    eprintln!("Repaired update is empty.");
                    continue;
                }
                let middle = repaired_update.len() / 2;
                let middle_value = repaired_update[middle].parse::<usize>().unwrap_or(0);
                sum_of_valid_middle_page += middle_value;
            } else {
                eprintln!("Failed to repair update: {:?}", update);
                exit(1);
            }
        }
    }
    println!("{}", sum_of_valid_middle_page);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topological_sort() {
        // Define a sample graph with nodes and edges
        let nodes: HashSet<&str> = ["A", "B", "C", "D", "E"].iter().cloned().collect();
        let edges: Vec<(&str, &str)> =
            vec![("A", "B"), ("A", "C"), ("B", "D"), ("C", "D"), ("D", "E")];

        // Perform topological sort
        let result = topological_sort(&nodes, &edges);

        // Verify the output
        assert!(result.is_some()); // Ensure a valid result exists
        let sorted_order = result.unwrap();
        assert_eq!(sorted_order, vec!["A", "B", "C", "D", "E"]);
    }

    #[test]
    fn test_topological_sort_with_cycle() {
        // Define a cyclic graph
        let nodes: HashSet<&str> = ["A", "B", "C"].iter().cloned().collect();
        let edges: Vec<(&str, &str)> = vec![
            ("A", "B"),
            ("B", "C"),
            ("C", "A"), // Cycle: C -> A
        ];

        // Perform topological sort
        let result = topological_sort(&nodes, &edges);

        // Verify that no valid result exists
        assert!(result.is_none());
    }

    #[test]
    fn test_topological_sort_empty_graph() {
        // Define an empty graph
        let nodes: HashSet<&str> = HashSet::new();
        let edges: Vec<(&str, &str)> = vec![];

        // Perform topological sort
        let result = topological_sort(&nodes, &edges);

        // Verify the output for an empty graph
        assert_eq!(result, Some(vec![])); // Empty graph should return an empty order
    }

    #[test]
    fn test_repair_update() {
        let rules = vec!["a|b", "b|c", "c|d", "d|e", "e|f", "f|g", "g|h"];
        let update = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let repaired_update = repair_update(update, &rules).unwrap();
        assert_eq!(
            repaired_update,
            vec!["a", "b", "c", "d", "e", "f", "g", "h"]
        );
    }
}
