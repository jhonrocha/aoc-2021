use std::fs::read_to_string;

#[allow(dead_code)]
pub fn challenge1(path: &str) -> u32 {
    let lines = read_to_string(path).expect("File not found");
    let (plays, boards) = lines.split_once("\n").expect("Wrong file");
    let plays: Vec<&str> = plays.split(",").collect();
    let mut boards: Vec<Vec<&str>> = boards
        .split("\n\n")
        .map(|b| b.trim().split_whitespace().collect())
        .collect();
    let mut winner: u32 = 0;
    plays.iter().find(|&k| {
        for board in boards.iter_mut() {
            let mut lines = vec![true; 5];
            let mut col = vec![true; 5];
            for (idx, hs) in board.iter_mut().enumerate() {
                if hs == k {
                    *hs = "-";
                }
                lines[idx % 5] = lines[idx % 5] && (*hs == "-");
                col[idx / 5] = col[idx / 5] && (*hs == "-");
            }
            if lines.iter().find(|&l| *l).is_some() || col.iter().find(|&c| *c).is_some() {
                winner = board.iter().filter_map(|el| el.parse::<u32>().ok()).sum();
                let mult: u32 = k.parse().unwrap();
                winner *= mult;
                break;
            }
        }
        winner != 0
    });
    winner
}

pub fn challenge2(path: &str) -> u32 {
    let lines = read_to_string(path).expect("File not found");
    let (plays, boards) = lines.split_once("\n").expect("Wrong file");
    let plays: Vec<&str> = plays.split(",").collect();
    let mut boards: Vec<Vec<&str>> = boards
        .split("\n\n")
        .map(|b| b.trim().split_whitespace().collect())
        .collect();
    let mut winner: u32 = 0;
    let len = boards.len();
    let mut ignore: Vec<bool> = vec![false; len];
    let mut counter = 0;
    plays.iter().find(|&k| {
        for (n_pos, board) in boards.iter_mut().enumerate() {
            if ignore[n_pos] {
                continue;
            }
            let mut lines = vec![true; 5];
            let mut col = vec![true; 5];
            for (idx, hs) in board.iter_mut().enumerate() {
                if hs == k {
                    *hs = "-";
                }
                lines[idx % 5] = lines[idx % 5] && (*hs == "-");
                col[idx / 5] = col[idx / 5] && (*hs == "-");
            }
            if lines.iter().find(|&l| *l).is_some() || col.iter().find(|&c| *c).is_some() {
                ignore[n_pos] = true;
                counter += 1;
                if counter == len {
                    winner = board.iter().filter_map(|el| el.parse::<u32>().ok()).sum();
                    let mult: u32 = k.parse().unwrap();
                    winner *= mult;
                    return true;
                }
            }
        }
        return false;
    });
    winner
}

fn main() {
    println!(
        "Day 03 - Challenge 1 results: {:?}",
        challenge1("fixtures/day04.txt")
    );
    println!(
        "Day 03 - Challenge 2 results: {:?}",
        challenge2("fixtures/day04.txt")
    );
}

#[cfg(test)]
mod anything {
    // use super::{challenge1, challenge2};
    use super::*;
    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("fixtures/day04-test.txt"), 4512);
    }
    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("fixtures/day04-test.txt"), 1924);
    }
}
