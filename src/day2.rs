use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(dead_code)]
pub fn challenge1(path: &str) -> isize {
    let f = File::open(path).expect(&format!("Could no open file {}", path));
    let reader = BufReader::new(f);
    // Directions: (horizontal, depth)
    let mut directions: (isize, isize) = (0, 0);
    for l in reader.lines() {
        let line = l.expect("missing line");
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["forward", v] => directions.0 += v.parse::<isize>().unwrap(),
            ["down", v] => directions.1 += v.parse::<isize>().unwrap(),
            ["up", v] => directions.1 -= v.parse::<isize>().unwrap(),
            _ => (),
        }
    }
    directions.0 * directions.1
}

#[allow(dead_code)]
pub fn challenge2(path: &str) -> isize {
    let f = File::open(path).expect(&format!("Could no open file {}", path));
    let reader = BufReader::new(f);
    // Directions: (horizontal, depth, aim)
    let mut directions: (isize, isize, isize) = (0, 0, 0);
    for l in reader.lines() {
        let line = l.expect("missing line");
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["down", v] => directions.2 += v.parse::<isize>().unwrap(),
            ["up", v] => directions.2 -= v.parse::<isize>().unwrap(),
            ["forward", v] => {
                let x = v.parse::<isize>().unwrap();
                directions.0 += x;
                directions.1 += directions.2 * x;
            }
            _ => (),
        };
    }
    directions.0 * directions.1
}

#[cfg(test)]
mod day2test {
    use super::challenge1;
    use super::challenge2;

    #[test]
    fn test_challenge1() {
        assert_eq!(challenge1("./src/day2/test-input.txt"), 150);
    }

    #[test]
    fn run_challenge1() {
        let resp = challenge1("./src/day2/input.txt");
        println!("Day 2 - Challenge1 Resp: {}", resp);
    }

    #[test]
    fn test_challenge2() {
        assert_eq!(challenge2("./src/day2/test-input.txt"), 900);
    }

    #[test]
    fn run_challenge2() {
        let resp = challenge2("./src/day2/input.txt");
        println!("Day 2 - Challenge2 Resp: {}", resp);
    }
}
