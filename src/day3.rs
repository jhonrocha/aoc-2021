use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

fn bin_to_dec(value: &Vec<char>) -> usize {
    let mut resp: usize = 0;
    value.iter().rev().enumerate().for_each(|(index, v)| {
        if v == &'1' {
            resp += (2 as usize).pow(index as u32);
        }
    });
    resp
}
pub fn part1(path: &str) -> usize {
    let file = File::open(path).expect(&format!("Could not open file {}", path));
    let reader = BufReader::new(file);
    let mut columns = Vec::new();
    let mut lines: usize = 0;

    reader.lines().for_each(|line| {
        let line = line.expect("Could not read a line");
        line.chars().enumerate().for_each(|(index, val)| {
            if val == '1' {
                if columns.len() > index {
                    columns[index] += 1;
                } else {
                    columns.push(1);
                }
            }
        });
        lines += 1;
    });
    let mut gamma: usize = 0;
    let mut epsilon = 0;
    let lines = lines / 2;
    columns.iter().rev().enumerate().for_each(|(index, v)| {
        if v >= &lines {
            // Column index major is 1
            gamma += (2 as usize).pow(index as u32);
        } else {
            // Column index major is 0
            epsilon += (2 as usize).pow(index as u32);
        }
    });
    gamma * epsilon
}

pub fn part2(path: &str) -> usize {
    let file = File::open(path).expect(&format!("Could not open file {}", path));
    let reader = BufReader::new(file);
    let mut oxygen: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("Line could not be parsed").chars().collect())
        .collect();
    let mut co2 = oxygen.clone();
    let mut idx = 0;
    while oxygen.len() > 1 {
        let mut oxyg1 = Vec::new();
        let mut oxyg0 = Vec::new();
        for line in oxygen {
            if line[idx] == '1' {
                oxyg1.push(line);
            } else {
                oxyg0.push(line);
            }
        }
        if oxyg1.len() >= oxyg0.len() {
            oxygen = oxyg1;
        } else {
            oxygen = oxyg0;
        }
        idx += 1;
    }
    let oxygen = bin_to_dec(&oxygen[0]);

    let mut idx = 0;
    while co2.len() > 1 {
        let mut c1 = Vec::new();
        let mut c0 = Vec::new();
        for line in co2 {
            if line[idx] == '1' {
                c1.push(line);
            } else {
                c0.push(line);
            }
        }
        if c1.len() < c0.len() {
            co2 = c1;
        } else {
            co2 = c0;
        }
        idx += 1;
    }
    let co2 = bin_to_dec(&co2[0]);
    co2 * oxygen
}

#[cfg(test)]
mod anything {
    // use super::{part1, part2};
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/day3/test.txt"), 198);
    }

    #[test]
    fn run_part1() {
        println!("Part1 result is {}", part1("./src/day3/input.txt"))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("./src/day3/test.txt"), 230);
    }

    #[test]
    fn run_part2() {
        println!("Part2 result is {}", part2("./src/day3/input.txt"))
    }
}
