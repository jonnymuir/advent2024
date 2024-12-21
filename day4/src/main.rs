use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod board;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage for word solution: {} <filename> <word>", args[0]);
        eprintln!("e.g: {} input.txt XMAS", args[0]);
        eprintln!("Usage for X-MAS solution: {} <filename>", args[0]);
        eprintln!("e.g: {} input.txt", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let board: board::Board = board::Board::new(
        reader
            .lines()
            .map(|line| line.unwrap().chars().collect::<Vec<char>>())
            .collect(),
    );

    if args.len() == 3 {
        println!("Word count: {}", board.solve(&args[2]));
    } else {
        println!("X-mas count: {}", board.solve_xmas());
    }
    Ok(())
}
