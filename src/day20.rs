use std::{collections::HashSet, fs::read_to_string};

fn part1(path: &str) -> u32 {
    let hash = read_to_string(path).unwrap().lines().enumerate().fold(
        HashSet::new(),
        |mut hash, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    hash.insert((x, y));
                }
            });
            hash
        },
    );
    println!("hash: {:?}", hash);
    0
}

fn part2(path: &str) {}

fn main() {
    println!("Part1: {:?}", part1("fixtures/day20.txt"));
}

#[cfg(test)]
mod day20 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day20-test.txt"), 10);
    }
}
