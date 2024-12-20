use std::io;

use day_05::{check_rules_for_line, split_rules_and_updates};

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let (rules, updates) = split_rules_and_updates(&lines);
    let mut sum_of_valid_middle_page = 0;
    for update in updates.iter() {
        if check_rules_for_line(update, &rules) {
            let page_value: Vec<usize> = update
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
