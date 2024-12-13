use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Debug)]
struct PageRelations {
    before: HashSet<usize>,
}

impl PageRelations {
    fn new() -> Self {
        Self {
            before: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct PageOrderTable {
    pages: HashMap<usize, PageRelations>,
}

impl PageOrderTable {
    fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    fn contains(&self, page: usize) -> bool {
        self.pages.contains_key(&page)
    }

    // Check if p1 is before p2
    fn is_before(&self, p1: usize, p2: usize) -> bool {
        self.pages
            .get(&p2)
            .map(|relations| relations.before.contains(&p1))
            .unwrap_or(false)
    }
}

fn build_page_order_table(updates: &str) -> PageOrderTable {
    let mut table = PageOrderTable::new();

    let updates = updates
        .split(',')
        .flat_map(|s| s.parse())
        .collect::<Vec<usize>>();
    let mut before_pages: HashSet<usize> = HashSet::new();
    for page in updates {
        let mut relation = PageRelations::new();
        relation.before = before_pages.clone();
        before_pages.insert(page);
        table.pages.insert(page, relation);
    }
    table
}

fn split_rule(rule: &str) -> (usize, usize) {
    let parts = rule.split('|').collect::<Vec<&str>>();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn split_rules_and_updates(lines: &[String]) -> (Vec<String>, Vec<String>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut is_rule = true;
    for line in lines {
        if line.is_empty() {
            is_rule = false;
            continue;
        }
        if is_rule {
            rules.push(line.clone());
        } else {
            updates.push(line.clone());
        }
    }
    (rules, updates)
}

fn check_rules_for_line(line: &str, rules: &[String]) -> bool {
    let page_order_table = build_page_order_table(line);

    let relevant_rule: Vec<(usize, usize)> = rules
        .iter()
        .map(|r| split_rule(r))
        .filter(|(a, b)| page_order_table.contains(*a) && page_order_table.contains(*b))
        .collect();

    for rule in relevant_rule.iter() {
        // Adjusted the call to is_before
        if !page_order_table.is_before(rule.0, rule.1) {
            return false;
        }
    }
    true
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let (rules, updates) = split_rules_and_updates(&lines);
    let mut sum_of_valid_middle_page = 0;
    for page in updates.iter() {
        if check_rules_for_line(page, &rules) {
            let page_value: Vec<usize> = page
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let middle = page_value.len() / 2;
            let middle_value = page_value[middle];
            sum_of_valid_middle_page += middle_value;
        }
    }
    println!("{}", sum_of_valid_middle_page);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decode_rules_updates() {
        let (rules, updates) = split_rules_and_updates(&sample_data());
        rules.iter().for_each(|r| println!("{}", r));
        assert_eq!(rules.len(), 21);
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_build_order_table() {
        let (_, updates) = split_rules_and_updates(&sample_data());
        for update in updates.iter() {
            let table = build_page_order_table(update);
            let pages = update.split(',').map(|s| s.parse().unwrap());
            let mut before_pages: HashSet<usize> = HashSet::new();
            for page in pages {
                table.contains(page);
                for bp in before_pages.iter() {
                    println!("{} is before {}", bp, page);
                    assert!(table.is_before(*bp, page));
                }
                before_pages.insert(page);
            }
        }
    }

    #[test]
    fn test_split_rule() {
        assert_eq!(split_rule("47|53"), (47, 53));
    }

    #[test]
    fn test_check_valid_lines() {
        let lines = ["75,47,61,53,29", "97,61,53,29,13", "75,29,13"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (rules, _) = split_rules_and_updates(&sample_data());
        for line in lines.iter() {
            assert!(check_rules_for_line(line, &rules));
        }
    }

    #[test]
    fn test_check_bad_lines() {
        let lines = ["61,13,29", "75,97,47,61,53", "97,13,75,29,47"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let (rules, _) = split_rules_and_updates(&sample_data());
        for line in lines.iter() {
            assert!(!check_rules_for_line(line, &rules));
        }
    }

    #[test]
    fn test_build_page_order_table() {
        let table = build_page_order_table("75,47,61,53,29");
        assert!(table.contains(75));
        assert!(table.contains(47));
        assert!(table.contains(61));
        assert!(table.contains(53));
        assert!(table.contains(29));
        assert!(!table.contains(13));
        // Adjusted the call to is_before
        assert!(table.is_before(47, 53)); // Now 47 is before 53
    }

    fn sample_data() -> Vec<String> {
        let lines = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        lines
    }
}
