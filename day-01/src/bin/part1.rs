use itertools::izip;
use std::io;

fn parse_lines_and_sum_differences(lines: &[String]) -> i32 {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];

    lines.iter().for_each(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        l1.push(parts[0].parse::<i32>().unwrap());
        l2.push(parts[1].parse::<i32>().unwrap());
    });
    l1.sort();
    l2.sort();
    let mut sum_distance = 0;
    for (x1, x2) in izip!(l1.iter(), l2.iter()) {
        sum_distance += (x1 - x2).abs();
    }
    sum_distance
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let sum_distance = parse_lines_and_sum_differences(&lines);
    println!("{}", sum_distance);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines_and_sum_differences() {
        let lines = vec![
            "1 8".to_string(),
            "3 6".to_string(),
            "5 4".to_string(),
            "7 2".to_string(),
        ];
        assert_eq!(parse_lines_and_sum_differences(&lines), 4);
    }

    #[test]
    fn test_parse_lines_and_sum_differences_2() {
        let lines = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(parse_lines_and_sum_differences(&lines), 11);
    }
}
