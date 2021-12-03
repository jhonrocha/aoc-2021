use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

pub fn part2(path: &str) -> isize {
    println!("{}", path);
    1
}

#[cfg(test)]
mod anything {
    // use super::{part1, part2};
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("./src/day3/test.txt"), 198);
        println!("Part1 result is {}", part1("./src/day3/input.txt"))
    }
}
