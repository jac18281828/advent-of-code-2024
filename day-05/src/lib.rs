use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct PageRelations {
    before: HashSet<usize>,
}

impl PageRelations {
    pub fn new() -> Self {
        Self {
            before: HashSet::new(),
        }
    }
}

impl Default for PageOrderTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PageOrderTable {
    pages: HashMap<usize, PageRelations>,
}

impl PageOrderTable {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    pub fn contains(&self, page: usize) -> bool {
        self.pages.contains_key(&page)
    }

    // Check if p1 is before p2
    pub fn is_before(&self, p1: usize, p2: usize) -> bool {
        self.pages
            .get(&p2)
            .map(|relations| relations.before.contains(&p1))
            .unwrap_or(false)
    }
}

pub fn build_page_order_table(updates: &str) -> PageOrderTable {
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

pub fn split_rule(rule: &str) -> (usize, usize) {
    let parts = rule.split('|').collect::<Vec<&str>>();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

pub fn split_rules_and_updates(lines: &[String]) -> (Vec<String>, Vec<String>) {
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

pub fn permute_update(update: &str) -> Vec<String> {
    let parts = update.split(',').collect::<Vec<&str>>();
    let mut permutations = Vec::new();
    for i in 0..parts.len() {
        for j in i + 1..parts.len() {
            let mut new_parts = parts.clone();
            new_parts.swap(i, j);
            permutations.push(new_parts.join(","));
        }
    }
    permutations
}

pub fn check_rules_for_line(line: &str, rules: &[String]) -> bool {
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

#[cfg(test)]
pub mod test_data;

#[cfg(test)]
mod tests {
    use super::test_data::data_helper::sample_data;
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
}
