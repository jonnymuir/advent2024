use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod board;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let board:board::Board = board::Board::new(reader
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect());
        
    println!("Word count: {}", board.solve("XMAS"));
 
    Ok(())
}