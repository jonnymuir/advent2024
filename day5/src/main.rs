mod rules;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rules::Rules;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    // Read all lines into a vector
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

    // Find the index of the blank line
    let blank_line_index = lines.iter().position(|line| line.trim().is_empty()).expect("No blank line found");

    // Split the vector into two slices
    let rules = Rules::from_lines(&lines[..blank_line_index]);
    let pages_lines = &lines[blank_line_index + 1..];

    // Parse the pages into a vector of vectors
    let parsed_pages: Vec<Vec<u32>> = pages_lines.iter()
        .map(|line| {
            line.split(',')
                .map(|num| num.trim().parse::<u32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    // Debug print to verify the split and parsing
    println!("Parsed Rules: {}", rules);
    println!("Parsed Pages: {:?}", parsed_pages);

    Ok(())
}
