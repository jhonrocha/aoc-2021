use std::{error::Error, fs::read_to_string, str::FromStr};

pub fn read_lines<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
{
    Ok(read_to_string(path)?
        .lines()
        .filter_map(|l| l.parse::<T>().ok())
        .collect::<Vec<T>>())
}

pub fn read_lines_chars<T, F>(path: &str, f: F) -> Result<Vec<Vec<T>>, Box<dyn Error>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>,
{
    Ok(read_to_string(path)?
        .lines()
        .map(|l| l.chars().filter_map(|c| f(c)).collect())
        .collect::<Vec<Vec<T>>>())
}
