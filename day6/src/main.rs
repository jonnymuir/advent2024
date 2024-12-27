mod guard;
mod map;

use guard::{Guard, TravelResult};
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
            println!("Visited {} unique locations", travel_until_done(&guard, &map));

            // And then use the guard to find out how many of them would 
            // head to an infinite path it they where rotated by 90%
            let infinite_paths = std::iter::successors(Some(guard), |g| match g.travel(&map) {
                TravelResult::GuardMoved(new_guard) => Some(new_guard),
                _ => None,
            }).enumerate()
            .inspect(|(i, _)| println!("Checking guard {}", i))
            .map(|(_, g)| is_on_infinite_path(&g.rotate90(), &map))
            .filter(|&b| b)
            .count();
            
            println!("{} of them would lead to an infinite path if rotated by 90%", infinite_paths);

            Ok(())
        }
        _ => panic!("Expected a guard"),
    }
}

fn travel_until_done(guard: &Guard, map: &Map) -> usize {
    match guard.travel(map) {
        TravelResult::GuardMoved(new_guard) => travel_until_done(&new_guard, map),
        _ => guard.unique_path_count(),
    }
}

fn is_on_infinite_path(guard: &Guard, map: &Map) -> bool {
    match guard.travel(map) {
        TravelResult::GuardMoved(new_guard) => is_on_infinite_path(&new_guard, map),
        TravelResult::InfinitePath => true,
        TravelResult::OutOfBounds => false
    }
}