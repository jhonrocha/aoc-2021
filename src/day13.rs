use std::{collections::HashSet, fs, str::FromStr};

enum Fold {
    Y(u32),
    X(u32),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("fold along ", "");
        let mut v = s.split(',').collect::<Vec<&str>>();
        match v.pop().unwrap() {
            "y" => Ok(Fold::Y(v.pop().unwrap().parse().unwrap())),
            "x" => Ok(Fold::X(v.pop().unwrap().parse().unwrap())),
            _ => Err("Wrong Value".to_string()),
        }
    }
}

fn part1(path: &str) -> u32 {
    let mut file: Vec<&str> = fs::read_to_string(path)
        .expect("missing file")
        .split("\n\n")
        .collect();

    let set = file
        .pop()
        .unwrap()
        .lines()
        .fold(HashSet::<(u32, u32)>::new(), |mut set, line| {
            let mut v = line
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<u32>>();
            set.insert((v.pop().unwrap(), v.pop().unwrap()));
            set
        });
    file.pop()
        .unwrap()
        .lines()
        .fold(set, |set, v| match v.parse::<Fold>().unwrap() {
            Fold::Y(split) => set,
            Fold::X(split) => set,
        });
    println!("{:?}", set);
    0
}

// fn part2(path: &str) -> u32 {}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day13.txt"));
    // println!("Results for Part2 are: {:?}", part2("fixtures/day13.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day13-test.txt"), 10);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2("fixtures/day13-test.txt"), 36);
    }
}
