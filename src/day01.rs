use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use aoc;

pub fn challenge1(filename: &str) -> usize {
    let f = File::open(filename).expect("missing file");
    let reader = BufReader::new(f);
    let mut previous = usize::MAX;
    reader.lines().fold(0, |mut acc, line| {
        let current = line.expect("Missing line").parse::<usize>().unwrap();
        if current > previous {
            acc += 1;
        }
        previous = current;
        acc
    })
}

pub fn challenge2(filename: &str) -> usize {
    let mut counter = 0;
    let mut idx = 0;
    aoc::file_lines_iter(filename).fold([0, 0, 0], |acc, line| {
        let current = line.expect("Missing line").parse::<usize>().unwrap();
        if (idx > 2) && (current > acc[2]) {
            counter += 1;
        }
        idx += 1;
        [current, acc[0], acc[1]]
    });
    counter
}

fn main() {
    println!(
        "Day 01 - First Challenge result: {:?}",
        challenge1("fixtures/day01.txt")
    );
    println!(
        "Day 01 - Second Challenge result: {:?}",
        challenge2("fixtures/day01.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day01-test.txt"), 7)
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("fixtures/day01-test.txt"), 5)
    }
}
