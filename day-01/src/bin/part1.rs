use std::io;
use itertools::izip;

fn main() {
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    lines.iter().for_each(|line| {
        let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
        l1.push(parts[0].parse::<i32>().unwrap());
        l2.push(parts[1].parse::<i32>().unwrap());
    });
    l1.sort();
    l2.sort();
    let mut sum_distance = 0;
    for (x1, x2) in izip!(l1.iter(), l2.iter()) {
        sum_distance = sum_distance + (x1 - x2).abs();
    }
    println!("{}", sum_distance);
}
