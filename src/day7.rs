use aoc::read_lines_split;

fn sort_crabs_cost(path: &str, fuel: fn(i32, i32) -> i32) -> i32 {
    let lines: Vec<Vec<i32>> = read_lines_split(path, |line| {
        Some(line.split(',').map(|f| f.parse::<i32>().unwrap()).collect())
    })
    .unwrap();
    let min = *lines[0].iter().min().unwrap();
    let max = *lines[0].iter().max().unwrap();
    (min..max)
        .map(|pos| lines[0].iter().fold(0, |acc, &fish| acc + fuel(fish, pos)))
        .min()
        .unwrap() as i32
}

fn part1(path: &str) -> i32 {
    sort_crabs_cost(path, |fish, pos| (fish - pos).abs())
}

fn part2(path: &str) -> i32 {
    sort_crabs_cost(path, |fish, pos| {
        let dist = (fish - pos).abs() as f64;
        ((dist + 1.0) * (dist / 2.0)) as i32
    })
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

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day7-test.txt"), 168);
    }
}
