use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn main() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    if let Ok(lines) = read_lines("./src/bin/input_day_2.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut split = line.split(" ");
                let command = split.next().expect("Should have command");
                let action: i32 = split.next().expect("Should have action")
                    .parse().expect("Action is not a valid number");

                println!("Command {} with Action {}", command, action);
                match command {
                    "up" => aim -= action,
                    "down" => aim += action,
                    "forward" => {
                        horizontal += action;
                        depth += (aim * action)
                    },
                    _ => continue
                };
            }
        }
    }

    println!("Resulting horizontal {} and aim {}, multiplied {}", horizontal, aim, horizontal*depth);
}
