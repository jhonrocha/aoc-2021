use std::{error::Error, str::FromStr};

use aoc::{self, Point};

#[derive(Debug)]
enum Directions {
    UP(usize),
    DOWN(usize),
    FORWARD(usize),
    BACKWARDS(usize),
}

#[derive(Debug)]
struct PointsAim {
    x: usize,
    y: usize,
    aim: usize,
}
impl PointsAim {
    fn new(x: usize, y: usize, aim: usize) -> PointsAim {
        PointsAim { x, y, aim }
    }
}

impl FromStr for Directions {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" ").collect::<Vec<&str>>().as_slice() {
            ["forward", v] => Ok(Self::FORWARD(v.parse::<usize>()?)),
            ["backward", v] => Ok(Self::BACKWARDS(v.parse::<usize>()?)),
            ["up", v] => Ok(Self::UP(v.parse::<usize>()?)),
            ["down", v] => Ok(Self::DOWN(v.parse::<usize>()?)),
            _ => Err(Box::new(aoc::LineParseError(s.to_string()))),
        }
    }
}

pub fn challenge1(path: &str) -> usize {
    let pos = aoc::parse_file_lines(path, |line| line.parse::<Directions>())
        .iter()
        .fold(Point::new(0, 0), |mut p, dir| {
            match dir {
                Directions::UP(v) => p.y -= v,
                Directions::DOWN(v) => p.y += v,
                Directions::FORWARD(v) => p.x += v,
                Directions::BACKWARDS(v) => p.x -= v,
            };
            p
        });
    pos.x * pos.y
}

pub fn challenge2(path: &str) -> usize {
    let pos = aoc::parse_file_lines(path, |line| line.parse::<Directions>())
        .iter()
        .fold(PointsAim::new(0, 0, 0), |mut p, dir| {
            match dir {
                Directions::UP(v) => p.aim -= v,
                Directions::DOWN(v) => p.aim += v,
                Directions::FORWARD(v) => {
                    p.x += v;
                    p.y += p.aim * v;
                }
                Directions::BACKWARDS(v) => p.x -= v,
            };
            p
        });
    pos.x * pos.y
}

fn main() {
    println!(
        "Day 02 - Challenge 1 results: {:?}",
        challenge1("fixtures/day02.txt")
    );
    println!(
        "Day 02 - Challenge 2 results: {:?}",
        challenge2("fixtures/day02.txt")
    );
}

#[cfg(test)]
mod day2test {
    use super::challenge1;
    use super::challenge2;

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day02-test.txt"), 150);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("fixtures/day02-test.txt"), 900);
    }
}
