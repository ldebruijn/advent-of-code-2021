use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn main() {
    // read input file into vector
    // create map with size of vector
    // index = 0
    // for each line in file
    //      check if map[index] exists
    //      if not, create map[index, [values]]
    //      else get map[index].put(line)
    //
    // let mut increases = 0;
    // for each item in map
    //  if item.value.length == 3
    //      let next = map[index+1]
    //      if next.value.length < 3 -> continue
    //          if next > item -> increases += 1;
    let mut increases = 0;

    let mut values: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("./src/bin/day_1/input.txt") {
        for line in lines {
            if let Ok(current) = line {
                values.push(current.parse().expect("Value not a valid number"));
            }
        }
    }

    println!("Printing values {:?}", values);

    let mut windows: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for value in &values {
        println!("Printing value {:?}", value);
        let next = match values.get(index as usize + 1) {
            None => continue,
            Some(val) => val
        };
        let next2 = match values.get(index as usize + 2) {
            None => continue,
            Some(val) => val
        };

        let total = *value + *next + *next2;
        println!("Printing total {:?}", total);
        windows.insert(index, total);
        index += 1;
    }

    for elem in &windows {
        let current = elem.1;
        let next = match windows.get(&(elem.0 + 1)) {
            Some(val) => *val,
            None => continue
        };

        if next > *current {
            increases += 1;
        }
    }
    println!("Total increases is {}", increases)
}
