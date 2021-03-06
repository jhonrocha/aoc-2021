use std::collections::VecDeque;

use aoc::read_lines_chars;

fn part1(path: &str, days: u64) -> u64 {
    let lines: Vec<Vec<u64>> = read_lines_chars(path, |c| c.to_digit(10).map(u64::from)).unwrap();
    let resp = (0..days).fold(lines[0].clone(), |l, _| {
        let mut resp = Vec::new();
        l.iter().for_each(|&v| {
            if v == 0 {
                resp.push(6);
                resp.push(8);
            } else {
                resp.push(v - 1);
            }
        });
        resp
    });
    resp.len().try_into().unwrap()
}

// From TJ!
fn part2(path: &str, days: u64) -> u64 {
    let mut count = VecDeque::from(vec![0; 9]);
    let lines: Vec<Vec<u64>> = read_lines_chars(path, |c| c.to_digit(10).map(u64::from)).unwrap();
    lines[0].iter().for_each(|&i| count[i as usize] += 1);
    (0..days).for_each(|_| {
        let born = count.pop_front().unwrap();
        count.push_back(born);
        count[6] += born;
    });
    count.iter().sum()
}

fn main() {
    println!(
        "Results for Part1 are: {:?}",
        part1("fixtures/day6.txt", 80)
    );
    println!(
        "Results for Part2 are: {:?}",
        part2("fixtures/day6.txt", 256)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day6-test.txt", 18), 26);
        assert_eq!(part1("fixtures/day6-test.txt", 80), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day6-test.txt", 18), 26);
        assert_eq!(part2("fixtures/day6-test.txt", 80), 5934);
        assert_eq!(part2("fixtures/day6-test.txt", 256), 26984457539);
    }
}
