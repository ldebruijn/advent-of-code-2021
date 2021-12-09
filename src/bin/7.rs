use std::collections::HashMap;
use std::fs;

struct Bingo {
    items: HashMap<usize, bool>,
}

impl Bingo {
    fn has_bingo(&self) -> bool {
        false
    }
    fn draw(&mut self, number: usize) {
        match self.items.get_mut(&number) {
            None => {}
            Some(_) => {
                true;
                ()
            }
        }
    }
}

fn read_input() -> (Vec<String>, HashMap<usize, Bingo>) {
    let splits = {
        let input = fs::read_to_string("src/bin/input_day_4.txt").expect("Could not read file");

        let splits: Vec<String> = input.split("---")
            .filter(|x| !x.is_empty())
            .map(|x| {
                String::from(x)
            })
            .map(|x| {
                x.replace("\n", " ")
            })
            .collect();

        splits
    };

    let draws = splits.get(0)
        .expect("First index is draws")
        .split(",")
        .into_iter()
        .map(|x| x.into())
        .collect();

    let mut boards: HashMap<usize, Bingo> = HashMap::new();

    for (index, board) in splits.iter().skip(1).enumerate() {
        let mut items: HashMap<usize, bool> = HashMap::new();
        // println!("Board {}", board);
        let input: Vec<usize> = board.split_whitespace()
            .into_iter()
            .map(|x| {
                x.parse().expect("Should be a number")
            })
            .collect();
        for i in input {
            items.insert(i, false);
        }

        boards.insert(
            index,
            Bingo {
                items
            },
        );
    }

    (draws, boards)
}

fn main() {
    // read input file
    // draws into array of u8
    // read board into structs
    let (draws, mut boards) = read_input();

    for number in draws {
        let number = number.parse().expect("Draw not a number");
        for board in &mut boards {
            board.1.draw(number);
            if (&board.1).has_bingo() {
                break; // break entire loop, keep track of bingo requirements
            }
        }
    }
}
