use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

enum Digit {
    One,
    Four,
    Seven,
    Eight,
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            2 => Ok(Digit::One),
            4 => Ok(Digit::Four),
            3 => Ok(Digit::Seven),
            7 => Ok(Digit::Eight),
            _ => Err(String::from("Number not valid")),
        }
    }
}

fn read_outputs(path: &str) -> usize {
    // let lines: Vec<Vec<&str>> =
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .flat_map(|line| {
            line.split('|')
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .split_whitespace()
                .filter_map(|digit| digit.parse::<Digit>().ok())
        })
        .count()
}

fn deduce_signals(signals: Vec<&str>) -> HashMap<usize, HashSet<char>> {
    let mut missing = Vec::<HashSet<char>>::new();

    let mut mask: HashMap<usize, HashSet<char>> =
        signals.iter().fold(HashMap::new(), |mut map, signal| {
            match signal.len() {
                2 => map.insert(1, signal.chars().collect::<HashSet<char>>()),
                3 => map.insert(7, signal.chars().collect::<HashSet<char>>()),
                4 => map.insert(4, signal.chars().collect::<HashSet<char>>()),
                7 => map.insert(8, signal.chars().collect::<HashSet<char>>()),
                _ => {
                    missing.push(signal.chars().collect::<HashSet<char>>());
                    None
                }
            };
            map
        });

    for m in missing {
        if m.len() == 6 {
            if m.intersection(mask.get(&4).unwrap()).count() == 4 {
                mask.insert(9, m);
            } else if m.intersection(mask.get(&1).unwrap()).count() == 1 {
                mask.insert(6, m);
            } else {
                mask.insert(0, m);
            }
        } else if m.intersection(mask.get(&1).unwrap()).count() == 2 {
            mask.insert(3, m);
        } else if m.intersection(mask.get(&4).unwrap()).count() == 3 {
            mask.insert(5, m);
        } else {
            mask.insert(2, m);
        }
    }
    mask
}

fn sonar_output(path: &str) -> usize {
    // let lines: Vec<Vec<&str>> =
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        // .flat_map(|line| {
        .map(|line| {
            let entries = line.split('|').collect::<Vec<&str>>();
            let decoder = deduce_signals(entries[0].split_whitespace().collect());
            let resp = entries[1].split_whitespace().fold(0, |mut acc, digit| {
                let d_set: HashSet<char> = digit.chars().collect();
                if let Some(value) = decoder.iter().find(|(_, val)| **val == d_set) {
                    acc = acc * 10 + value.0
                } else {
                }
                acc
            });
            resp
        })
        .sum::<usize>()
}

fn part1(path: &str) -> usize {
    read_outputs(path)
}

fn part2(path: &str) -> usize {
    sonar_output(path)
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day8.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day8.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day8-test.txt"), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day8-test.txt"), 61229);
    }
}
