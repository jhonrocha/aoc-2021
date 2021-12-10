use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn check_board(mut board: Vec<Num>, key: &str, line_size: usize) -> bool {
    for (idx, Num(k, f)) in board.iter_mut().enumerate() {
        if k == key {
            *f = true;
        }
    }
    false
}

struct Num(String, bool);
struct Board {
    houses: Vec<Num>,
    dimensions: (usize, usize),
}

impl Board {
    // add code here
    fn new() -> Board {
        Board {
            houses: Vec::<Num>::new(),
            dimensions: (0, 0),
        }
    }
}

pub fn part1(path: &str) -> usize {
    let file = File::open(path).expect(&format!("Could not open file {}", path));
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let first_line = lines.next().expect("Could not read any line").unwrap();

    let mut b_set: Vec<Board> = Vec::new();
    let mut board = Board::new();
    let mut count_columns = 0;
    let mut count_lines = 0;
    for line in lines {
        let line = line.expect("Could not read a line");
        if line.len() == 0 {
            if count_columns > 0 {
                board.dimensions = (count_lines, count_columns);
                b_set.push(board);
                board = Board::new();
                count_lines = 0;
            }
        } else {
            count_lines += 1;
            count_columns = 0;
            line.split_whitespace().for_each(|key| {
                count_columns += 1;
                board.houses.push(Num(key.to_string(), false));
            });
        }
    }
    first_line.split(",").find(|&k| {
        for board in b_set {
            let (lines, columns) = board.dimensions;
            let mut lines_check = vec![true; lines];
            let mut columns_check = vec![true; columns];
            for (idx, &mut Num(key, found)) in board.houses.iter_mut().enumerate() {
                println!("------------>{:?}: {:?}", key, found);
                if k == key {
                    found = true;
                    println!("K: {:?}, key: {:?}, found: {:?}", k, key, found);
                }
                lines_check[idx % lines] = found && lines_check[idx % lines];
                columns_check[idx / columns] = found && columns_check[idx % columns];
            }
            if let Some(found) = lines_check.iter().find(|&&l| !l) {
                return true;
            }
            println!("Lines: {:?}", lines_check);
            println!("Columns: {:?}", columns_check);
            false
        }
        false
    });
    198
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
