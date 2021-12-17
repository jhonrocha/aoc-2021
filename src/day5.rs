use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct Line {
    origin: (usize, usize),
    end: (usize, usize),
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|v| {
                let nums: Vec<&str> = v.split(",").collect();
                (
                    nums[0].parse::<usize>().unwrap(),
                    nums[1].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<(usize, usize)>>();
        Ok(Line {
            origin: points[0],
            end: points[1],
        })
    }
}

pub fn part1(path: &str) -> usize {
    let rows = read_to_string(path)
        .expect("File not found")
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .collect::<Vec<Line>>()
        .iter()
        .fold(HashMap::new(), |mut hash, line| {
            if line.origin.0 == line.end.0 {
                let begin = min(line.origin.1, line.end.1);
                let end = max(line.origin.1, line.end.1) + 1;
                println!("{:?}", line);
                println!("Begin {:?}", begin);
                println!("End {:?}", end);
                for i in begin..end {
                    let count = hash.entry((i, line.origin.1)).or_insert(0);
                    println!("i {:?}", i);
                    *count += 1;
                    println!("count {:?}", count);
                }
            } else if line.origin.1 == line.end.1 {
                let begin = min(line.origin.0, line.end.0);
                let end = max(line.origin.0, line.end.0) + 1;
                println!("{:?}", line);
                println!("Begin {:?}", begin);
                println!("End {:?}", end);
                for i in begin..end {
                    println!("i {:?}", i);
                    let count = hash.entry((line.origin.0, i)).or_insert(0);
                    *count += 1;
                    println!("count {:?}", count);
                }
            }
            hash
        });
    println!("{:?}", rows);
    rows.values().filter(|&&v| v > 1).collect::<Vec<_>>().len()
}

// fn part2() {}

fn main() {
    assert_eq!(part1("fixtures/day5-test.txt"), 5);
    // println!("{:?}", part1("fixtures/day5.txt"));
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day5-test.txt"), 5);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part1("fixtures/day5-test.txt"), 5);
    }
}
