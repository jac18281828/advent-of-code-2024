use std::collections::HashMap;

// struct representing a 2D map of characters
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
}
