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
        self.index_position(x, y)
            .and_then(|index| self.data.get(index))
            .map_or(false, Option::is_some)
    }

    pub fn get(&self, x: usize, y: usize) -> Result<Option<&G>, BitmapError> {
        self.index_position(x, y)
            .map(|index| self.data.get(index).unwrap().as_ref())
            .ok_or(BitmapError::OutOfBounds)
    }

    pub fn set(&mut self, x: usize, y: usize, value: G) -> Result<(), BitmapError> {
        if let Some(index) = self.index_position(x, y) {
            self.data[index] = Some(value);
            Ok(())
        } else {
            Err(BitmapError::OutOfBounds)
        }
    }
    pub fn clear(&mut self, x: usize, y: usize) -> Result<(), BitmapError> {
        if let Some(index) = self.index_position(x, y) {
            self.data[index] = None;
            Ok(())
        } else {
            Err(BitmapError::OutOfBounds)
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
        let width = lines.first().map_or(0, String::len);
        let height = lines.len();
        let mut bitmap = Bitmap::new(width, height);

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
                print!("{}", self.get(x, y).unwrap_or(Some(&' ')).unwrap_or(&' '));
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
        for i in 0..10 {
            for j in 0..10 {
                assert!(bitmap.get(i, j).expect("is in bounds").is_none());
            }
        }
    }

    #[test]
    fn test_bitmap_set_and_get() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert_eq!(bitmap.get(2, 3).expect("is in bounds"), Some(&42));
    }

    #[test]
    fn test_bitmap_clear() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert!(
            bitmap.get(2, 3).expect("is in bounds").is_some(),
            "Value must be set"
        );
        bitmap.clear(2, 3).expect("Clear must succeed");
        assert!(
            bitmap.get(2, 3).expect("is in bounds").is_none(),
            "value must be cleared"
        );
    }

    #[test]
    fn test_bitmap_is_set() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(!bitmap.is_set(2, 3), "Value must not be set");
        bitmap.set(2, 3, 42).expect("Set must succeed");
        assert!(bitmap.is_set(2, 3), "Value must be set");
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
        assert!(bitmap.get(10, 10).is_err());
        assert!(bitmap.get(11, 0).is_err());
        assert!(bitmap.get(0, 11).is_err());
    }

    #[test]
    fn test_bitmap_clear_out_of_bounds() {
        let mut bitmap: Bitmap<i32> = Bitmap::new(10, 10);
        assert!(bitmap.clear(10, 10).is_err());
        assert!(bitmap.clear(11, 0).is_err());
        assert!(bitmap.clear(0, 11).is_err());
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
        assert_eq!(bitmap.get(2, 3).expect("is in bounds"), Some(&'a'));
    }

    #[test]
    fn test_import_char() {
        let lines = sample_data();
        let bitmap = Bitmap::import(&lines);
        assert_eq!(bitmap.width(), 10);
        assert_eq!(bitmap.height(), 10);
        assert_eq!(bitmap.get(0, 0).expect("is in bounds"), Some(&'.'));
        assert_eq!(bitmap.get(9, 9).expect("is in bounds"), Some(&'.'));
        assert_eq!(bitmap.get(4, 6).expect("is in bounds"), Some(&'^'));
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
