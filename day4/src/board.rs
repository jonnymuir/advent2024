use itertools::iproduct;
use std::fmt;

pub struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        let height = board.len();
        let width = if height > 0 { board[0].len() } else { 0 };
        Self {
            board,
            width,
            height,
        }
    }

    pub fn solve(&self, word: &str) -> u32 {
        self.coordinates()
            .map(|coord| self.wordcount(word, coord))
            .sum()
    }

    fn wordcount(&self, word: &str, coord: (usize, usize)) -> u32 {
        if self.board[coord.1][coord.0] == word.chars().next().unwrap() {
            if word.len() == 1 {
                return 1;
            } else {
                return self
                    .adjacent_coords(coord)
                    .iter()
                    .map(|coord| self.wordcount(&word[1..], *coord))
                    .sum::<u32>();
            }
        }

        0
    }

    fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(0..self.width, 0..self.height)
    }

    fn adjacent_coords(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let directions = [
            (0, 1),   // Right
            (0, -1),  // Left
            (1, 0),   // Down
            (-1, 0),  // Up
            (1, 1),   // Diagonal Down-Right
            (-1, -1), // Diagonal Up-Left
            (1, -1),  // Diagonal Down-Left
            (-1, 1),  // Diagonal Up-Right
        ];

        directions
            .iter()
            .map(|(dx, dy)| (coord.0 as i32 + dx, coord.1 as i32 + dy))
            .filter_map(|(x, y)| {
                if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
