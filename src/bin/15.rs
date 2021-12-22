use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

struct Input {
    input: Vec<String>,
    output: Vec<String>,
}

fn parse_input() -> Vec<Input> {
    let mut results: Vec<Input> = Vec::new();
    if let Ok(lines) = read_lines("src/bin/input_day_8.txt") {
        for line in lines {
            if let Ok(val) = line {
                let mut split = val.split("|");

                results.push(Input {
                    input: split.next().expect("Value expected")
                        .split(" ")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                    output: split.next().expect("Value expected")
                        .split(" ")
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                })
            }
        }
    }

    return results;
}

fn main() {
    /*
    1: ad
    2:
    3:
    4: gadf
    5:
    6:
    7: agd
    8: afgedbc
    9:

 gggg
e    a
e    a
 ffff
g    b
g    b
 cccc

      0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
*/

    let input = parse_input();

    let mut result: i32 = 0;

    for i in input {
        let one = find_unique_digit(&i, &|s| s.len() == 2).expect("One");
        let four = find_unique_digit(&i, &|s| s.len() == 4).expect("Four");
        let seven = find_unique_digit(&i, &|s| s.len() == 3).expect("Seven");
        let eight = find_unique_digit(&i, &|s| s.len() == 7).expect("Eight");

        // println!("One {} four {} seven {} eight {}", one, four, seven, eight);

        let three = find_three(&i, &one).expect("Three");
        let five = find_five(&i, &one, &four).expect("Five");
        let two = find_two(&i, &three, &five).expect("Two");

        // println!("Two {} Three {} Five {}", two, three, five);

        let nine = find_nine(&i, three).expect("Nine");
        let zero = find_zero(&i, nine, seven).expect("Zero");
        let six = find_six(&i, zero, nine).expect("Six");

        // println!("Zero {} Six {} Nine {}", zero, six, nine);

        let mut output = String::new();
        for index in 0..i.output.len() {
            let element = &i.output[index];

            if find_exact_match(&element, &zero) {
                output += "0";
            } else if find_exact_match(&element, &one) {
                output += "1";
            } else if find_exact_match(&element, &two) {
                output += "2";
            } else if find_exact_match(&element, &three) {
                output += "3";
            } else if find_exact_match(&element, &four) {
                output += "4";
            } else if find_exact_match(&element, &five) {
                output += "5";
            } else if find_exact_match(&element, &six) {
                output += "6";
            } else if find_exact_match(&element, &seven) {
                output += "7";
            } else if find_exact_match(&element, &eight) {
                output += "8";
            } else if find_exact_match(&element, &nine) {
                output += "9";
            }
        }

        println!("Output {:?} result {}", i.output, output);

        result += output.parse::<i32>().expect("Valid number");
    }

    println!("Result {}", result)
}

fn find_exact_match(original: &str, match_against: &str) -> bool {
    if original.len() != match_against.len() {
        return false;
    }

    find_fully_contains(original, match_against)
}

// Six is one of the 6 length characters, amongst 0 and 9
// We can find six by filtering the input for length 6 segments
// and removing 0 and 9 from the list.
fn find_six<'a>(line: &'a Input, zero: &str, nine: &str) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 6 && element != zero && element != nine {
            return Some(element);
        }
    }
    return None;
}

// Zero is one of the 6 length characters, amongst 6 and 9
// We can find zero by filtering the input for length 6 segments
// ignoring the segment that we've already identified for nine
// and finding a full match for the segments of 7
fn find_zero<'a>(line: &'a Input, nine: &str, seven: &str) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 6 && element != nine && find_fully_contains(element, seven) {
            return Some(element);
        }
    }
    return None;
}

// Nine is one of the 6 length characters, amongst 0 and 6
// We can find nine by filtering the input for length 6 segments
// and finding a full match for the segments of 3
fn find_nine<'a>(line: &'a Input, three: &str) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 6 && find_fully_contains(element, three) {
            return Some(element);
        }
    }
    return None;
}

// Two is one of the 5 length characters, amongst 3 and 5
// We can find two by filtering the input for length 5 segments
// and removing 3 and 5 from the list.
fn find_two<'a>(line: &'a Input, one: &str, four: &str) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 5 && element != one && element != four {
            return Some(element);
        }
    }
    return None;
}

// Five is one of the 5 length characters, amongst 2 and 3
// Take the 4 segment, remote the one segment from that to leave the middle and top left segment
// compare the result to the length 5 segments, that is 5
fn find_five<'a>(line: &'a Input, one: &str, four: &str) -> Option<&'a str> {
    let identifying_segments = remove_matching_segments(four, one);

    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 5 && find_fully_contains(&element, &identifying_segments) {
            return Some(element);
        }
    }
    return None;
}

fn remove_matching_segments(original: &str, segments_to_remove: &str) -> String {
    original.chars().filter(|c| !segments_to_remove.contains(&c.to_string())).collect()
}

// Three is one of the 5 length characters, amongst 2 and 5
// Three fully matches with the segments of 1, so use that to identify 3.
fn find_three<'a>(line: &'a Input, one: &str) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if element.len() == 5 && find_fully_contains(element, one) {
            return Some(element);
        }
    }
    return None;
}

fn find_fully_contains(original: &str, contains: &str) -> bool {
    contains.chars().all(|c| original.contains(c))
}

fn find_non_overlap(a: &str, b: &str) -> String {
    a.chars().filter(|x| !b.contains(&x.to_string())).collect()
}

fn find_unique_digit<'a>(line: &'a Input, condition: &dyn Fn(&String) -> bool) -> Option<&'a str> {
    for i in 0..line.input.len() {
        let element = &line.input[i];
        if condition(element) {
            return Some(element);
        }
    }
    return None;
}
