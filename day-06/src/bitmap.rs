use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitmapError {
    #[error("Index out of bounds")]
    OutOfBounds,
}

#[derive(Debug, PartialEq)]
pub struct Bitmap<G> {
    data: Vec<Option<G>>,
    width: usize,
    height: usize,
}

impl<G> Bitmap<G> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: (0..width * height).map(|_| None).collect(),
            width,
            height,
        }
    }

    pub fn is_set(&self, x: usize, y: usize) -> bool {
        if let Some(dx) = self.index_position(x, y) {
            return self.data.get(dx).unwrap().is_some();
        }
        false
    }

    pub fn is_set_check_bounds(&self, x: usize, y: usize) -> Result<bool, BitmapError> {
        if let Some(dx) = self.index_position(x, y) {
            return Ok(self.data.get(dx).is_some());
        }
        Err(BitmapError::OutOfBounds)
    }

    pub fn get_check_bounds(&self, x: usize, y: usize) -> Result<&Option<G>, BitmapError> {
        if let Some(dx) = self.index_position(x, y) {
            return Ok(self.data.get(dx).unwrap());
        }
        Err(BitmapError::OutOfBounds)
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&G> {
        let index = self.index_position(x, y);
        if let Some(dx) = index {
            return self.data.get(dx).unwrap().as_ref();
        }
        None
    }

    pub fn set(&mut self, x: usize, y: usize, value: G) -> Result<(), BitmapError> {
        let index = self.index_position(x, y);
        if let Some(dx) = index {
            self.data[dx] = Some(value);
            return Ok(());
        }
        Err(BitmapError::OutOfBounds)
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        let index = self.index_position(x, y);
        if let Some(dx) = index {
            self.data[dx] = None;
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn index_position(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width() + x)
    }
}

impl Bitmap<char> {
    pub fn import(lines: &[String]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut bitmap: Bitmap<char> = Bitmap::new(width, height);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                bitmap.set(x, y, c).expect("Set must succeed");
            }
        }
        bitmap
    }

    pub fn print(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let c = self.get(x, y).unwrap_or(&' ');
                print!("{}", c);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_new() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert_eq!(bitmap.width(), 10);
        assert_eq!(bitmap.height(), 10);
        for i in 0..100 {
            for j in 0..100 {
                assert!(bitmap.get(i, j).is_none());
            }
        }
    }

    #[test]
    fn test_bitmap_set_and_get() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert_eq!(bitmap.get(2, 3), Some(&42));
    }

    #[test]
    fn test_bitmap_clear() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert!(bitmap.get(2, 3).is_some(), "Value must be set");
        bitmap.clear(2, 3);
        assert!(bitmap.get(2, 3).is_none(), "value must be cleared");
    }

    #[test]
    fn test_bitmap_is_set() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(!bitmap.is_set(2, 3), "Value must not be set");
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert!(bitmap.is_set(2, 3), "Value must be set");
    }

    #[test]
    fn test_bitmap_out_of_bounds() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.get(10, 10).is_none());
        assert!(bitmap.get(11, 0).is_none());
        assert!(bitmap.get(0, 11).is_none());
    }

    #[test]
    fn test_bitmap_is_set_check_bounds() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.is_set_check_bounds(10, 10).is_err());
        assert!(bitmap.is_set_check_bounds(11, 0).is_err());
        assert!(bitmap.is_set_check_bounds(0, 11).is_err());
    }

    #[test]
    fn test_bitmap_get_check_bounds() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.get_check_bounds(10, 10).is_err());
        assert!(bitmap.get_check_bounds(11, 0).is_err());
        assert!(bitmap.get_check_bounds(0, 11).is_err());
    }

    #[test]
    fn test_bitmap_set_out_of_bounds() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.set(10, 10, 42).is_err());
        assert!(bitmap.set(11, 0, 42).is_err());
        assert!(bitmap.set(0, 11, 42).is_err());
    }

    #[test]
    fn test_bitmap_get_out_of_bounds() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.get(10, 10).is_none());
        assert!(bitmap.get(11, 0).is_none());
        assert!(bitmap.get(0, 11).is_none());
    }

    #[test]
    fn test_bitmap_clear_out_of_bounds() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        bitmap.clear(10, 10);
        bitmap.clear(11, 0);
        bitmap.clear(0, 11);
    }

    #[test]
    fn test_bitmap_is_set_out_of_bounds() {
        let bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(!bitmap.is_set(10, 10));
        assert!(!bitmap.is_set(11, 0));
        assert!(!bitmap.is_set(0, 11));
    }

    #[test]
    fn test_charmap() {
        let mut bitmap: Bitmap<char> = Bitmap::new(10, 10);
        bitmap.set(2, 3, 'a').expect("Set must succeed");
        assert_eq!(bitmap.get(2, 3), Some(&'a'));
    }

    #[test]
    fn test_import_char() {
        let lines = sample_data();
        let bitmap = Bitmap::import(&lines);
        assert_eq!(bitmap.width(), 10);
        assert_eq!(bitmap.height(), 10);
        assert_eq!(bitmap.get(0, 0), Some(&'.'));
        assert_eq!(bitmap.get(9, 9), Some(&'.'));
        assert_eq!(bitmap.get(4, 6), Some(&'^'));
    }

    fn sample_data() -> Vec<String> {
        let lines = [
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ];
        lines.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }
}
