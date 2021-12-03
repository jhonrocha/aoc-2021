use std::fs;

pub fn challenge1() {
    let content = fs::read_to_string("./day1-1.txt").expect("File not found");
    let mut entries: [isize; 3] = [0, 0, 0];
    let mut counter: isize = 0;
    for (index, line) in content.lines().enumerate() {
        let value: isize = line.parse().expect("Line is not a number");
        if (index > 3) && (value > entries[0]) {
            counter += 1;
        }
        entries[0] = entries[1];
        entries[1] = entries[2];
        entries[2] = value;
    }
    println!("{}", counter);
}

pub fn challenge2() {
    let content = fs::read_to_string("./day1-1.txt").expect("File not found");
    let mut entries: [isize; 3] = [0, 0, 0];
    let mut counter: isize = 0;
    for (index, line) in content.lines().enumerate() {
        let value: isize = line.parse().expect("Line is not a number");
        if (index > 3) && (value > entries[0]) {
            counter += 1;
        }
        entries[0] = entries[1];
        entries[1] = entries[2];
        entries[2] = value;
    }
    println!("{}", counter);
}
