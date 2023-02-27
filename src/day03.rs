use std::{char, fs::read_to_string};

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

fn challenge2(path: &str) -> usize {
    let lines: Vec<Vec<char>> = read_to_string(path)
        .expect("File is missing!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut oxygen_idx = (0..lines.len()).map(|x| x).collect::<Vec<usize>>();
    let mut co2_indexes = (0..lines.len()).map(|x| x).collect::<Vec<usize>>();
    for (current, _) in lines[0].iter().enumerate() {
        if oxygen_idx.len() > 1 {
            let mut oxygen_0s = Vec::<usize>::new();
            let mut oxygen_1s = Vec::<usize>::new();
            oxygen_idx.iter().for_each(|&idx| {
                if lines[idx][current] == '1' {
                    oxygen_1s.push(idx);
                } else {
                    oxygen_0s.push(idx);
                }
            });
            if oxygen_1s.len() >= oxygen_0s.len() {
                oxygen_idx = oxygen_1s
            } else {
                oxygen_idx = oxygen_0s
            }
        }
        if co2_indexes.len() > 1 {
            let mut co2_0s = Vec::<usize>::new();
            let mut co2_1s = Vec::<usize>::new();
            co2_indexes.iter().for_each(|&idx| {
                if lines[idx][current] == '1' {
                    co2_1s.push(idx);
                } else {
                    co2_0s.push(idx);
                }
            });
            if co2_1s.len() >= co2_0s.len() {
                co2_indexes = co2_0s
            } else {
                co2_indexes = co2_1s
            }
        }
    }
    let ox = lines[oxygen_idx[0]].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });
    let co2 = lines[co2_indexes[0]].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });
    ox * co2
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
