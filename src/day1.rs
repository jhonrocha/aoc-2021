use std::fs;

use aoc::parse_file_lines;

pub fn challenge1(filename: &str) -> isize {
    parse_file_lines(filename);
    0
    // let content = fs::read_to_string("./day1-1.txt").expect("File not found");
    // let mut entries: [isize; 3] = [0, 0, 0];
    // let mut counter: isize = 0;
    // for (index, line) in content.lines().enumerate() {
    //     let value: isize = line.parse().expect("Line is not a number");
    //     if (index > 3) && (value > entries[0]) {
    //         counter += 1;
    //     }
    //     entries[0] = entries[1];
    //     entries[1] = entries[2];
    //     entries[2] = value;
    // }
    // counter
}

pub fn challenge2(filename: &str) {
    let content = fs::read_to_string("./day1-1.txt").expect("File not found");
    let mut entries: [isize; 3] = [0, 0, 0];
    let mut counter: isize = 0;

    for (index, line) in content.lines().enumerate() {
        let value: isize = line.parse().expect("Line is not a number");
        if (index > 3) && (value > entries[0]) {
            counter += 1;
        }
        entries[0] = entries[1];
        entries[1] = entries[2];
        entries[2] = value;
    }
    println!("{}", counter);
}

fn main() {
    challenge1("../fixtures/day1.txt");
    challenge2("../fixtures/day1.txt");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day01-test.txt"), 7)
    }
}
