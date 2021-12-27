use std::{collections::HashSet, fs::read_to_string};

use aoc::Point;

fn step(
    east: &mut HashSet<Point>,
    south: &mut HashSet<Point>,
    lines: usize,
    columns: usize,
) -> usize {
    let mut changes = 0;
    let keys = east.clone();
    keys.iter().for_each(|p| {
        let x = if p.x + 1 < columns { p.x + 1 } else { 0 };
        let next = Point { x, y: p.y };
        if !keys.contains(&next) && !south.contains(&next) {
            east.remove(p);
            east.insert(next);
            changes += 1;
        }
    });
    let keys = south.clone();
    keys.iter().for_each(|p| {
        let y = if p.y + 1 < lines { p.y + 1 } else { 0 };
        let next = Point { x: p.x, y };
        if !east.contains(&next) && !keys.contains(&next) {
            south.remove(p);
            south.insert(next);
            changes += 1;
        }
    });
    changes
}

fn part1(path: &str) -> u64 {
    let mut east = HashSet::<Point>::new();
    let mut south = HashSet::<Point>::new();
    let file = read_to_string(path).unwrap();
    let mut lines: usize = 0;
    let mut columns: usize = 0;
    file.lines().enumerate().for_each(|(y, line)| {
        lines += 1;
        line.chars().enumerate().for_each(|(x, c)| {
            if y == 0 {
                columns += 1;
            }
            match c {
                'v' => south.insert(Point { x, y }),
                '>' => east.insert(Point { x, y }),
                _ => true,
            };
        })
    });
    let mut steps = 0;
    while step(&mut east, &mut south, lines, columns) > 0 {
        steps += 1;
    }
    steps + 1
}

// fn part2(path: &str) -> u64 {}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day25.txt"));
    // println!("Results for Part2 are: {:?}", part1("fixtures/day25.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day25-test.txt"), 58);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("fixtures/day25-test.txt"), 2188189693529);
    // }
}
