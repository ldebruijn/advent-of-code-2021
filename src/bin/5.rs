use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
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

    let mut bit_counts: HashMap<i32, i32> = HashMap::new();
    for index in 0..12 {
        bit_counts.insert(index, 0);
    }

    for line in lines {
        if let Ok(binary) = line {
            for (index, char) in binary.chars().enumerate() {
                if char =='1' {
                    match bit_counts.get_mut(&(index as i32)) {
                        Some(val) => *val += 1,
                        _ => {}
                    }
                }
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    let lines = read_lines("src/bin/input_day_3.txt").expect("Could not read file contents");
    let total = lines.count();

    for index in 0..12 {
        if let Some(value) = bit_counts.get(&index) {
            println!("Counts index {} value {} total {}", index, value, total);

            if value > &((total / 2) as i32) {
                gamma.push('1');
                epsilon.push('0');
            } else {
                gamma.push('0');
                epsilon.push('1');
            }
        }
    }

    println!("Gamma value {}", gamma);
    println!("Epsilon value {}", epsilon);
    println!("Power consumption {}", isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap())

}
