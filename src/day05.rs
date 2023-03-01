use std::{
    cmp::{max, min},
    collections::HashMap,
    num::ParseIntError,
    str::FromStr,
};

use aoc::Point;

#[derive(Debug)]
struct Line {
    origin: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|v| {
                let nums: Vec<&str> = v.split(',').collect();
                Point::new(
                    nums[0].parse::<isize>().unwrap(),
                    nums[1].parse::<isize>().unwrap(),
                )
            })
            .collect::<Vec<Point>>();
        Ok(Line {
            origin: points[0],
            end: points[1],
        })
    }
}

fn challenge1(path: &str) -> usize {
    let rows = aoc::read_lines::<Line>(path)
        .expect("Parsing file failed")
        .iter()
        .fold(HashMap::new(), |mut hash, line| {
            if line.origin.x == line.end.x {
                let begin = min(line.origin.y, line.end.y);
                let end = max(line.origin.y, line.end.y);
                for y in begin..end + 1 {
                    let count = hash.entry((line.origin.x, y)).or_insert(0);
                    *count += 1;
                }
            } else if line.origin.y == line.end.y {
                let begin = min(line.origin.x, line.end.x);
                let end = max(line.origin.x, line.end.x);
                for i in begin..end + 1 {
                    let count = hash.entry((i, line.origin.y)).or_insert(0);
                    *count += 1;
                }
            }
            hash
        });
    rows.values().filter(|&&v| v > 1).count()
}

fn challenge2(path: &str) -> usize {
    let rows = aoc::read_lines::<Line>(path)
        .expect("Parsing file failed")
        .iter()
        .fold(HashMap::new(), |mut hash, line| {
            let delta_x: isize = line.end.x - line.origin.x;
            let delta_y: isize = line.end.y - line.origin.y;
            let steps: isize = (if delta_x == 0 { delta_y } else { delta_x }).abs();
            if steps < 1 {
                return hash;
            }
            for step in 0..steps + 1 {
                let x = line.origin.x + (delta_x / steps) * step;
                let y = line.origin.y + (delta_y / steps) * step;
                let count = hash.entry((x, y)).or_insert(0);
                *count += 1;
            }
            hash
        });
    rows.values().filter(|&&v| v > 1).count()
}

fn main() {
    println!(
        "Day 05 - Challenge 1 results: {:?}",
        challenge1("fixtures/day05.txt")
    );
    println!(
        "Day 05 - Challenge 2 results: {:?}",
        challenge2("fixtures/day05.txt")
    );
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day05-test.txt"), 5);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("fixtures/day05-test.txt"), 12);
    }
}
