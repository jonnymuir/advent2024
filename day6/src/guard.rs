use std::fmt;

pub struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    pub fn new(position: (i32, i32), direction: Direction) -> Guard {
        Guard { position, direction }
    }
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guard at ({}, {}), facing {:?}", self.position.0, self.position.1, self.direction)
    }
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}