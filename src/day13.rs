use std::{
    cmp::Ordering::{Equal, Greater},
    collections::HashSet,
    num::ParseIntError,
    str::FromStr,
};

use aoc::{print_iter, read_two_splits};

enum Fold {
    Y(u32),
    X(u32),
}

impl FromStr for Fold {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("fold along ", "");
        let mut v = s.split('=').collect::<Vec<&str>>();
        match *v.first().unwrap() {
            "y" => Ok(Fold::Y(v.pop().unwrap().parse().unwrap())),
            "x" => Ok(Fold::X(v.pop().unwrap().parse().unwrap())),
            _ => Err("Wrong Value".to_string()),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.split(',').collect::<Vec<&str>>();
        Ok(Point {
            y: v.pop().unwrap().parse()?,
            x: v.pop().unwrap().parse()?,
        })
    }
}

fn fold_set(fold: &Fold, set: &mut HashSet<Point>) {
    match fold {
        Fold::Y(line) => {
            for mut point in set.iter().cloned().collect::<Vec<Point>>() {
                match point.y.cmp(line) {
                    Greater => {
                        set.remove(&point);
                        point.y = line * 2 - point.y;
                        set.insert(point);
                    }
                    Equal => {
                        set.remove(&point);
                    }
                    _ => (),
                }
            }
        }
        Fold::X(column) => {
            for mut point in set.iter().cloned().collect::<Vec<Point>>() {
                match point.x.cmp(column) {
                    Greater => {
                        set.remove(&point);
                        point.x = column * 2 - point.x;
                        set.insert(point);
                    }
                    Equal => {
                        set.remove(&point);
                    }
                    _ => (),
                }
            }
        }
    }
}

fn part1(path: &str) -> u32 {
    // Read the file
    let (dots, instructions) = read_two_splits(
        path,
        "\n\n",
        |line| line.parse::<Point>().ok(),
        |line| line.parse::<Fold>().ok(),
    )
    .unwrap();
    let mut dots = dots.into_iter().collect::<HashSet<Point>>();
    fold_set(&instructions[0], &mut dots);
    dots.len() as u32
}

fn print_set(set: &HashSet<Point>) {
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    set.iter().for_each(|&Point { x, y }| {
        if x > max_x {
            max_x = x
        };
        if y > max_y {
            max_y = y
        };
    });
    // print_iter(set.iter());
    for y in 0..=max_y {
        for x in 0..=max_x {
            if set.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn part2(path: &str) -> u32 {
    // Read the file
    let (dots, instructions) = read_two_splits(
        path,
        "\n\n",
        |line| line.parse::<Point>().ok(),
        |line| line.parse::<Fold>().ok(),
    )
    .unwrap();
    let mut dots = dots.into_iter().collect::<HashSet<Point>>();
    instructions.iter().for_each(|ins| fold_set(ins, &mut dots));
    print_set(&dots);
    dots.len() as u32
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day13.txt"));
    println!(
        "Results for Part2 are: {:?}",
        part2("fixtures/day13-test.txt")
    );
    println!("Results for Part2 are: {:?}", part2("fixtures/day13.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day13-test.txt"), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day13-test.txt"), 16);
    }
}
