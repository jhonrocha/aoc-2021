use std::{cmp::Ordering, collections::BinaryHeap};

use aoc::{cardinal_dirs, read_lines_chars, Point};

#[derive(PartialEq, Eq)]
struct State {
    point: Point,
    cost: u32,
}

impl State {
    fn new(point: Point, cost: u32) -> State {
        State { point, cost }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Brute Force
fn part1(path: &str) -> u32 {
    let lines: Vec<Vec<u32>> = read_lines_chars(path, |c| c.to_digit(10)).unwrap();
    let size = lines.len();
    let mut costs = vec![vec![u32::MAX; size]; size];
    let mut unchecked = vec![Point::new(0, 0)];
    costs[0][0] = 0;
    let mut count = 0;
    while !unchecked.is_empty() {
        let pos = unchecked.pop().unwrap();
        count += 1;
        for Point { x, y } in cardinal_dirs(pos, size, size, false) {
            if x == 0 && y == 0 {
                continue;
            }
            let v = costs[pos.x][pos.y] + lines[x][y];
            if v < costs[x][y] {
                costs[x][y] = v;
                unchecked.push(Point::new(x, y));
            }
        }
    }
    println!("Iterations: {:?}", count);
    costs[size - 1][size - 1]
}

fn dijkstra(lines: &[Vec<u32>], start: Point) -> u32 {
    let size = lines.len();
    let mut costs = vec![vec![u32::MAX; size]; size];
    costs[start.x][start.y] = 0;
    let mut unchecked = BinaryHeap::<State>::new();
    unchecked.push(State::new(start, 0));
    let mut count = 0;
    while !unchecked.is_empty() {
        // let State { point, cost: _ } = unchecked.pop().unwrap();
        let State { point, cost } = unchecked.pop().unwrap();
        count += 1;
        for Point { x, y } in cardinal_dirs(point, size, size, false) {
            if x == 0 && y == 0 {
                continue;
            }
            let v = cost + lines[x][y];
            if v < costs[x][y] {
                costs[x][y] = v;
                unchecked.push(State::new(Point::new(x, y), v));
            }
        }
    }
    println!("Iterations: {:?}", count);
    costs[size - 1][size - 1]
}

// Using Dijkstra
fn part2(path: &str) -> u32 {
    let file: Vec<Vec<u32>> = read_lines_chars(path, |c| c.to_digit(10)).unwrap();
    let size = file.len();
    let mut lines = vec![vec![0; size * 5]; size * 5];
    file.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, cost)| {
            for i in 0..5 {
                for j in 0..5 {
                    let new_x = j * size + x;
                    let new_y = i * size + y;
                    let new_cost = cost + j as u32 + i as u32;
                    lines[new_y][new_x] = if new_cost > 9 { new_cost - 9 } else { new_cost };
                }
            }
        });
    });
    dijkstra(&lines, Point::new(0, 0))
}

fn main() {
    // println!("Results for Part1 are: {:?}", part1("fixtures/day15.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day15.txt"));
}

#[cfg(test)]
mod day15 {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day15-test.txt"), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day15-test.txt"), 315);
    }
}
