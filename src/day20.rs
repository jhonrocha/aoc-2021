use std::{collections::HashSet, fs::read_to_string};

use aoc::{cardinal_dirs, Point};

fn sort_points(points: &mut Vec<Point>) -> u32 {
    0
}

fn enhancement(img: &mut HashSet<Point>, mask: Vec<u8>, times: u32) {
    for i in 0..times {
        let aux = HashSet::<Point>::new();
        img.iter().for_each(|p| {
            let mut neighbors = cardinal_dirs(p, usize::MAX, usize::MAX, true);
            sort_points(&mut neighbors);
            let pos = neighbors.iter().fold(0, |mut acc, n| {
                acc *= 2;
                if img.contains(n) {
                    acc + 1
                } else {
                    acc
                }
            });
            if mask.get(pos).unwrap() {
                aux.insert(p);
            }
        });
    }
}

fn part1(path: &str, times: u32) -> u32 {
    let mut img = read_to_string(path).unwrap().lines().enumerate().fold(
        HashSet::<Point>::new(),
        |mut hash, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    hash.insert(Point::new(x, y));
                }
            });
            hash
        },
    );
    let mask = vec![0; 10];
    enhancement(&mut img, mask, times);
    0
}

fn part2(path: &str) {}

fn main() {
    println!("Part1: {:?}", part1("fixtures/day20.txt", 2));
}

#[cfg(test)]
mod day20 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day20-test.txt", 2), 10);
    }
}
