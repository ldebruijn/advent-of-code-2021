use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(BufReader::new(file).lines())
}

struct Point {
    x: u16,
    y: u16,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x {}, y {}", &self.x, &self.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x {}, y {}", &self.x, &self.y)
    }
}

struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_horizontal_line(&self) -> bool {
        &self.from.x == &self.to.x || &self.from.y == &self.to.y
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line from {} to {}", &self.from, &self.to)
    }
}

fn build_point(input: &str) -> Point {
    let mut split = input.trim().split(",");

    Point {
        x: split.next().expect("X coordinate").parse().expect("Should be a number"),
        y: split.next().expect("Y coordinate").parse().expect("Should be a number"),
    }
}

fn calculate_path(line: &Line) -> Vec<Point> {
    // println!("line {}", line);
    if line.from.x == line.to.x {
        return if line.from.y < line.to.y {
            // println!("Y from > to");
            (line.from.y..line.to.y + 1).into_iter()
                .map(|p| Point {
                    x: line.from.x, //take either original X coordinate
                    y: p,
                })
                .collect()
        } else {
            // println!("Y from < to");
            (line.to.y..line.from.y + 1).into_iter()
                .map(|p| Point {
                    x: line.from.x, //take either original X coordinate
                    y: p,
                })
                .collect()
        };
    }

    return if line.from.x < line.to.x {
        // println!("X from > to");
        (line.from.x..line.to.x + 1).into_iter()
            .map(|p| Point {
                x: p,
                y: line.from.y, //take either original Y coordinate
            })
            .collect()
    } else {
        // println!("X from < to");
        (line.to.x..line.from.x + 1).into_iter()
            .map(|p| Point {
                x: p,
                y: line.from.y, //take either original Y coordinate
            })
            .collect()
    };
}

// todo this doesnt work nicely yet
fn apply_path(path: Vec<Point>, map: &mut Vec<Vec<i32>>) {
    println!("Path {:?}", path);
    for point in path {
        println!("Point {}", point);
        map[point.y as usize][point.x as usize] += 1;
    }
}

fn main() {
    let input = parse_input();

    // build map of vec<vec>
    // iterate over input
    // build path based on from/to
    // increase counter along path

    // map of row/columns
    let mut map = vec![vec![0; 1000]; 1000];

    for line in input {
        if !line.is_horizontal_line() {
            continue;
        }

        let path = calculate_path(&line);
        // println!("{:?}",path);
        apply_path(path, &mut map);
    }

    let overlaps = find_overlaps(&mut map);

    // for row in map {
    //     println!("{:?}", row);
    // }


    println!("Overlaps {}", overlaps);
}

fn find_overlaps(map: &Vec<Vec<i32>>) -> i32 {
    let mut overlaps = 0;

    for row in map {
        for col in row {
            if *col >= 2 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn parse_input() -> Vec<Line> {
    let mut map: Vec<Line> = Vec::new();
    if let Ok(lines) = read_lines("src/bin/input_day_5.txt") {
        for line in lines {
            if let Ok(coord) = line {
                let mut points = coord.split(" -> ");
                let from = build_point(points.next().expect("From point"));
                let to = build_point(points.next().expect("To point"));
                // println!("From {} to {}", from, to);

                map.push(Line {
                    from,
                    to,
                }
                );
            }
        }
    }

    return map;
}
