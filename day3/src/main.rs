use std::env;
use std::fs::read_to_string;

// Executes a commands string (the input) recursively
fn execute(commands: &str) -> i32 {
 
    if commands.starts_with("mul(") {
        return  process_multiplication(&commands[4..]);
    } else if commands.starts_with("don't()") {
        return process_dont(&commands[7..]);
    } else if commands.len() > 0 {
        return execute(&commands[1..])
    }

    0
}

// The dont command scans for the a do() and ignores everything in between
fn process_dont(the_rest: &str) -> i32 {
    if let Some(index) = the_rest.find("do()") {
        return execute(&the_rest[index + 4..]);
    }

    0
}

// The multiplication command looks for "x,y)" and multiplies them
fn process_multiplication(the_rest: &str) -> i32 {
    // Regex pattern to match numbers separated by a comma and enclosed in parentheses
    let re = regex::Regex::new(r"^(\d+),(\d+)\)").unwrap();

    if let Some(captures) = re.captures(&the_rest) {
        let x_str = captures.get(1).unwrap().as_str();
        let y_str = captures.get(2).unwrap().as_str();

        let x: i32 = x_str.parse().unwrap();
        let y: i32 = y_str.parse().unwrap();

        return x * y + execute(&the_rest[captures.get(0).unwrap().end()..]);
    }

    // If the format doesn't match, recurse
    execute(the_rest)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    println!("sum: {}", execute(&read_to_string(&args[1])?));

    Ok(())
}