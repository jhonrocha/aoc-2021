use std::str::FromStr;

use aoc::read_lines_chars;

#[derive(Debug, PartialEq, Clone)]
enum Signal {
    OpenBracket,
    OpenSquare,
    OpenCurly,
    OpenAngle,
    CloseBracket,
    CloseSquare,
    CloseCurly,
    CloseAngle,
}

impl Signal {
    fn match_open(&self) -> Option<Self> {
        match self {
            Signal::CloseBracket => Some(Signal::OpenBracket),
            Signal::CloseSquare => Some(Signal::OpenSquare),
            Signal::CloseCurly => Some(Signal::OpenCurly),
            Signal::CloseAngle => Some(Signal::OpenAngle),
            _ => None,
        }
    }

    fn get_value(&self) -> usize {
        match self {
            Signal::CloseBracket => 3,
            Signal::CloseSquare => 57,
            Signal::CloseCurly => 1197,
            Signal::CloseAngle => 25137,
            Signal::OpenBracket => 1,
            Signal::OpenSquare => 2,
            Signal::OpenCurly => 3,
            Signal::OpenAngle => 4,
        }
    }
}

impl FromStr for Signal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Signal::OpenBracket),
            "[" => Ok(Signal::OpenSquare),
            "{" => Ok(Signal::OpenCurly),
            "<" => Ok(Signal::OpenAngle),
            ")" => Ok(Signal::CloseBracket),
            "]" => Ok(Signal::CloseSquare),
            "}" => Ok(Signal::CloseCurly),
            ">" => Ok(Signal::CloseAngle),
            _ => Err("Wrong Signal".to_string()),
        }
    }
}

enum Diagnostic {
    Corrupted(Signal),
    Incomplete(Vec<Signal>),
    Correct,
}

fn check_line(line: &[Signal]) -> Diagnostic {
    let mut queue = Vec::<Signal>::new();
    let corrupt = line.iter().find(|&sig| {
        if let Some(open) = sig.match_open() {
            if !queue.is_empty() {
                open != queue.pop().expect("Must be a Signal")
            } else {
                true
            }
        } else {
            queue.push(sig.clone());
            false
        }
    });
    if let Some(value) = corrupt {
        Diagnostic::Corrupted(value.clone())
    } else if !queue.is_empty() {
        Diagnostic::Incomplete(queue)
    } else {
        Diagnostic::Correct
    }
}

fn part1(path: &str) -> usize {
    read_lines_chars(path, |c| c.to_string().parse::<Signal>().ok())
        .unwrap()
        .iter()
        .fold(0, |acc, line| match check_line(line) {
            Diagnostic::Corrupted(value) => acc + value.get_value(),
            _ => acc,
        })
}

fn part2(path: &str) -> usize {
    let mut scores = read_lines_chars(path, |c| c.to_string().parse::<Signal>().ok())
        .unwrap()
        .iter()
        .filter_map(|line| match check_line(line) {
            Diagnostic::Incomplete(missing) => {
                Some(
                    missing
                        .iter()
                        .rev()
                        .fold(0, |acc, sig| acc * 5 + sig.get_value()),
                )
            }
            _ => None,
        })
        .collect::<Vec<usize>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day10.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day10.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day10-test.txt"), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day10-test.txt"), 288957);
    }
}
