use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

// struct representing a 2D map of characters
#[derive(Clone, Debug, PartialEq)]
pub struct LetterMap {
    width: usize,
    height: usize,
    map: HashMap<(usize, usize), char>,
}

impl LetterMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.map.insert((x, y), c);
        true
    }

    pub fn set_string(&mut self, s: &str) -> bool {
        if self.width * self.height != s.len() {
            return false;
        }

        for (i, c) in s.chars().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            self.set(x, y, c);
        }
        true
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.map.get(&(x, y))
    }

    pub fn is_set(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.map.contains_key(&(x, y))
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn rotate_45(&self) -> LetterMap {
        let width = self.get_width() as isize;
        let height = self.get_height() as isize;

        // Calculate the range of new coordinates
        let min_u = 0;
        let max_u = (width - 1) + (height - 1);

        let min_v = -(height - 1);
        let max_v = width - 1;

        let new_width = max_u - min_u + 1;
        let new_height = max_v - min_v + 1;

        let v_offset = -min_v;

        let mut new_map = LetterMap::new(new_width as usize, new_height as usize);

        for y in 0..height {
            for x in 0..width {
                let x_orig = x;
                let y_orig = y;

                let u = x_orig + y_orig;
                let v = x_orig - y_orig;

                let v_shifted = (v + v_offset) as usize;

                if let Some(&ch) = self.get(x as usize, y as usize) {
                    new_map.set(u as usize, v_shifted, ch);
                }
            }
        }

        new_map
    }
}

impl Display for LetterMap {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(ch) = self.get(x, y) {
                    write!(f, "{}", ch)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut map = LetterMap::new(10, 10);
        assert!(map.set(0, 0, 'a'));
        assert_eq!(map.get(0, 0), Some(&'a'));
        assert_eq!(map.get(1, 0), None);
        assert_eq!(map.get(0, 1), None);
    }

    #[test]
    fn test_is_set() {
        let mut map = LetterMap::new(10, 10);
        assert!(map.set(0, 0, 'a'));
        assert!(map.is_set(0, 0));
        assert!(!map.is_set(1, 0));
        assert!(!map.is_set(0, 1));
    }

    #[test]
    fn test_get_width_height() {
        let map = LetterMap::new(10, 20);
        assert_eq!(map.get_width(), 10);
        assert_eq!(map.get_height(), 20);
    }

    #[test]
    fn test_set_string() {
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
        assert_eq!(map.get(0, 0), Some(&'M'));
        assert_eq!(map.get(9, 9), Some(&'X'));
        assert_eq!(map.get(10, 0), None);
        assert_eq!(map.get(0, 10), None);
    }

    #[test]
    fn test_rotate_45() {
        let map_string = ["M.S".to_string(), ".A.".to_string(), "M.S".to_string()].join("");
        let mut map = LetterMap::new(3, 3);
        assert!(map.set_string(&map_string));
        let rotated = map.rotate_45();
        let width = rotated.get_width();
        let height = rotated.get_height();
        assert_eq!(width, 5);
        assert_eq!(height, 5);
        let mut expect_map = LetterMap::new(width, height);
        expect_map.set(2, 0, 'M');
        expect_map.set(1, 1, '.');
        expect_map.set(1, 3, '.');
        expect_map.set(3, 1, '.');
        expect_map.set(3, 3, '.');
        expect_map.set(0, 2, 'M');
        expect_map.set(4, 2, 'S');
        expect_map.set(2, 4, 'S');
        expect_map.set(2, 2, 'A');
        assert_eq!(rotated, expect_map);
    }
}
