use crate::guard::{Guard, Direction};
use std::fmt;

pub struct Map {
    // Define the fields for your map here
    // For example:
    board: Vec<String>,
}

impl Map {
    pub fn from_lines(board: Vec<String>) -> (Guard, Map) {
        let guard = Guard::new((0, 0), Direction::North);

        (guard, Map { board })
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