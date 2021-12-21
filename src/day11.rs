use std::collections::HashSet;

use aoc::read_lines_chars;

// Add 1 to each step and return the ones that hit 9 or above
fn incr(lines: &mut Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    lines
        .iter_mut()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter_mut().enumerate().filter_map(move |(y, v)| {
                if *v >= 9 {
                    *v = 0;
                    Some((x, y))
                } else {
                    *v += 1;
                    None
                }
            })
        })
        .collect()
}

fn incr_around(lines: &mut Vec<Vec<u32>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let operation = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let len_x = lines.len() as i32;
    let len_y = lines[0].len() as i32;
    operation
        .into_iter()
        .filter_map(|(add_x, add_y)| {
            let (t_x, t_y) = (x as i32 + add_x, y as i32 + add_y);
            if t_x >= 0 && t_x < len_x && t_y >= 0 && t_y < len_y {
                let ty = t_y as usize;
                let tx = t_x as usize;
                if lines[tx][ty] >= 9 {
                    lines[tx][ty] = 0;
                    Some((tx, ty))
                } else {
                    lines[tx][ty] += 1;
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn part1(path: &str, steps: u32) -> u32 {
    let mut lines = read_lines_chars(path, |c| c.to_digit(10)).unwrap();
    let mut total = 0;
    for _ in 0..steps {
        let mut hits: Vec<(usize, usize)> = incr(&mut lines);
        let mut flashes = HashSet::<(usize, usize)>::new();
        while !hits.is_empty() {
            if let Some(p) = hits.pop() {
                if !flashes.contains(&p) {
                    flashes.insert(p);
                    hits.append(&mut incr_around(&mut lines, p));
                }
            }
        }
        flashes.iter().for_each(|&(x, y)| {
            lines[x][y] = 0;
            total += 1;
        });
    }
    total
}

fn part2(path: &str) -> u32 {
    let mut lines = read_lines_chars(path, |c| c.to_digit(10)).unwrap();
    let total = lines.len() * lines[0].len();
    let mut i = 0;
    loop {
        i += 1;
        let mut hits: Vec<(usize, usize)> = incr(&mut lines);
        let mut flashes = HashSet::<(usize, usize)>::new();
        while !hits.is_empty() {
            if let Some(p) = hits.pop() {
                if !flashes.contains(&p) {
                    flashes.insert(p);
                    hits.append(&mut incr_around(&mut lines, p));
                }
            }
        }
        if flashes.len() == total {
            break i;
        } else {
            flashes.iter().for_each(|&(x, y)| {
                lines[x][y] = 0;
            });
        }
    }
}

fn main() {
    println!(
        "Results for Part1 are: {:?}",
        part1("fixtures/day11.txt", 100)
    );
    println!("Results for Part2 are: {:?}", part2("fixtures/day11.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day11-test.txt", 1), 0);
        assert_eq!(part1("fixtures/day11-test.txt", 2), 35);
        assert_eq!(part1("fixtures/day11-test.txt", 10), 204);
        assert_eq!(part1("fixtures/day11-test.txt", 100), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day11-test.txt"), 195);
    }
}
