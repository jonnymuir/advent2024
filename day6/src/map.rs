use crate::guard::{Direction, Guard};
use std::fmt;

pub struct Map {
    // Define the fields for your map here
    // For example:
    pub board: Vec<String>,
    pub width: usize,
    pub height: usize
}

impl Map {
    pub fn from_lines(board: Vec<String>) -> (Option<Guard>, Map) {
        let width = board[0].len();
        let height = board.len();
        for (y, line) in board.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {

                let direction = match c {
                    '^' => Some(Direction::North),
                    '>' => Some(Direction::East),
                    'v' => Some(Direction::South),
                    '<' => Some(Direction::West),
                    _ => None,
                };

                if let Some(direction) = direction {
                    return (
                        Some(Guard::new((x, y), direction)),
                        Map { board, width, height},
                    );
                }
            }
        }
        (None, Map { board, width, height})
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.board {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_guard_display() {
        let board = vec![String::from("^."), String::from("..")];

        let (guard, _) = Map::from_lines(board);

        assert_eq!(
            format!("{}", guard.unwrap()),
            "Guard at (0, 0), facing North"
        );
    }

    #[test]
    fn test_guard_at_1_0() {
        let board = vec![String::from(".^"), String::from("..")];

        let (guard, _) = Map::from_lines(board);

        assert_eq!(
            format!("{}", guard.unwrap()),
            "Guard at (1, 0), facing North"
        );
    }

    #[test]
    fn test_guard_at_1_0_east() {
        let board = vec![String::from(".>"), String::from("..")];

        let (guard, _) = Map::from_lines(board);

        assert_eq!(
            format!("{}", guard.unwrap()),
            "Guard at (1, 0), facing East"
        );
    }

    #[test]
    fn test_guard_at_1_0_south() {
        let board = vec![String::from(".v"), String::from("..")];

        let (guard, _) = Map::from_lines(board);

        assert_eq!(
            format!("{}", guard.unwrap()),
            "Guard at (1, 0), facing South"
        );
    }

    #[test]
    fn test_guard_at_1_0_west() {
        let board = vec![String::from(".<"), String::from("..")];

        let (guard, _) = Map::from_lines(board);

        assert_eq!(
            format!("{}", guard.unwrap()),
            "Guard at (1, 0), facing West"
        );
    }

}
