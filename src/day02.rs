use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use aoc::{self, Point};

#[derive(Debug)]
enum Directions {
    UP(usize),
    DOWN(usize),
    FORWARD(usize),
    BACKWARDS(usize),
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn challenge2(path: &str) -> isize {
    let f = File::open(path).expect(&format!("Could no open file {}", path));
    let reader = BufReader::new(f);
    // Directions: (horizontal, depth, aim)
    let mut directions: (isize, isize, isize) = (0, 0, 0);
    for l in reader.lines() {
        let line = l.expect("missing line");
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["down", v] => directions.2 += v.parse::<isize>().unwrap(),
            ["up", v] => directions.2 -= v.parse::<isize>().unwrap(),
            ["forward", v] => {
                let x = v.parse::<isize>().unwrap();
                directions.0 += x;
                directions.1 += directions.2 * x;
            }
            _ => (),
        };
    }
    directions.0 * directions.1
}

fn main() {
    println!(
        "Day 02 - Challenge 1 results: {:?}",
        challenge1("fixtures/day02.txt")
    )
}

#[cfg(test)]
mod day2test {
    use super::challenge1;
    use super::challenge2;

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day02-test.txt"), 150);
    }

    // #[test]
    // fn test_challenge2() {
    //     assert_eq!(challenge2("fixtures/day01-test.txt"), 900);
    // }
}
