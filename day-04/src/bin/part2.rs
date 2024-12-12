use std::io;

use day_04::lettermap::LetterMap;

// Search for MAS in the shape of an X
//
// here are the ways to write MAS in the shape of an X
// down right and up right
// M . S
// . A .
// M . S
//
// down left and down right
// S . M
// . A .
// S . M
//
// down right and up left
// M . M
// . A .
// S . S
//
// up right and down left
// S . S
// . A .
// M . M
pub fn search(map: &LetterMap, word: &str) -> Option<u64> {
    let n = word.len();
    if n == 0 {
        return None;
    }
    // An X pattern requires a center. If word length is even, no single center exists.
    if n % 2 == 0 {
        return None;
    }

    let mid = n / 2;
    let wchars: Vec<char> = word.chars().collect();
    let revchars: Vec<char> = wchars.iter().copied().rev().collect();

    let width = map.get_width();
    let height = map.get_height();

    let mut match_count = 0;

    for y in 0..height {
        for x in 0..width {
            // The center of the X must match the middle character of the word
            if let Some(&center_ch) = map.get(x, y) {
                if center_ch != wchars[mid] {
                    continue;
                }

                // Gather characters along the two diagonals that would form the X
                // Diagonal \: from (x-mid,y-mid) to (x+mid,y+mid)
                // Diagonal /: from (x+mid,y-mid) to (x-mid,y+mid)

                // Check bounds first:
                if x < mid || y < mid || x + mid >= width || y + mid >= height {
                    // The \ diagonal doesn't fit inside the grid
                    continue;
                }
                if y < mid || x + mid >= width || y + mid >= height || x < mid {
                    // The / diagonal doesn't fit inside the grid (re-checking carefully)
                    // Actually we need to ensure that all these indices fit:
                    if x < mid || y + mid >= height || x + mid >= width || y < mid {
                        continue;
                    }
                }

                // Extract the \ diagonal chars
                let mut diag_backslash = Vec::with_capacity(n);
                for i in 0..n {
                    let dx = (x + i) as isize - mid as isize;
                    let dy = (y + i) as isize - mid as isize;
                    if dx < 0 || dy < 0 {
                        diag_backslash.clear();
                        break;
                    }
                    let dxu = dx as usize;
                    let dyu = dy as usize;
                    if dxu >= width || dyu >= height {
                        diag_backslash.clear();
                        break;
                    }
                    if let Some(&ch) = map.get(dxu, dyu) {
                        diag_backslash.push(ch);
                    } else {
                        diag_backslash.clear();
                        break;
                    }
                }

                if diag_backslash.len() != n {
                    // Can't form the \ diagonal properly
                    continue;
                }

                // Extract the / diagonal chars
                let mut diag_slash = Vec::with_capacity(n);
                for i in 0..n {
                    let dx = (x + mid) as isize - i as isize;
                    let dy = (y + i) as isize - mid as isize;
                    if dx < 0 || dy < 0 {
                        diag_slash.clear();
                        break;
                    }
                    let dxu = dx as usize;
                    let dyu = dy as usize;
                    if dxu >= width || dyu >= height {
                        diag_slash.clear();
                        break;
                    }
                    if let Some(&ch) = map.get(dxu, dyu) {
                        diag_slash.push(ch);
                    } else {
                        diag_slash.clear();
                        break;
                    }
                }

                if diag_slash.len() != n {
                    // Can't form the / diagonal properly
                    continue;
                }

                // Now we have two diagonals: diag_backslash and diag_slash
                // Check the four combinations:
                // 1. \ = word, / = word
                // 2. \ = word, / = rev
                // 3. \ = rev,  / = word
                // 4. \ = rev,  / = rev
                let backslash_word = diag_backslash == wchars;
                let backslash_rev = diag_backslash == revchars;
                let slash_word = diag_slash == wchars;
                let slash_rev = diag_slash == revchars;

                if (backslash_word || backslash_rev) && (slash_word || slash_rev) {
                    match_count += 1;
                }
            }
        }
    }

    if match_count > 0 {
        Some(match_count)
    } else {
        Some(0)
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
    let word = "MAS";
    match search(&map, word) {
        Some(count) => println!("{}", count),
        None => println!("0"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search1() {
        let mut map = LetterMap::new(3, 3);
        let test_string = "M.S.A.M.S";
        assert!(map.set_string(test_string));
        assert_eq!(search(&map, "MAS"), Some(1));
    }

    #[test]
    fn test_search2() {
        let mut map = LetterMap::new(3, 3);
        let test_string = "S.M.A.S.M";
        assert!(map.set_string(test_string));
        assert_eq!(search(&map, "MAS"), Some(1));
    }

    #[test]
    fn test_search3() {
        let mut map = LetterMap::new(3, 3);
        let test_string = "M.M.A.S.S";
        assert!(map.set_string(test_string));
        assert_eq!(search(&map, "MAS"), Some(1));
    }

    #[test]
    fn test_search4() {
        let mut map = LetterMap::new(3, 3);
        let test_string = "S.S.A.M.M";
        assert!(map.set_string(test_string));
        assert_eq!(search(&map, "MAS"), Some(1));
    }

    #[test]
    fn test_search_no_mas() {
        let mut map = LetterMap::new(3, 3);
        let test_string = "M.M.A.M.M";
        assert!(map.set_string(test_string));
        assert_eq!(search(&map, "MAS"), Some(0));
    }

    #[test]
    fn test_search_not_found() {
        let mut map = LetterMap::new(10, 10);
        let test_string = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuv";
        assert!(map.set_string(test_string));

        assert_eq!(search(&map, "vxw"), Some(0));
        assert_eq!(search(&map, "uwv"), Some(0));
        assert_eq!(search(&map, "tvu"), Some(0));
        assert_eq!(search(&map, "rts"), Some(0));
        assert_eq!(search(&map, "qsr"), Some(0));
    }

    #[test]
    fn test_example_search() {
        let map_string = [
            ".M.S......".to_string(),
            "..A..MSMS.".to_string(),
            ".M.S.MAA..".to_string(),
            "..A.ASMSM.".to_string(),
            ".M.S.M....".to_string(),
            "..........".to_string(),
            "S.S.S.S.S.".to_string(),
            ".A.A.A.A..".to_string(),
            "M.M.M.M.M.".to_string(),
            "........A.".to_string(),
        ]
        .join("");
        let mut map = LetterMap::new(10, 10);
        assert!(map.set_string(&map_string));
        let word = "MAS";
        assert_eq!(search(&map, word), Some(9));
    }
}
