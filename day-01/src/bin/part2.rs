use std::io;

fn main() {
    let mut l1: Vec<i32> = vec![];
    let mut count_map = std::collections::HashMap::new();
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
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
    println!("{}", sum_similarity);
}
