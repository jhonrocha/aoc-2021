use std::fs::read_to_string;

#[allow(dead_code)]
pub fn part1(path: &str) -> u32 {
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

#[allow(dead_code)]
pub fn part2(path: &str) -> u32 {
    let lines = read_to_string(path).expect("File not found");
    let (plays, boards) = lines.split_once("\n").expect("Wrong file");
    let plays: Vec<&str> = plays.split(",").collect();
    let mut boards: Vec<Vec<&str>> = boards
        .split("\n\n")
        .map(|b| b.trim().split_whitespace().collect())
        .collect();
    let mut winner: u32 = 0;
    let mut ignore = Vec::<usize>::new();
    plays.iter().find(|&k| {
        for (n_pos, board) in boards.iter_mut().enumerate() {
            if ignore.iter().find(|&&v| v == n_pos).is_some() {
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
                winner = board.iter().filter_map(|el| el.parse::<u32>().ok()).sum();
                let mult: u32 = k.parse().unwrap();
                winner *= mult;
                ignore.push(n_pos);
            }
        }
        false
    });
    winner
}

#[cfg(test)]
mod anything {
    // use super::{part1, part2};
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day4-test.txt"), 4512);
    }

    #[test]
    fn run_part1() {
        println!("Part1 result is {}", part1("fixtures/day4.txt"))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day4-test.txt"), 1924);
    }

    #[test]
    fn run_part2() {
        println!("Part2 result is {}", part2("fixtures/day4.txt"))
    }
}
