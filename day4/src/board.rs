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

    pub fn solve(&self, word: &str) -> usize {
        // We create an iterator of all direction transforms (e.g. 0,1 = right, -1,-1 = diagonal up and left etc )
        let directions = iproduct!(-1..2, -1..2);

        // Then for each coordinate for each direction - look for the word. 
        // This could be optimised as it means checking all directions even though the letter doesn't match
        iproduct!(self.coordinates(), directions)
            .filter(|(coord, direction)| self.is_word_at_coord(word, *coord, *direction))
            .count()
    }

    pub fn solve_xmas(&self) -> usize {
        // To solve X-MAS, we iterate round the board, looking for an A that has diagnal pairs of M and S's surrounding them
        self.coordinates()
            .filter(|(x, y)| {
                if *x > 0
                    && *x < self.width - 1
                    && *y > 0
                    && *y < self.height - 1
                    && self.board[*y][*x] == 'A'
                {
                    let pairs = [
                        ((x - 1, y - 1), (x + 1, y + 1)),
                        ((x + 1, y + 1), (x - 1, y - 1)),
                        ((x + 1, y - 1), (x - 1, y + 1)),
                        ((x - 1, y + 1), (x + 1, y - 1)),
                    ];

                    let matches = pairs.iter().filter(|&(p1, p2)| {
                        self.board[p1.1][p1.0] == 'M' && self.board[p2.1][p2.0] == 'S'
                    }).count();

                    return matches >= 2;
                }
                false
            })
            .count()
    }

    fn is_word_at_coord(&self, word: &str, coord: (usize, usize), direction: (i32, i32)) -> bool {
        println!(
            "Testing {:?} for {} in direction {:?}",
            coord,
            word.chars().next().unwrap(),
            direction
        );
        if self.board[coord.1][coord.0] == word.chars().next().unwrap() {
            println!("Found");

            // Are we the end of the word? If so return a match
            if word.len() == 1 {
                println!("Found word finishing at {:?}", coord);
                return true;
            }

            // Otherwise transform the coord by the direction
            let (new_x, new_y) = (
                (coord.0 as i32 + direction.0),
                (coord.1 as i32 + direction.1),
            );

            if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
                // The new coordinate is within bounds, so use it
                return self.is_word_at_coord(
                    &word[1..],
                    (new_x as usize, new_y as usize),
                    direction,
                );
            }
        }
        println!("Not found");
        false
    }

    fn coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        iproduct!(0..self.width, 0..self.height)
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
