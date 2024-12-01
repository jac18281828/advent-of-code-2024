use std::io;

fn parse_lines_and_sum_similarity_score(lines: &Vec<String>) -> i32 {
    let mut l1: Vec<i32> = vec![];
    let mut count_map = std::collections::HashMap::new();
    lines.iter().for_each(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let x1 = parts[0].parse::<i32>().unwrap();
        l1.push(x1);
        count_map.entry(x1).or_insert(0);
        let x2 = parts[1].parse::<i32>().unwrap();
        let count = count_map.entry(x2).or_insert(0);
        *count += 1;
    });
    let mut sum_similarity = 0;
    for x1 in l1.iter() {
        let count = count_map.get_mut(&x1).unwrap();
        sum_similarity = sum_similarity + *count * x1;
    }
    sum_similarity
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let sum_similarity = parse_lines_and_sum_similarity_score(&lines);
    println!("{}", sum_similarity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines_and_sum_similarity_score() {
        let lines = vec![
            "1 8".to_string(),
            "3 6".to_string(),
            "5 4".to_string(),
            "7 2".to_string(),
        ];
        assert_eq!(parse_lines_and_sum_similarity_score(&lines), 0);
    }

    #[test]
    fn test_parse_lines_and_sum_similarity_score_2() {
        let lines = vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(parse_lines_and_sum_similarity_score(&lines), 31);
    }
}
