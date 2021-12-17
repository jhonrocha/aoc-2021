use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read_to_string,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct Line {
    origin: (isize, isize),
    end: (isize, isize),
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|v| {
                let nums: Vec<&str> = v.split(",").collect();
                (
                    nums[0].parse::<isize>().unwrap(),
                    nums[1].parse::<isize>().unwrap(),
                )
            })
            .collect::<Vec<(isize, isize)>>();
        Ok(Line {
            origin: points[0],
            end: points[1],
        })
    }
}

fn read_lines(path: &str) -> Vec<Line> {
    read_to_string(path)
        .expect("File not found")
        .lines()
        .map(|l| l.parse::<Line>().unwrap())
        .collect::<Vec<Line>>()
}

fn part1(path: &str) -> usize {
    let rows = read_lines(path)
        .iter()
        .fold(HashMap::new(), |mut hash, line| {
            if line.origin.0 == line.end.0 {
                let begin = min(line.origin.1, line.end.1);
                let end = max(line.origin.1, line.end.1) + 1;
                for i in begin..end {
                    let count = hash.entry((line.origin.0, i)).or_insert(0);
                    *count += 1;
                }
            } else if line.origin.1 == line.end.1 {
                let begin = min(line.origin.0, line.end.0);
                let end = max(line.origin.0, line.end.0) + 1;
                for i in begin..end {
                    let count = hash.entry((i, line.origin.1)).or_insert(0);
                    *count += 1;
                }
            }
            hash
        });
    rows.values().filter(|&&v| v > 1).collect::<Vec<_>>().len()
}

fn part2(path: &str) -> usize {
    let rows = read_lines(path)
        .iter()
        .fold(HashMap::new(), |mut hash, line| {
            if line.origin.0 == line.end.0 {
                let begin = min(line.origin.1, line.end.1);
                let end = max(line.origin.1, line.end.1) + 1;
                for i in begin..end {
                    let count = hash.entry((line.origin.0, i)).or_insert(0);
                    *count += 1;
                }
            } else if line.origin.1 == line.end.1 {
                let begin = min(line.origin.0, line.end.0);
                let end = max(line.origin.0, line.end.0) + 1;
                for i in begin..end {
                    let count = hash.entry((i, line.origin.1)).or_insert(0);
                    *count += 1;
                }
            } else {
                let diff_x = line.end.0 - line.origin.0;
                let diff_y = line.end.1 - line.origin.1;
                if (diff_x / diff_y).abs() == 1 {
                    let step_x = if diff_x > 0 { 1 } else { -1 };
                    let step_y = if diff_y > 0 { 1 } else { -1 };
                    for step in 0..diff_x.abs() + 1 {
                        let count = hash
                            .entry((line.origin.0 + step * step_x, line.origin.1 + step * step_y))
                            .or_insert(0);
                        *count += 1;
                    }
                }
            }
            hash
        });
    rows.values().filter(|&&v| v > 1).collect::<Vec<_>>().len()
}

fn main() {
    println!("The Result for Part 1 is: {:?}", part1("fixtures/day5.txt"));
    println!("The Result for Part 2 is: {:?}", part2("fixtures/day5.txt"));
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
        assert_eq!(part2("fixtures/day5-test.txt"), 12);
    }
}
