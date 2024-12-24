use crate::Map;
use std::fmt;

pub enum TravelResult {
    OutOfBounds,
    InfinitePath,
    GuardMoved(Guard),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn rotate90(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn apply(self, coord: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::North => (coord.0, coord.1 - 1),
            Direction::South => (coord.0, coord.1 + 1),
            Direction::East => (coord.0 + 1, coord.1),
            Direction::West => (coord.0 - 1, coord.1),
        }
    }
}

pub struct Guard {
    position: (usize, usize),
    direction: Direction,
    path: Vec<((usize, usize), Direction)>,
}

impl Guard {
    pub fn new(position: (usize, usize), direction: Direction) -> Guard {
        Guard {
            position,
            direction,
            path: vec![(position, direction)],
        }
    }

    pub fn unique_path_count(&self) -> usize {
        self.path
            .iter()
            .map(|(pos, _)| pos)
            .collect::<std::collections::HashSet<_>>()
            .len()
    }

    pub fn rotate90(&self) -> Guard {
        Guard {
            position: self.position,
            direction: self.direction.rotate90(),
            path: [self.path.clone(), vec![(self.position, self.direction.rotate90())]].concat()
        }
    }

    pub fn travel(&self, map: &Map) -> TravelResult {
        // Test in bounds
        if self.position.0 == 0 && self.direction == Direction::West
            || self.position.1 == 0 && self.direction == Direction::North
            || self.position.0 == map.width - 1 && self.direction == Direction::East
            || self.position.1 == map.height - 1 && self.direction == Direction::South
        {
            return TravelResult::OutOfBounds;
        }

        let new_position = self.direction.apply(self.position);

        // Test for collision - if we collide rotate 90 degrees clockwise otherwise move
        let new_guard = match map.board[new_position.1].chars().nth(new_position.0) {
            Some('#') => Guard {
                position: self.position,
                direction: self.direction.rotate90(),
                path: [
                    self.path.clone(),
                    vec![(self.position, self.direction.rotate90())],
                ]
                .concat(),
            },
            _ => Guard {
                position: new_position,
                direction: self.direction,
                path: [self.path.clone(), vec![(new_position, self.direction)]].concat(),
            },
        };

        // Test for infinite path
        if self
            .path
            .iter()
            .any(|(pos, direction)| *pos == new_guard.position && *direction == new_guard.direction)
        {
            return TravelResult::InfinitePath;
        };

        TravelResult::GuardMoved(new_guard)
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
                let new_guard = match guard.travel(&map) {
                    TravelResult::GuardMoved(new_guard) => new_guard,
                    _ => panic!("Expected TravelResult::GuardMoved"),
                };

                assert_eq!(new_guard.position, (0, 0));
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_travel_out_of_bounds() {
        match Map::from_lines(vec![String::from("^")]) {
            (Some(guard), map) => {
                assert!(
                    matches!(guard.travel(&map), TravelResult::OutOfBounds),
                    "Expected TravelResult::OutOfBounds"
                );
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_moves() {
        match Map::from_lines(vec![String::from(".."), String::from(".<")]) {
            (Some(guard), map) => match guard.travel(&map) {
                TravelResult::GuardMoved(guard_moved) => {
                    assert_eq!(guard_moved.position, (0, 1));
                    assert_eq!(guard_moved.direction, Direction::West);
                }
                _ => panic!("Expected TravelResult::GuardMoved"),
            },
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_rotate90() {
        match Map::from_lines(vec![String::from("<")]) {
            (Some(guard), _) => {
                let new_guard = guard.rotate90();
                assert_eq!(new_guard.position, (0, 0));
                assert_eq!(new_guard.direction, Direction::North);
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_moves_count_is_unique() {
        match Map::from_lines(vec![String::from("###"), String::from("#.<")]) {
            (Some(guard), map) => {
                let guard_moved1 = match guard.travel(&map) {
                    TravelResult::GuardMoved(new_guard) => new_guard,
                    _ => panic!("Expected TravelResult::GuardMoved"),
                };

                let guard_moved2 = match guard_moved1.travel(&map) {
                    TravelResult::GuardMoved(new_guard) => new_guard,
                    _ => panic!("Expected TravelResult::GuardMoved"),
                };

                assert_eq!(guard_moved2.unique_path_count(), 2);
            }
            _ => panic!("Expected a guard"),
        }
    }

    #[test]
    fn test_guard_collision() {
        match Map::from_lines(vec![String::from("#."), String::from("^.")]) {
            (Some(guard), map) => {
                let new_guard = match guard.travel(&map) {
                    TravelResult::GuardMoved(new_guard) => new_guard,
                    _ => panic!("Expected TravelResult::GuardMoved"),
                };

                assert_eq!(new_guard.position, (0, 1));
                assert_eq!(new_guard.direction, Direction::East);
            }
            _ => panic!("Expected a guard"),
        }
    }
    #[test]
    fn test_infinite_path() {
        match Map::from_lines(vec![
            String::from("###"),
            String::from("#<#"),
            String::from("###"),
        ]) {
            (Some(guard), map) => {
                let result = std::iter::successors(Some(guard), |g| match g.travel(&map) {
                    TravelResult::GuardMoved(new_guard) => Some(new_guard),
                    _ => None,
                })
                .last()
                .map(|g| g.travel(&map))
                .unwrap();

                assert!(
                    matches!(result, TravelResult::InfinitePath),
                    "Expected TravelResult::InfinitePath"
                );
            }
            _ => panic!("Expected a guard"),
        }
    }
}
