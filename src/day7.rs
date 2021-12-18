use std::collections::VecDeque;

use aoc::{read_lines, read_lines_split};

fn part1(path: &str) -> u64 {
    let lines: Vec<Vec<u64>> = read_lines_split(path, |line| {
        Ok(line.split_whitespace().map(|f| f.parse::<u64>().unwrap()))
    })
    .unwrap();
    println!("{:?}", lines);

    0
}

fn part2(path: &str) -> u64 {
    0
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day7.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day7.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day7-test.txt"), 37);
    }

    // #[test]
    // fn test_part2() {
    // }
}
