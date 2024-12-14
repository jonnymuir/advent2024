use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let (left_numbers, right_numbers): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line: Result<String, io::Error>| {
            let line = line.unwrap();
            let (left_str, right_str) = line
                .split_once("   ")
                .expect("Expected two numbers per line");
            (
                left_str.parse::<i32>().unwrap(),
                right_str.parse::<i32>().unwrap(),
            )
        })
        .unzip();

    // Part one
    println!(
        "Total sum of differences: {}",
        left_numbers
            .iter()
            .sorted()
            .zip(right_numbers.iter().sorted())
            .map(|(left, right)| left.abs_diff(*right))
            .sum::<u32>()
    );

    // Part two

    let right_counts: HashMap<i32, i32> =
        right_numbers.iter().fold(HashMap::new(), |mut acc, num| {
            *acc.entry(*num).or_insert(0) += 1;
            acc
        });

    let part_two_sum: i32 = left_numbers
        .iter()
        .map(|left| *left * right_counts.get(left).unwrap_or(&0))
        .sum();

    println!("Part two sum: {}", part_two_sum);

    Ok(())
}
