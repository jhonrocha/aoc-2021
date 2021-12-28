use std::{collections::VecDeque, error::Error, fmt::Debug, fs::read_to_string, str::FromStr};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

pub fn cardinal_dirs(p: Point, lines: usize, columns: usize) -> Vec<Point> {
    let mut coords = Vec::<Point>::new();
    if p.x > 0 {
        coords.push(Point::new(p.x - 1, p.y));
    }
    if p.x < (lines - 1) {
        coords.push(Point::new(p.x + 1, p.y));
    }
    if p.y > 0 {
        coords.push(Point::new(p.x, p.y - 1));
    }
    if p.y < (columns - 1) {
        coords.push(Point::new(p.x, p.y + 1));
    }
    coords
}

pub fn read_lines<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
{
    Ok(read_to_string(path)?
        .lines()
        .filter_map(|l| l.parse::<T>().ok())
        .collect::<Vec<T>>())
}

/// Apply filter_map each char of every line
///
/// # Arguments
///
/// * `path` - The file path
/// * `f` - A function to parse each char returning an Option
///
/// # Examples
///
/// ```
/// use aoc::read_lines_chars;
/// let lines = read_lines_chars(path, |c| c.to_digit(10)).unwrap();
/// ```
pub fn read_lines_chars<T, F>(path: &str, f: F) -> Result<Vec<Vec<T>>, Box<dyn Error>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    Ok(read_to_string(path)?
        .lines()
        .map(|l| l.chars().filter_map(&f).collect())
        .collect::<Vec<Vec<T>>>())
}

pub fn read_queue_chars<T, F>(path: &str, f: F) -> Result<Vec<VecDeque<T>>, Box<dyn Error>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    Ok(read_to_string(path)?
        .lines()
        .map(|l| l.chars().filter_map(&f).collect())
        .collect::<Vec<VecDeque<T>>>())
}

pub fn read_lines_split<T, F>(path: &str, f: F) -> Result<Vec<Vec<T>>, Box<dyn Error>>
where
    F: Fn(&str) -> Option<Vec<T>>,
{
    Ok(read_to_string(path)?
        .lines()
        .filter_map(f)
        .collect::<Vec<Vec<T>>>())
}

pub fn read_two_splits<T, K, F, S>(
    path: &str,
    split: &str,
    f1: F,
    f2: S,
) -> Result<(Vec<T>, Vec<K>), Box<dyn Error>>
where
    F: Fn(&str) -> Option<T>,
    S: Fn(&str) -> Option<K>,
{
    // Read the file
    let file = read_to_string(path).expect("missing file");
    let parts: Vec<&str> = file.split(split).collect();
    Ok((
        parts.first().unwrap().lines().filter_map(f1).collect(),
        parts.last().unwrap().lines().filter_map(f2).collect(),
    ))
}

pub fn print_iter<T>(it: T)
where
    T: std::iter::Iterator,
    <T as Iterator>::Item: Debug,
{
    it.for_each(|v| println!("{:?}", v));
}
