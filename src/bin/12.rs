use std::fs;
use std::fs::{File, read};
use std::path::Path;

fn parse_input() -> Vec<CrabMarine> {
    let input = fs::read_to_string("src/bin/input_day_7.txt").expect("Expect file");
    return input
        .trim()
        .split(",")
        .into_iter()
        .map(|pos| pos.parse::<u16>().expect("Parsable number"))
        .map(|position| CrabMarine {
            position
        })
        .collect();
}

struct CrabMarine {
    position: u16,
}

impl CrabMarine {
    fn fuel_cost_to_position(&self, to: &u16) -> u16 {
        if &self.position > to {
            println!("From {} to {} = {}", &self.position, to, &self.position - to);
            return &self.position - to
        }

        println!("From {} to {} = {}", &self.position, to, to - &self.position);
        to - &self.position
    }
}

fn main() {
    let input = parse_input();
    let max = input.iter().max_by_key(|c| c.position).expect("Expected max crabmarine").position;
    let min = input.iter().min_by_key(|c| c.position).expect("Expected min crabmarine").position;

    println!("Max {} min {}", max, min);

    let mut lowest_fuel_effort = u32::MAX;
    for pos in min..max {
        let effort = input
            .iter()
            .map(|cm| cm.fuel_cost_to_position(&pos) as u32)
            .sum();

        if effort < lowest_fuel_effort {
            lowest_fuel_effort = effort;
        }
    }

    println!("Lowest fuel effort {}", lowest_fuel_effort);
}
