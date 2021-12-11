use std::collections::HashMap;
use std::fs;

struct Bingo {
    // row/column structure of the bingo cards
    items: Vec<Vec<usize>>
}

impl Bingo {
    fn has_bingo(&self, draws: &[usize]) -> bool {
        // check for bingo horizontally
        for row in &self.items {
            if matching(&row, draws) {
                // println!("bingo!");
                return true; // break all loops
            }
        }

        // check for bingo vertically
        // for each column index
        //  for each row
        //      take value from row[column_index], collect
        // matching
        for col in 0..5 {
            let mut values = Vec::new();
            for row in &self.items {
               values.push(*row.get(col).expect("Bingo value to be there"));
            }
            if matching(&values, draws) {
                // println!("bingo!");
                return true; // break all loops
            }
        }
        false
    }
    fn find_all_unmarked_numbers(&self, draws: &[usize]) -> Vec<usize> {
        let mut undrawn_numbers = Vec::new();
        for row in &self.items {
            for col in row {
                if !draws.contains(col) {
                    undrawn_numbers.push(*col);
                }
            }
        }
        return undrawn_numbers
    }
}

// need not check if its matching, but if is a full match for the contents of b
fn matching(a: &[usize], total: &[usize]) -> bool {
    a.iter().all(|item| total.contains(item))
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
        let mut items : Vec<Vec<usize>> = Vec::new();
        println!("Board {}", board);
        let input: Vec<usize> = board.split_whitespace()
            .into_iter()
            .map(|x| {
                x.parse().expect("Should be a number")
            })
            .collect();
        // single board as one array, iterate over it and create a new vec ever 5th element for a new row
        let chunks = input.chunks(5);

        for chunk in chunks {
            // println!("i {}", i);
            items.push(chunk.to_vec());
        }

        println!("items {:?} ", items);
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
    let (draws, boards) = read_input();
    let draws: Vec<usize> = draws.iter().map(|x| x.trim().parse().expect("Draw not a number")).collect();
    println!("Draws {:?}", draws);

    let won_game = play_bingo(&boards, &draws);

    println!("Bingo! Draws {:?} last winning board {:?}", won_game.draws, won_game.winning_board);

    let winning_board = boards.get(&won_game.winning_board).expect("Index should point to last winning board");
    println!("Last winning board {:?}", winning_board.items);
    let undrawn_numbers = winning_board.find_all_unmarked_numbers(&*won_game.draws);
    println!("Undrawn numbers {:?}", undrawn_numbers);
    let sum_undrawn_numbers: usize = undrawn_numbers.iter().sum();
    println!("Sum {}", sum_undrawn_numbers);

    let last_draw = won_game.draws.last().expect("Last draw");

    println!("Final score {}*{}= {}",sum_undrawn_numbers, last_draw, sum_undrawn_numbers * last_draw)

}

fn play_bingo(boards: &HashMap<usize, Bingo>, draws: &Vec<usize>) -> WonGame {
    let mut last_bingo_draws: &[usize] = &[];
    let mut last_winning_board = None;
    let mut bingo_boards: Vec<usize> =Vec::new();

    for (index,_) in draws.iter().enumerate() {
        let draw = &draws[..index];
        println!("draw {:?}", draw);
        for board in boards {
            if bingo_boards.contains(board.0) {
                continue;
            }

            if (board.1).has_bingo(draw) {
                println!("Board has bingo {}", board.0);
                last_bingo_draws = draw;
                last_winning_board = Some(*board.0);
                bingo_boards.push(*board.0);
            }
        }
    }
    return WonGame {
        draws: last_bingo_draws.to_vec(),
        winning_board: last_winning_board.expect("A board should have won")
    }
}

struct WonGame {
    draws : Vec<usize>,
    winning_board: usize
}
