use std::any::Any;

use thiserror::Error;

use day_06::bitmap::{Bitmap, BitmapError};

const GUARD: &str = "^<>v";
const OBSTACLE: char = '#';
const DOT: char = '.';

trait GamePiece {
    fn get_representation(&self) -> char;

    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, Error)]
enum GuardError {
    #[error("Invalid guard representation")]
    InvalidRepresentation,
}

#[derive(Debug, PartialEq)]
struct Guard {
    guard: char,
}

impl GamePiece for Guard {
    fn get_representation(&self) -> char {
        self.guard
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Guard {
    fn new(guard: char) -> Result<Self, GuardError> {
        if !Guard::is_guard(guard) {
            return Err(GuardError::InvalidRepresentation);
        }
        Ok(Self { guard })
    }

    fn is_guard(representation: char) -> bool {
        GUARD.contains(representation)
    }

    fn turn_right(&mut self) {
        self.guard = match self.guard {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Invalid guard char"),
        };
    }

    fn get_delta(&self) -> (i32, i32) {
        match self.guard {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!("Invalid guard char"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Obstacle {}

impl Obstacle {
    fn new() -> Self {
        Self {}
    }

    fn is_obstacle(representation: char) -> bool {
        representation == OBSTACLE
    }
}

impl GamePiece for Obstacle {
    fn get_representation(&self) -> char {
        OBSTACLE
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, PartialEq)]
struct Dot {}

impl Dot {
    fn new() -> Self {
        Self {}
    }

    fn is_dot(representation: char) -> bool {
        representation == DOT
    }
}

impl GamePiece for Dot {
    fn get_representation(&self) -> char {
        DOT
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct GameBoard {
    board: Bitmap<Box<dyn GamePiece>>,
}

impl GameBoard {
    fn new(width: usize, height: usize) -> Self {
        let board = Bitmap::new(width, height);
        Self { board }
    }

    fn import(board: &[String]) -> Self {
        let width = board[0].len();
        let height = board.len();
        let mut game_board = GameBoard::new(width, height);
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.chars().enumerate() {
                let piece = match cell {
                    '^' | '>' | 'v' | '<' => {
                        Box::new(Guard::new(cell).expect("Invalid guard mapping"))
                            as Box<dyn GamePiece>
                    }
                    '#' => Box::new(Obstacle::new()) as Box<dyn GamePiece>,
                    '.' => Box::new(Dot::new()) as Box<dyn GamePiece>,
                    _ => panic!("Invalid game piece"),
                };
                game_board
                    .board
                    .set(x, y, piece)
                    .expect("Setting game piece failed");
            }
        }
        game_board
    }

    fn set(&mut self, x: usize, y: usize, piece: Box<dyn GamePiece>) -> Result<(), BitmapError> {
        self.board.set(x, y, piece)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Box<dyn GamePiece>> {
        self.board.get(x, y)
    }

    fn print(&self) {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let piece = self.get(x, y).unwrap();
                print!("{}", piece.get_representation());
            }
            println!();
        }
        println!()
    }

    fn find_guard(&self) -> Option<(usize, usize)> {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let piece = self.get(x, y).unwrap();
                if Guard::is_guard(piece.get_representation()) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn step_guard(&self) -> Option<(usize, usize)> {
        if let Some(guard_position) = self.find_guard() {
            return self.step_guard_from_position(guard_position);
        }
        None
    }

    fn step_guard_from_position(&self, guard_position: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = guard_position;
        let guard = self.get(x, y).unwrap();
        // makes me sweat
        let guard = guard.as_any().downcast_ref::<Guard>().unwrap();
        let guard_delta = guard.get_delta();
        let new_position = self.add_delta_signed(x, y, guard_delta);
        if let Some((new_x, new_y)) = new_position {
            if !self.is_obstacle(new_x, new_y) {
                return Some((new_x, new_y));
            }
        }
        None
    }

    fn step_if_possible(
        &mut self,
        existing_position: Option<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let old_position = existing_position.unwrap_or(self.find_guard().unwrap());
        if let Some(new_position) = self.step_guard_from_position(old_position) {
            let guard = self.get(old_position.0, old_position.1).unwrap();
            let guard_rep = guard.get_representation();
            if self
                .set(old_position.0, old_position.1, Box::new(Dot::new()))
                .is_err()
            {
                return None;
            }
            if self
                .set(
                    new_position.0,
                    new_position.1,
                    Box::new(Guard::new(guard_rep).expect("Invalid guard rep"))
                        as Box<dyn GamePiece>,
                )
                .is_err()
            {
                return None;
            }
            return Some(new_position);
        }
        None
    }

    fn step_until_stopped(&mut self) -> usize {
        let mut guard_position = None;
        let mut steps = 0;
        loop {
            guard_position = self.step_if_possible(guard_position);
            if guard_position.is_none() {
                break;
            }
            steps += 1;
        }
        steps
    }

    fn count_all_paths_until_stuck(&mut self) -> usize {
        let mut count = 0;
        loop {
            self.print();
            count += self.step_until_stopped();
            let guard_position = self.find_guard().unwrap();
            let guard = self.get(guard_position.0, guard_position.1).unwrap();
            let mut guard = Guard::new(guard.get_representation()).expect("Require valid guard");
            guard.turn_right();
            self.set(
                guard_position.0,
                guard_position.1,
                Box::new(guard) as Box<dyn GamePiece>,
            )
            .expect("Must update guard");
            let guard_position = self.step_guard();
            if guard_position.is_none() {
                break;
            }
        }
        self.print();
        count
    }

    fn add_delta_signed(&self, x: usize, y: usize, delta: (i32, i32)) -> Option<(usize, usize)> {
        let new_x = x as i32 + delta.0;
        let new_y = y as i32 + delta.1;
        if new_x < 0 || new_y < 0 {
            return None;
        }
        if new_x >= self.board.width() as i32 || new_y >= self.board.height() as i32 {
            return None;
        }
        Some((new_x as usize, new_y as usize))
    }

    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        let piece = self.get(x, y).unwrap();
        Obstacle::is_obstacle(piece.get_representation())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_new() {
        let guard = Guard::new('^');
        assert!(guard.is_ok());
    }

    #[test]
    fn test_guard_new_invalid() {
        let guard = Guard::new('a');
        assert!(guard.is_err());
    }

    #[test]
    fn test_guard_turn_right() {
        let mut guard = Guard::new('^').unwrap();
        guard.turn_right();
        assert_eq!(guard.get_representation(), '>');
    }

    #[test]
    fn test_guard_turn_right_twice() {
        let mut guard = Guard::new('^').unwrap();
        guard.turn_right();
        guard.turn_right();
        assert_eq!(guard.get_representation(), 'v');
    }

    #[test]
    fn test_guard_turn_right_thrice() {
        let mut guard = Guard::new('^').unwrap();
        guard.turn_right();
        guard.turn_right();
        guard.turn_right();
        assert_eq!(guard.get_representation(), '<');
    }

    #[test]
    fn tesst_three_lefts_make_a_right() {
        let mut guard = Guard::new('^').unwrap();
        guard.turn_right();
        guard.turn_right();
        guard.turn_right();
        guard.turn_right();
        assert_eq!(guard.get_representation(), '^');
    }

    #[test]
    fn test_is_guard() {
        assert!(Guard::is_guard('^'));
        assert!(Guard::is_guard('>'));
        assert!(Guard::is_guard('v'));
        assert!(Guard::is_guard('<'));
    }

    #[test]
    fn test_is_obstacle() {
        assert!(Obstacle::is_obstacle(OBSTACLE));
    }

    #[test]
    fn test_new_obstacle() {
        let obstacle = Obstacle::new();
        assert!(Obstacle::is_obstacle(obstacle.get_representation()));
    }

    #[test]
    fn test_dot_new() {
        assert!(Dot::is_dot(DOT));
    }

    #[test]
    fn test_new_dot() {
        let dot = Dot::new();
        assert!(Dot::is_dot(dot.get_representation()));
    }

    #[test]
    fn test_game_board_new() {
        let game_board = GameBoard::new(10, 10);
        assert!(game_board.board.get(0, 0).is_none());
        assert!(game_board.board.get(9, 9).is_none());
    }

    #[test]
    fn test_game_board_import() {
        let lines = sample_data();
        let game_board = GameBoard::import(&lines);
        game_board.print();
        assert_eq!(game_board.get(4, 6).unwrap().get_representation(), '^');
        assert_eq!(game_board.get(0, 0).unwrap().get_representation(), '.',);
        assert_eq!(game_board.get(0, 1).unwrap().get_representation(), '.',);
        assert_eq!(game_board.get(4, 0).unwrap().get_representation(), '#',);
        assert_eq!(game_board.get(0, 8).unwrap().get_representation(), '#',);
    }

    #[test]
    fn test_game_board_find_guard() {
        let lines = sample_data();
        let game_board = GameBoard::import(&lines);
        let guard = game_board.find_guard().unwrap();
        assert_eq!(guard, (4, 6));
    }

    #[test]
    fn test_step_guard() {
        let lines = sample_data();
        let game_board = GameBoard::import(&lines);
        game_board.print();
        let guard = game_board.step_guard().unwrap();
        assert_eq!(guard, (4, 5));
        game_board.print();
    }

    #[test]
    fn test_step_if_possible() {
        let lines = sample_data();
        let mut game_board = GameBoard::import(&lines);
        game_board.print();
        let guard = game_board.step_if_possible(None).unwrap();
        assert_eq!(guard, (4, 5));
        let guard_piece = game_board.get(4, 5).unwrap();
        assert_eq!(guard_piece.get_representation(), '^');
    }

    #[test]
    fn test_step_guard_twice() {
        let lines = sample_data();
        let game_board = GameBoard::import(&lines);
        game_board.print();
        let guard = game_board.step_guard().unwrap();
        assert_eq!(guard, (4, 5));
        game_board.print();
        let guard = game_board.step_guard().unwrap();
        assert_eq!(guard, (4, 5));
        game_board.print();
    }

    #[test]
    fn test_step_until_stuck() {
        let lines = sample_data();
        let mut game_board = GameBoard::import(&lines);
        game_board.print();
        game_board.step_until_stopped();
        game_board.print();
        let guard_position = game_board.find_guard().unwrap();
        assert_eq!(guard_position, (4, 1));
        for y in 2..9 {
            assert_eq!(game_board.get(4, y).unwrap().get_representation(), '.');
        }
        assert!(game_board.is_obstacle(4, 0));
    }

    #[test]
    fn test_count_all_paths_until_stuck() {
        let lines = sample_data();
        let mut game_board = GameBoard::import(&lines);
        let count = game_board.count_all_paths_until_stuck();
        assert_eq!(count, 41);
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
