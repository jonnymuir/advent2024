mod guard;
mod map;

use map::Map;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    // Read all lines from the file and create a map from them
    let (guard, map) = Map::from_lines(reader.lines().collect::<Result<Vec<_>, _>>()?);
 
    println!("{}", guard);
    println!("{}", map);

    Ok(())
}
