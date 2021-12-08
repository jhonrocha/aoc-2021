use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_board(mut board: Vec<Num>, key: &str, line_size: usize) -> bool {
    for (idx, Num (k, f)) in board.iter_mut().enumerate() {
        if k == key {
            *f = true;
        }

    }
    false
}

struct Num(String, bool);

pub fn part1(path: &str) -> usize {
    let file = File::open(path).expect(&format!("Could not open file {}", path));
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first_line = lines.next().expect("Could not read any line").unwrap();
    println!("{:?}", first_line);

    let mut boards: Vec<Vec<Num>> = Vec::new();
    let mut board = Vec::<Num>::new();
    let mut line_size = 0;
    for line in lines {
        line_size = 0;
        let line = line.expect("Could not read a line");
        if line.len() == 0 {
            boards.push(board);
            board = Vec::<Num>::new();
        } else {
            line.split_whitespace().for_each(|key| {
                println!("{}", key);
                line_size += 1;
                board.push(Num(key.to_string(), false));
            });
        }
    }
    first_line.split_whitespace().find(|k| {
        if let Some(winner) = boards.iter_mut().find(|b| check_board(**b, *k, line_size)) {
            true
        } else {
            false
        }
    });

    0
}

pub fn part2(path: &str) -> usize {
    0
}

#[cfg(test)]
mod anything {
    // use super::{part1, part2};
    use super::*;
    #[test]
    // #[ignore]
    fn test_part1() {
        assert_eq!(part1("./src/day4/test.txt"), 198);
    }

    #[test]
    #[ignore]
    fn run_part1() {
        println!("Part1 result is {}", part1("./src/day4/input.txt"))
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2("./src/day4/test.txt"), 230);
    }

    #[test]
    #[ignore]
    fn run_part2() {
        println!("Part2 result is {}", part2("./src/day4/input.txt"))
    }
}
