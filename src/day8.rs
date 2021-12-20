use std::{collections::HashSet, str::FromStr, string::ParseError};

enum Digit {
    One,
    Four,
    Seven,
    Eight,
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            2 => Ok(Digit::One),
            4 => Ok(Digit::Four),
            3 => Ok(Digit::Seven),
            7 => Ok(Digit::Eight),
            _ => Err(String::from("Number not valid")),
        }
    }
}

fn read_outputs(path: &str) -> usize {
    // let lines: Vec<Vec<&str>> =
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.split('|')
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .split_whitespace()
                .filter_map(|digit| digit.parse::<Digit>().ok())
        })
        .count()
}

fn sonar_output(path: &str) -> usize {
    // let lines: Vec<Vec<&str>> =
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        // .flat_map(|line| {
        .map(|line| {
            let entries = line.split('|').collect::<Vec<&str>>();
            let signals = entries[0];
            let a: Vec<HashSet<_>> = signals
                .split_whitespace()
                .map(|pat| pat.chars().collect::<HashSet<_>>())
                .collect();
            println!("{:?}", a);
            let numbers = entries[1];
            // .pop()
            // .unwrap()
            // .split_whitespace()
            // .filter_map(|digit| digit.parse::<Digit>().ok())
            0
        })
        .count();
    0
}

fn part1(path: &str) -> usize {
    read_outputs(path)
}

fn part2(path: &str) -> usize {
    sonar_output(path)
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day8.txt"));
    // println!("Results for Part2 are: {:?}", part2("fixtures/day8.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day8-test.txt"), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day8-test.txt"), 168);
    }

    // cf      1
    // acf     7
    // bcdf    4
    // acdeg   2
    // acdfg   3
    // abdfg   5
    // abcefg  0
    // abdefg  6
    // abcdfg  9
    // abcdefg 8
}
