use std::io;

const MAX_SAFE_DIFFERENCE: i64 = 3;

fn parse_line_to_number_vector(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn is_safe_difference(difference: i64) -> bool {
    difference.abs() > 0 && difference.abs() <= MAX_SAFE_DIFFERENCE
}

fn is_increasing(numbers: &[i64]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] >= numbers[i + 1] {
            return false;
        }
    }
    true
}

fn is_decreasing(numbers: &[i64]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] <= numbers[i + 1] {
            return false;
        }
    }
    true
}

fn is_safe(numbers: &[i64]) -> bool {
    if is_decreasing(numbers) || is_increasing(numbers) {
        for i in 0..numbers.len() - 1 {
            if !is_safe_difference(numbers[i + 1] - numbers[i]) {
                return false;
            }
        }
        return true;
    }
    false
}

fn count_safe_lines(lines: &[String]) -> i64 {
    let mut safe_lines = 0;
    for line in lines {
        let number_vec = parse_line_to_number_vector(line);
        if is_safe(&number_vec) {
            safe_lines += 1;
        }
    }
    safe_lines
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
    fn test_count_safe_lines() {
        let lines = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(count_safe_lines(&lines), 2);
    }

    #[test]
    fn test_count_safe_line_check() {
        let lines = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        let expect = [true, false, false, false, false, true];
        for (line, &expect) in lines.iter().zip(expect.iter()) {
            let number_vec = parse_line_to_number_vector(line);
            assert_eq!(is_safe(&number_vec), expect);
        }
    }

    #[test]
    fn test_broken_line() {
        let line = "9 7 6 2 1".to_string();
        let number_vec = parse_line_to_number_vector(&line);
        assert!(!is_safe(&number_vec));
    }

    #[test]
    fn test_parse_line_to_number_vector() {
        let line = "1 2 3 4 5";
        assert_eq!(parse_line_to_number_vector(line), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_line_to_number_vector_ignore_whitespace() {
        let line = " 1   2 3  4 5\t \n";
        assert_eq!(parse_line_to_number_vector(line), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_is_safe_difference_negative() {
        assert!(is_safe_difference(-1));
        assert!(is_safe_difference(-2));
        assert!(is_safe_difference(-3));
        assert!(!is_safe_difference(-4));
    }

    #[test]
    fn test_is_safe_difference_positive() {
        assert!(!is_safe_difference(0));
        assert!(is_safe_difference(1));
        assert!(is_safe_difference(2));
        assert!(is_safe_difference(3));
        assert!(!is_safe_difference(4));
    }

    #[test]
    fn test_is_increasing() {
        assert!(is_increasing(&[1, 2, 3, 4, 5]));
        assert!(!is_increasing(&[1, 2, 3, 3, 5]));
        assert!(!is_increasing(&[1, 2, 3, 2, 5]));
        assert!(!is_increasing(&[1, 2, 3, 4, 3]));
        assert!(!is_increasing(&[1, 2, 2, 4, 5]));
    }

    #[test]
    fn test_is_decreasing() {
        assert!(is_decreasing(&[5, 4, 3, 2, 1]));
        assert!(!is_decreasing(&[5, 4, 3, 3, 1]));
        assert!(!is_decreasing(&[5, 4, 3, 2, 3]));
        assert!(!is_decreasing(&[5, 4, 3, 2, 4]));
        assert!(!is_decreasing(&[5, 4, 4, 2, 1]));
    }
}
