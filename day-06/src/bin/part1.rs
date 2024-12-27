use std::collections::HashSet;
use std::io;

use thiserror::Error;

use day_06::bitmap::{Bitmap, BitmapError};

const GUARD: &str = "^<>v";
const OBSTACLE: char = '#';
const DOT: char = '.';

#[derive(Debug, Clone, PartialEq)]
enum GameBoardCell {
    Guard(&'static Guard),
    Obstacle(&'static Obstacle),
    Dot(&'static Dot),
}

impl GameBoardCell {
    fn get_representation(&self) -> char {
        match self {
            GameBoardCell::Guard(guard) => guard.get_representation(),
            GameBoardCell::Obstacle(obstacle) => obstacle.get_representation(),
            GameBoardCell::Dot(dot) => dot.get_representation(),
        }
    }

    fn as_guard(&self) -> Option<&Guard> {
        match self {
            GameBoardCell::Guard(guard) => Some(guard),
            _ => None,
        }
    }
}

impl Default for GameBoardCell {
    fn default() -> Self {
        GameBoardCell::Dot(&DOT_FLY_WEIGHT)
    }
}

trait GamePiece {
    fn get_representation(&self) -> char;
}

#[derive(Debug, Error)]
enum GuardError {
    #[error("Invalid guard representation")]
    InvalidRepresentation,
}

const GUARD_FLY_WEIGHT_UP: Guard = Guard { guard: '^' };
const GUARD_FLY_WEIGHT_RIGHT: Guard = Guard { guard: '>' };
const GUARD_FLY_WEIGHT_DOWN: Guard = Guard { guard: 'v' };
const GUARD_FLY_WEIGHT_LEFT: Guard = Guard { guard: '<' };

#[derive(Debug, PartialEq, Clone)]
struct Guard {
    guard: char,
}

impl GamePiece for Guard {
    fn get_representation(&self) -> char {
        self.guard
    }
}

impl Guard {
    fn new(guard: char) -> Result<&'static Self, GuardError> {
        match guard {
            '^' => Ok(&GUARD_FLY_WEIGHT_UP),
            '>' => Ok(&GUARD_FLY_WEIGHT_RIGHT),
            'v' => Ok(&GUARD_FLY_WEIGHT_DOWN),
            '<' => Ok(&GUARD_FLY_WEIGHT_LEFT),
            _ => Err(GuardError::InvalidRepresentation),
        }
    }

    fn is_guard(representation: char) -> bool {
        GUARD.contains(representation)
    }

    fn turn_right(&self) -> Option<&'static Self> {
        match self.guard {
            '^' => Some(&GUARD_FLY_WEIGHT_RIGHT),
            '>' => Some(&GUARD_FLY_WEIGHT_DOWN),
            'v' => Some(&GUARD_FLY_WEIGHT_LEFT),
            '<' => Some(&GUARD_FLY_WEIGHT_UP),
            _ => None,
        }
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

const OBSTACLE_FLY_WEIGHT: Obstacle = Obstacle {};

#[derive(Debug, PartialEq)]
struct Obstacle {}

impl Obstacle {
    fn new() -> &'static Self {
        &OBSTACLE_FLY_WEIGHT
    }

    fn is_obstacle(representation: char) -> bool {
        representation == OBSTACLE
    }
}

impl GamePiece for Obstacle {
    fn get_representation(&self) -> char {
        OBSTACLE
    }
}

const DOT_FLY_WEIGHT: Dot = Dot {};

#[derive(Debug, PartialEq)]
struct Dot {}

impl Dot {
    fn new() -> &'static Self {
        &DOT_FLY_WEIGHT
    }

    #[allow(dead_code)]
    fn is_dot(representation: char) -> bool {
        representation == DOT
    }
}

impl GamePiece for Dot {
    fn get_representation(&self) -> char {
        DOT
    }
}

struct GameBoard {
    board: Bitmap<GameBoardCell>,
    visited: HashSet<(usize, usize)>,
}

impl GameBoard {
    fn new(width: usize, height: usize) -> Self {
        let board = Bitmap::new(width, height);
        Self {
            board,
            visited: HashSet::new(),
        }
    }

    fn import(board: &[String]) -> Self {
        let width = board[0].len();
        let height = board.len();
        let mut game_board = GameBoard::new(width, height);
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.chars().enumerate() {
                let piece = match cell {
                    '^' | '>' | 'v' | '<' => {
                        GameBoardCell::Guard(Guard::new(cell).expect("Invalid guard mapping"))
                    }
                    '#' => GameBoardCell::Obstacle(Obstacle::new()),
                    '.' => GameBoardCell::Dot(Dot::new()),
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

    fn set(&mut self, x: usize, y: usize, piece: GameBoardCell) -> Result<(), BitmapError> {
        self.board.set(x, y, piece)
    }

    fn get(&self, x: usize, y: usize) -> Option<&GameBoardCell> {
        if let Some(piece) = self.board.get(x, y).unwrap_or(None) {
            return Some(piece);
        }
        None
    }

    #[allow(dead_code)]
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
        if let Some(guard) = guard.as_guard() {
            let guard_delta = guard.get_delta();
            let new_position = self.add_delta_signed(x, y, guard_delta);
            if let Some((new_x, new_y)) = new_position {
                if !self.is_obstacle(new_x, new_y) {
                    return Some((new_x, new_y));
                }
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
                .set(
                    old_position.0,
                    old_position.1,
                    GameBoardCell::Dot(Dot::new()),
                )
                .is_err()
            {
                return None;
            }
            if self
                .set(
                    new_position.0,
                    new_position.1,
                    GameBoardCell::Guard(Guard::new(guard_rep).expect("Invalid guard rep")),
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
            self.visited.insert(guard_position.unwrap());
            steps += 1;
        }
        steps
    }

    fn count_all_paths_until_stuck(&mut self) -> usize {
        let mut count = 0;
        loop {
            count += self.step_until_stopped();
            let guard_position = self.find_guard().unwrap();
            let guard = self.get(guard_position.0, guard_position.1).unwrap();
            let guard = Guard::new(guard.get_representation()).expect("Require valid guard");
            let guard = guard.turn_right();
            if let Some(guard) = guard {
                self.set(
                    guard_position.0,
                    guard_position.1,
                    GameBoardCell::Guard(guard),
                )
                .expect("Must update guard");
            } else {
                break;
            }
            let guard_position = self.step_guard();
            if guard_position.is_none() {
                break;
            }
        }
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

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut game_board = GameBoard::import(&lines);
    let count = game_board.count_all_paths_until_stuck();
    println!(
        "Steps: {}, having visited: {}",
        count,
        game_board.visited_count()
    );
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
        let guard = Guard::new('^').unwrap();
        let guard = guard.turn_right().expect("must rotate");
        assert_eq!(guard.get_representation(), '>');
    }

    #[test]
    fn test_guard_turn_right_twice() {
        let guard = Guard::new('^').unwrap();
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
        assert_eq!(guard.get_representation(), 'v');
    }

    #[test]
    fn test_three_rights_make_a_left() {
        let guard = Guard::new('^').unwrap();
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
        assert_eq!(guard.get_representation(), '<');
    }

    #[test]
    fn test_all_around_the_world() {
        let guard = Guard::new('^').unwrap();
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
        let guard = guard.turn_right().expect("must rotate");
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
        assert_eq!(game_board.get(0, 0), Some(&GameBoardCell::default()));
        assert_eq!(game_board.get(9, 9), Some(&GameBoardCell::default()));
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
        assert_eq!(count, 44);
        assert_eq!(game_board.visited_count(), 41);
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
