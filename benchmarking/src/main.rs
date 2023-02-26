use std::{
    env,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

// Read a file and parse each line to a Point{X, Y}
#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split(" ").collect();
        Ok(Point {
            x: values[0].parse::<isize>().unwrap(),
            y: values[1].parse::<isize>().unwrap(),
        })
    }
}

fn read_to_memory() -> Vec<Point> {
    read_to_string("input.txt")
        .expect("input.txt not found")
        .lines()
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<Point>>()
}

fn read_to_buf() -> Vec<Point> {
    let input = File::open("input.txt");
    let reader = BufReader::new(input.unwrap());
    let mut points = Vec::<Point>::new();
    for line in reader.lines() {
        points.push(line.unwrap().parse::<Point>().unwrap())
    }
    points
}

fn main() {
    let bench = env::var("READ_STRATEGY").expect("set READ_STRATEGY");
    if bench == "MEMORY" {
        // println!("{:?}", read_to_memory());
        read_to_memory();
    } else {
        // println!("{:?}", read_to_buf());
        read_to_buf();
    }
}
