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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_to_number_vector() {
        assert_eq!(parse_line_to_number_vector("1 2 3"), vec![1, 2, 3]);
        assert_eq!(parse_line_to_number_vector("10 -5 0"), vec![10, -5, 0]);
        assert_eq!(parse_line_to_number_vector(""), vec![]);
    }

    #[test]
    fn test_is_difference_within_safe_range() {
        assert!(is_difference_within_safe_range(1));
        assert!(is_difference_within_safe_range(3));
        assert!(!is_difference_within_safe_range(4));
        assert!(!is_difference_within_safe_range(0));
    }

    #[test]
    fn test_is_monotonic() {
        assert_eq!(is_monotonic(&[1, 2, 3]), Some(Ordering::Less));
        assert_eq!(is_monotonic(&[3, 2, 1]), Some(Ordering::Greater));
        assert_eq!(is_monotonic(&[1, 2, 1]), None);
        assert_eq!(is_monotonic(&[1, 1, 1]), None);
    }

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&[1, 2, 3]));
        assert!(is_safe(&[3, 2, 1]));
        assert!(!is_safe(&[1, 5, 5]));
    }

    #[test]
    fn test_is_safe_or_is_safe_after_removing_one_sample() {
        assert!(is_safe_or_is_safe_after_removing_one_sample(&[1, 2, 3]));
        assert!(is_safe_or_is_safe_after_removing_one_sample(&[3, 2, 1]));
        assert!(is_safe_or_is_safe_after_removing_one_sample(&[1, 2, 4]));
        assert!(is_safe_or_is_safe_after_removing_one_sample(&[1, 5, 2]));
        assert!(!is_safe_or_is_safe_after_removing_one_sample(&[1, 5, 5, 2]));
    }

    #[test]
    fn test_count_safe_lines() {
        let lines = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(count_safe_lines(&lines), 4);
    }
}
