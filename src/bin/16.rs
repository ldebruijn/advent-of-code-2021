use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

fn parse_input() -> Vec<Vec<u8>> {
    let mut results: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = read_lines("src/bin/input_day_9.txt") {
        for line in lines {
            if let Ok(val) = line {
                let row = val.chars().map(|c| c.to_string().parse().expect("Valid number")).collect();
                results.push(row);
            }
        }
    }

    return results;
}


fn main() {
    let map = parse_input();

    // start top left and traverse to bottom right
    // check for neighbours on vertical & horizontal sides, make sure row > 0 and col > 0
    // If current is lowest, highlight,
    let mut total_risk_level: u32 = 0;

    for (row_index, row) in map.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            // find neighbours
            let top_neighbour = find_top_neighbour(row_index, &map, col_index);
            let left_neighbour = find_left_neighbour(col_index, &row);
            let right_neighbour = find_right_neighbour(col_index, &row);
            let bottom_neighbour = find_bottom_neighbour(row_index, &map, col_index);

            // println!("Top {:?} Left {:?} Right {:?} bottom {:?}", top_neighbour, left_neighbour, right_neighbour, bottom_neighbour);

            if is_lowest(value, top_neighbour, left_neighbour, right_neighbour, bottom_neighbour) {
                let risk_level = *value as u32 + 1;
                println!("Lowest point! {} Risk Level {} Top {:?} Left {:?} Right {:?} bottom {:?}", value, risk_level, top_neighbour, left_neighbour, right_neighbour, bottom_neighbour);
                total_risk_level += risk_level;
            }
        }
    }

    println!("Total risk level: {}", total_risk_level);
}

fn is_lowest(value: &u8, top_neighbour: Option<u8>, left_neighbour: Option<u8>, right_neighbour: Option<u8>, bottom_neighbour: Option<u8>) -> bool {
    let top = match top_neighbour {
        Some(val) => val,
        // assign max to essentially ignore this value if None
        _ => u8::MAX
    };
    let left = match left_neighbour {
        Some(val) => val,
        // assign max to essentially ignore this value if None
        _ => u8::MAX
    };
    let right = match right_neighbour {
        Some(val) => val,
        // assign max to essentially ignore this value if None
        _ => u8::MAX
    };
    let bottom = match bottom_neighbour {
        Some(val) => val,
        // assign max to essentially ignore this value if None
        _ => u8::MAX
    };

    value < &top && value < &left && value < &right && value < &bottom
}

fn find_top_neighbour(row_index: usize, map: &Vec<Vec<u8>>, col_index: usize) -> Option<u8> {
    if row_index < 1 {
        return None;
    }

    Some(map[row_index - 1][col_index])
}

fn find_bottom_neighbour(row_index: usize, map: &Vec<Vec<u8>>, col_index: usize) -> Option<u8> {
    if row_index >= map.len() - 1 {
        return None;
    }

    Some(map[row_index + 1][col_index])
}

fn find_left_neighbour(col_index: usize, col: &[u8]) -> Option<u8> {
    if col_index < 1 {
        return None;
    }

    Some(col[col_index - 1])
}

fn find_right_neighbour(col_index: usize, col: &[u8]) -> Option<u8> {
    if col_index >= col.len() - 1 {
        return None;
    }

    Some(col[col_index + 1])
}
