mod guard;
mod map;

use guard::{Guard, TravelError};
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
    match  Map::from_lines(reader.lines().collect::<Result<Vec<_>, _>>()?) {
        (Some(guard), map) => {
            let final_guard = travel_until_out_of_bounds(guard, &map);
            println!("Visited {} unique locations", final_guard.unique_path_count());
            Ok(())
        }
        _ => panic!("Expected a guard"),
    }
}

fn travel_until_out_of_bounds(guard: Guard, map: &Map) -> Guard {
    match guard.travel(map) {
        Ok(new_guard) => travel_until_out_of_bounds(new_guard, map),
        Err(TravelError::OutOfBounds) => guard
    }
}
