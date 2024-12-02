use std::io;

const MAX_SAFE_DIFFERENCE: i32 = 3;

fn parse_line_to_number_vector(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn is_safe_difference(difference: i32) -> bool {
    difference.abs() > 0 && difference.abs() <= MAX_SAFE_DIFFERENCE
}

fn is_increasing(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] >= numbers[i + 1] {
            return false;
        }
    }
    true
}

fn is_decreasing(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() - 1 {
        if numbers[i] <= numbers[i + 1] {
            return false;
        }
    }
    true
}

fn is_safe(numbers: &[i32]) -> bool {
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

fn count_safe_lines(lines: &[String]) -> i32 {
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
        let lines = vec![
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
        let lines = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        let expect = vec![true, false, false, false, false, true];
        for (line, &expect) in lines.iter().zip(expect.iter()) {
            let number_vec = parse_line_to_number_vector(line);
            assert_eq!(is_safe(&number_vec), expect);
        }
    }

    #[test]
    fn test_broken_line() {
        let line = "9 7 6 2 1".to_string();
        let number_vec = parse_line_to_number_vector(&line);
        assert_eq!(is_safe(&number_vec), false);
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
        assert_eq!(is_safe_difference(-1), true);
        assert_eq!(is_safe_difference(-2), true);
        assert_eq!(is_safe_difference(-3), true);
        assert_eq!(is_safe_difference(-4), false);
    }

    #[test]
    fn test_is_safe_difference_positive() {
        assert_eq!(is_safe_difference(0), false);
        assert_eq!(is_safe_difference(1), true);
        assert_eq!(is_safe_difference(2), true);
        assert_eq!(is_safe_difference(3), true);
        assert_eq!(is_safe_difference(4), false);
    }

    #[test]
    fn test_is_increasing() {
        assert_eq!(is_increasing(&[1, 2, 3, 4, 5]), true);
        assert_eq!(is_increasing(&[1, 2, 3, 3, 5]), false);
        assert_eq!(is_increasing(&[1, 2, 3, 2, 5]), false);
        assert_eq!(is_increasing(&[1, 2, 3, 4, 3]), false);
        assert_eq!(is_increasing(&[1, 2, 2, 4, 5]), false);
    }

    #[test]
    fn test_is_decreasing() {
        assert_eq!(is_decreasing(&[5, 4, 3, 2, 1]), true);
        assert_eq!(is_decreasing(&[5, 4, 3, 3, 1]), false);
        assert_eq!(is_decreasing(&[5, 4, 3, 2, 3]), false);
        assert_eq!(is_decreasing(&[5, 4, 3, 2, 4]), false);
        assert_eq!(is_decreasing(&[5, 4, 4, 2, 1]), false);
    }
}
