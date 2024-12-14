use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_valid_row_with_fail(row: &[i32]) -> bool {
    if is_valid_row(row) {
        return true;
    }
    else {
        for i in 0..row.len() {
            if is_valid_row(&[&row[0..i],&row[i+1..row.len()]].concat()) {
                return true;
            }
        }
    }
    false
}

fn is_valid_row(row: &[i32]) -> bool {
    if row.len() <= 1 {
        return false;
    }

    let direction = row[1] - row[0];
    for i in 0..row.len() - 1 {
        let diff = row[i + 1] - row[i];

        if diff * direction <= 0 || diff.abs() > 3{
            return false;
        }
    }
    true
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let valid_count = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|row| is_valid_row_with_fail(row))
        .count();

    println!("Number of valid rows: {}", valid_count);

    Ok(())
}
