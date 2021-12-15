use std::collections::HashMap;
use std::fmt::{Debug, Formatter, write};
use std::fs;
use std::hash::Hash;

#[derive(Clone)]
struct LanternFish {
    timer: u8
}

impl Debug for LanternFish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.timer)
    }
}

impl LanternFish {
    fn pass_day_and_generate_offspring(&mut self) -> bool {
        if &self.timer < &1u8 {
            self.timer = 6;
            return true;
        }
        self.timer -= 1;
        return false;
    }
}

fn parse_input() -> HashMap<u8, u64> {
    let raw = fs::read_to_string("src/bin/input_day_6.txt").expect("Could not read file");
    let mut input: HashMap<u8, u64> = HashMap::new();
    input.insert(0, 0);
    input.insert(1, 0);
    input.insert(2, 0);
    input.insert(3, 0);
    input.insert(4, 0);
    input.insert(5, 0);
    input.insert(6, 0);
    input.insert(7, 0);
    input.insert(8, 0);

    raw.trim().split(",")
        .for_each(|x| {
            let x: u8 = x.parse().expect("Value to be a number");
            if let Some(val) = input.get_mut(&x) {
                *val += 1;
            }
        });
    return input
}

fn main() {
    let mut fishes = parse_input();
    let num_days = 256;

    for day in 0..num_days {
        // Move each value one key down i.e. 5: 20: 4: 10 -> 5:20, 3: 10
        // add everything from 0 back to 6
        // add equal number from 0 to 8
        let mut new_map: HashMap<u8, u64> = HashMap::new();
        new_map.insert(0, 0);
        new_map.insert(1, 0);
        new_map.insert(2, 0);
        new_map.insert(3, 0);
        new_map.insert(4, 0);
        new_map.insert(5, 0);
        new_map.insert(6, 0);
        new_map.insert(7, 0);
        new_map.insert(8, 0);

        *new_map.get_mut(&8).unwrap() = fishes[&0];
        *new_map.get_mut(&7).unwrap() = fishes[&8];
        *new_map.get_mut(&6).unwrap() = fishes[&7] + fishes[&0];
        *new_map.get_mut(&5).unwrap() = fishes[&6];
        *new_map.get_mut(&4).unwrap() = fishes[&5];
        *new_map.get_mut(&3).unwrap() = fishes[&4];
        *new_map.get_mut(&2).unwrap() = fishes[&3];
        *new_map.get_mut(&1).unwrap() = fishes[&2];
        *new_map.get_mut(&0).unwrap() = fishes[&1];

        fishes = new_map;

        println!("Ending day {} with {} fish", day, fishes.values().sum::<u64>());
    }

    println!("Number of fish after {} days is {}", num_days, fishes.values().sum::<u64>());
}
