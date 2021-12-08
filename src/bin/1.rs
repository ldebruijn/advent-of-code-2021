use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn main() {
    // read input file into array
    // for each item, index in array
    // if has previous measurement
    // if previous measurement < current measurement
    //      increase counter
    // output counter
    if let Ok(lines) = read_lines("./src/bin/day_1/input.txt") {
        let mut previous = -1;
        let mut counter = 0;

        for line in lines {
            if let Ok(current) = line {
                if previous < 0 {
                    previous = current.parse().expect("Not a valid number");
                    continue;
                }

                let current = current.parse().expect("Not a valid number");
                if previous < current {
                    counter += 1;
                }

                previous = current;
            }
        }
        println!("Total increases is {}", counter)
    }
}
