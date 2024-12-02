use std::cmp::Ordering;
use std::io;

const MAX_SAFE_DIFFERENCE: i32 = 3;

fn parse_line_to_number_vector(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn is_difference_within_safe_range(diff: i32) -> bool {
    (1..=MAX_SAFE_DIFFERENCE).contains(&diff.abs())
}

fn is_monotonic(numbers: &[i32]) -> Option<Ordering> {
    if numbers.windows(2).all(|w| w[0] < w[1]) {
        Some(Ordering::Less)
    } else if numbers.windows(2).all(|w| w[0] > w[1]) {
        Some(Ordering::Greater)
    } else {
        None
    }
}

fn is_safe(numbers: &[i32]) -> bool {
    if is_monotonic(numbers).is_some() {
        numbers
            .windows(2)
            .all(|w| is_difference_within_safe_range(w[1] - w[0]))
    } else {
        false
    }
}

fn is_safe_or_is_safe_after_removing_one_sample(numbers: &[i32]) -> bool {
    if is_safe(numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        if is_safe(&[&numbers[..i], &numbers[i + 1..]].concat()) {
            return true;
        }
    }
    false
}

fn count_safe_lines(lines: &[String]) -> i32 {
    lines
        .iter()
        .map(|line| parse_line_to_number_vector(line))
        .filter(|nums| is_safe_or_is_safe_after_removing_one_sample(nums))
        .count() as i32
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let safe_lines = count_safe_lines(&lines);
    println!("{}", safe_lines);
}
