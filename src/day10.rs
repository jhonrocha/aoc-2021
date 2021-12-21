fn part1(path: &str) -> usize {
    unimplemented!()
}

fn part2(path: &str) -> usize {
    unimplemented!()

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("fixtures/day10-test.txt"), 1134);
    // }
}
