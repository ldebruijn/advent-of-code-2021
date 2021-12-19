use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn parse_input() {
    let valid_numbers = [2,3,4,7];
    let mut num_valid_numbers: i32 = 0;
    if let Ok(lines) = read_lines("src/bin/input_day_8.txt") {
        for line in lines {
            if let Ok(val) = line {
                let mut split = val.split("|");
                split.next().expect("Value expected");
                let output_values = split.next().expect("Output values expected");

                let counts: i32 =  output_values.split(" ")
                    .filter(|segment| {
                        let size = segment.chars().count();
                        valid_numbers.contains(&size)
                    })
                    .map(|x| {
                        let size = x.chars().count() as i32;
                        println!("{} - {}", x, size);
                        size
                    })
                    .count() as i32;

                num_valid_numbers += counts;
            }
        }
    }

    println!("Valid numbesr {}", num_valid_numbers);
}

fn main() {
    let input = parse_input();
}
