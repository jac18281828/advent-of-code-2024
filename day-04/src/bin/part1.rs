use std::io;

use day_04::lettermap::LetterMap;

// search LetterMap up, down, forward, backward, left, right, and diagonally for a word
// return the count
pub fn search(map: &LetterMap, word: &str) -> Option<u64> {
    let mut count = 0;
    let width = map.get_width();
    let height = map.get_height();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect(); // Precompute chars for efficiency

    for y in 0..height {
        for x in 0..width {
            // search right
            if x + word_len <= width
                && (0..word_len).all(|i| map.get(x + i, y) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search down
            if y + word_len <= height
                && (0..word_len).all(|i| map.get(x, y + i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search down-right
            if x + word_len <= width
                && y + word_len <= height
                && (0..word_len).all(|i| map.get(x + i, y + i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search down-left
            if x >= word_len - 1
                && y + word_len <= height
                && (0..word_len).all(|i| map.get(x - i, y + i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search up-right
            if x + word_len <= width
                && y >= word_len - 1
                && (0..word_len).all(|i| map.get(x + i, y - i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search up-left
            if x >= word_len - 1
                && y >= word_len - 1
                && (0..word_len).all(|i| map.get(x - i, y - i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search up
            if y >= word_len - 1 && (0..word_len).all(|i| map.get(x, y - i) == Some(&word_chars[i]))
            {
                count += 1;
            }

            // search left
            if x >= word_len - 1 && (0..word_len).all(|i| map.get(x - i, y) == Some(&word_chars[i]))
            {
                count += 1;
            }
        }
    }

    if count > 0 {
        Some(count)
    } else {
        None
    }
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut map = LetterMap::new(lines[0].len(), lines.len());
    let lines = lines.join("");
    if !map.set_string(&lines) {
        eprintln!("Error: Invalid input");
        return;
    }
    let word = "XMAS";
    match search(&map, word) {
        Some(count) => println!("{}", count),
        None => println!("0"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut map = LetterMap::new(10, 10);
        let test_string = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuv";
        assert!(map.set_string(test_string));

        assert_eq!(search(&map, "abc"), Some(3));
        assert_eq!(search(&map, "def"), Some(3));
        assert_eq!(search(&map, "ghi"), Some(3));
        assert_eq!(search(&map, "jkl"), Some(3));
        assert_eq!(search(&map, "mno"), Some(3));
        assert_eq!(search(&map, "pqr"), Some(4));
        assert_eq!(search(&map, "stu"), Some(3));
    }

    #[test]
    fn test_search_not_found() {
        let mut map = LetterMap::new(10, 10);
        let test_string = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuv";
        assert!(map.set_string(test_string));

        assert_eq!(search(&map, "vxw"), None);
        assert_eq!(search(&map, "uwv"), None);
        assert_eq!(search(&map, "tvu"), None);
        assert_eq!(search(&map, "rts"), None);
        assert_eq!(search(&map, "qsr"), None);
    }

    #[test]
    fn test_example_search() {
        let map_string = [
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ]
        .join("");
        let mut map = LetterMap::new(10, 10);
        assert!(map.set_string(&map_string));
        let word = "XMAS";
        assert_eq!(search(&map, word), Some(18));
    }
}
