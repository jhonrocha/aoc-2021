use aoc::read_lines_split;

fn part1(path: &str) -> u32 {
    let a = "a";
    let lines: Vec<Vec<u32>> = read_lines_split(path, |line| {
        Some(line.split(',').map(|f| f.parse::<u32>().unwrap()).collect())
    })
    .unwrap();
    let min = *lines[0].iter().min().unwrap();
    let max = *lines[0].iter().max().unwrap();
    (min..max)
        .map(|pos| {
            lines[0]
                .iter()
                .fold(0, |acc, &fish| acc + (fish as i32 - pos as i32).abs())
        })
        .min()
        .unwrap() as u32
}

fn part2(path: &str) -> u64 {
    0
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day7.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day7.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day7-test.txt"), 37);
    }

    // #[test]
    // fn test_part2() {
    // }
}
