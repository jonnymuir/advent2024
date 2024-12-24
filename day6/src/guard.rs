use crate::Map;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TravelError {
    OutOfBounds,
}

impl fmt::Display for TravelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for TravelError {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Guard {
    position: (usize, usize),
    direction: Direction,
    path: Vec<(usize, usize)>,
}

impl Guard {
    pub fn new(position: (usize, usize), direction: Direction) -> Guard {
        Guard {
            position,
            direction,
            path: vec![position],
        }
    }

    pub fn unique_path_count(&self) -> usize {
        self.path
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    pub fn travel(&self, map: &Map) -> Result<Guard, TravelError> {
        // Test in bounds
        if self.position.0 == 0 && self.direction == Direction::West
            || self.position.1 == 0 && self.direction == Direction::North
            || self.position.0 == map.width - 1 && self.direction == Direction::East
            || self.position.1 == map.height - 1 && self.direction == Direction::South
        {
            return Err(TravelError::OutOfBounds);
        }

        let (new_x, new_y) = match self.direction {
            Direction::North => (self.position.0, self.position.1 - 1),
            Direction::South => (self.position.0, self.position.1 + 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0 - 1, self.position.1),
        };

        // Test for collision - if we collide rotate 90 degrees clockwise and try again
        if map.board[new_y].chars().nth(new_x).unwrap() == '#' {
            return Guard {
                position: self.position,
                direction: match self.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
                path: self.path.clone(),
            }
            .travel(map);
        }

        Ok(Guard {
            position: (new_x as usize, new_y as usize),
            direction: self.direction,
            path: [self.path.clone(), vec![(new_x, new_y)]].concat(),
        })
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Guard at ({}, {}), facing {:?}",
            self.position.0, self.position.1, self.direction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_travel_within_bounds() {
        match Map::from_lines(vec![String::from("."), String::from("^")]) {
            (Some(guard), map) => {
                let new_guard = guard.travel(&map).expect("Travel failed");
                assert_eq!(new_guard.position, (0, 0));
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_travel_out_of_bounds() {
        match Map::from_lines(vec![String::from("^")]) {
            (Some(guard), map) => {
                let result = guard.travel(&map);
                assert!(result.is_err());
                assert_eq!(result.err().unwrap(), TravelError::OutOfBounds);
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_moves() {
        match Map::from_lines(vec![String::from(".."), String::from(".<")]) {
            (Some(guard), map) => {
                let guard_moved = guard.travel(&map).expect("Travel failed");
                assert_eq!(format!("{}", guard_moved), "Guard at (0, 1), facing West");

                assert_eq!(guard_moved.unique_path_count(), 2);
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_moves_count_is_unique() {
        match Map::from_lines(vec![String::from("###"), String::from("#.<")]) {
            (Some(guard), map) => {
                let guard_moved1 = guard.travel(&map).expect("Travel failed");
                let guard_moved2 = guard_moved1.travel(&map).expect("Travel failed");

                assert_eq!(guard_moved2.unique_path_count(), 2);
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_collision() {
        match Map::from_lines(vec![String::from("#."), String::from("^.")]) {
            (Some(guard), map) => {
                let guard = guard.travel(&map);
                assert_eq!(
                    format!("{}", guard.unwrap()),
                    "Guard at (1, 1), facing East"
                );
            }
            _ => panic!("Expected a guard"),
        }
    }
}
