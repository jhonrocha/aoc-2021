use std::{
    char,
    fs::{read_to_string, File},
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

pub fn challenge1(path: &str) -> usize {
    let mut total = 0;
    let mut gamma = 0;
    let mut epsilon = 0;
    read_to_string(path)
        .expect("File is missing!")
        .lines()
        .into_iter()
        .fold(Vec::<usize>::new(), |mut acc, line| {
            line.chars().enumerate().for_each(|(idx, c)| {
                if c == '1' {
                    if let Some(v) = acc.get(idx) {
                        acc[idx] = v + 1;
                    } else {
                        acc.push(1);
                    }
                }
            });
            total += 1;
            acc
        })
        .into_iter()
        .for_each(|occurrency| {
            if occurrency > total / 2 {
                gamma = (gamma << 1) + 1;
                epsilon = epsilon << 1;
            } else {
                gamma = gamma << 1;
                epsilon = (epsilon << 1) + 1;
            }
        });
    gamma * epsilon
}

pub fn challenge2(path: &str) -> usize {
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

fn main() {
    println!(
        "Day 03 - Challenge 1 results: {:?}",
        challenge1("fixtures/day03.txt")
    );
    println!(
        "Day 03 - Challenge 2 results: {:?}",
        challenge2("fixtures/day03.txt")
    );
}

#[cfg(test)]
mod anything {
    // use super::{challenge1, challenge2};
    use super::*;
    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day03-test.txt"), 198);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("fixtures/day03-test.txt"), 230);
    }
}
