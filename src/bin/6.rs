use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn oxygen_bit_criteria(count: usize, total: usize) -> bool {
    count >= (total as f64 / 2 as f64).round() as usize
}

fn c02_scrub_bit_criteria(count: usize, total: usize) -> bool {
    let half = (total as f64 / 2 as f64).round() as usize;
    count < half
}

fn most_common_bit_for_index(index: usize, input: &[String], criteria: &dyn Fn(usize, usize) -> bool) -> char {
    let mut count = 0;
    for value in input {
        let char = value.chars().nth(index).unwrap();
        // println!("Value {} Char {} from index {}", value, char, index);
        if char == '1' {
            count += 1;
        }
    }
    // println!("Count 1s {} length {}", count, input.len());
    if criteria(count as usize, input.len()) { '1' } else { '0' }
}

fn find_rating_value_pattern(values: &[String], criteria: &dyn Fn(usize, usize) -> bool) -> String {
    let mut results: Vec<String> = values.to_vec();
    let mut pattern = String::new();
    let mut value = String::new();
    for index in 0..12 {
        let most_common_bit = most_common_bit_for_index(index, &results, criteria);
        // println!("Index {} most common bit {}", index, most_common_bit);
        pattern.push(most_common_bit);
        // println!("Pattern {}", pattern);
        results = results
            .into_iter()
            .filter(|x| x.starts_with(&pattern))
            .collect();

        if results.len() == 1 {
            value = results.get(0).unwrap().clone();
            break;
        }
    }
    return value
}

fn main() {
    // 12 bits
    // for each line in file
    //  count 1s per bit
    //
    // for each key in map
//      if count > lines.len / 2
//          gamma = 1
//          epsilon = 0
    let lines = read_lines("src/bin/input_day_3.txt").expect("Could not read file contents");

    let mut values: Vec<String> = Vec::new();

    for line in lines {
        if let Ok(binary) = line {
            values.push(binary);
        }
    }

    let oxygen_pattern = find_rating_value_pattern(&values, &oxygen_bit_criteria); // 010101100111
    let c02_pattern = find_rating_value_pattern(&values, &c02_scrub_bit_criteria); //

    println!("Oxygen pattern {}, C02 pattern {}", oxygen_pattern, c02_pattern);

    let oxygen = isize::from_str_radix(&oxygen_pattern, 2).unwrap();
    let c02 = isize::from_str_radix(&c02_pattern, 2).unwrap();

    println!("Oxygen {} c02 {} multiplied {}", oxygen, c02, oxygen * c02)
}
