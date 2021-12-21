use std::collections::HashSet;

use aoc::read_lines_chars;

struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn new(x: usize, y: usize, height: usize) -> Self {
        Point { x, y, height }
    }
}

fn find_min(grid: &[Vec<usize>]) -> Vec<Point> {
    let lines = grid.len() - 1;
    let columns = grid[0].len() - 1;
    grid.iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(j, &value)| {
                    if (i > 0 && value >= grid[i - 1][j])
                        || (i < lines && value >= grid[i + 1][j])
                        || (j > 0 && value >= grid[i][j - 1])
                        || (j < columns && value >= grid[i][j + 1])
                    {
                        None
                    } else {
                        Some(Point::new(i as usize, j as usize, value))
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn part1(path: &str) -> usize {
    let lines: Vec<Vec<usize>> =
        read_lines_chars(path, |c| c.to_digit(10).map(|v| v as usize)).unwrap();
    find_min(&lines).iter().map(|p| p.height + 1).sum()
}

fn part2(path: &str) -> usize {
    let lines: Vec<Vec<usize>> =
        read_lines_chars(path, |c| c.to_digit(10).map(|v| v as usize)).unwrap();
    let max_lines = lines.len() - 1;
    let max_columns = lines[0].len() - 1;

    let mut bains = find_min(&lines)
        .iter()
        .map(|p| {
            let mut points = HashSet::<(usize, usize)>::new();
            points.insert((p.x, p.y));
            let mut check = vec![(p.x, p.y)];
            let mut len = 1;
            while len > 0 {
                let mut aux = Vec::new();
                for (x, y) in check {
                    if x > 0 && lines[x - 1][y] != 9 && points.get(&(x - 1, y)) == None {
                        points.insert((x - 1, y));
                        aux.push((x - 1, y));
                    }
                    if x < max_lines && lines[x + 1][y] != 9 && points.get(&(x + 1, y)) == None {
                        points.insert((x + 1, y));
                        aux.push((x + 1, y));
                    }
                    if y > 0 && lines[x][y - 1] != 9 && points.get(&(x, y - 1)) == None {
                        points.insert((x, y - 1));
                        aux.push((x, y - 1));
                    }
                    if y < max_columns && lines[x][y + 1] != 9 && points.get(&(x, y + 1)) == None {
                        points.insert((x, y + 1));
                        aux.push((x, y + 1));
                    }
                }
                len = aux.len();
                check = aux;
            }
            points.len()
        })
        .collect::<Vec<usize>>();
    bains.sort_unstable();
    bains
        .iter()
        .rev()
        .take(3)
        .cloned()
        .reduce(|mult, v| mult * v)
        .unwrap()
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day9.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day9.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day9-test.txt"), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day9-test.txt"), 1134);
    }
}
