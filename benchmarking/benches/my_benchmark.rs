use std::fs::read_to_string;

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_loops(c: &mut Criterion) {
    let mut group = c.benchmark_group("File Reading");
    let path = "fixtures/day03.txt";
    group.bench_with_input("Single_Loop", path, |b, filename| {
        b.iter(|| single_loop(filename))
    });
    group.bench_with_input("Double_Loop", path, |b, filename| {
        b.iter(|| double_loop(filename))
    });
    group.finish();
}

criterion_group!(benches, bench_loops);
criterion_main!(benches);

pub fn double_loop(path: &str) -> usize {
    let mut oxygen: Vec<Vec<char>> = read_to_string(path)
        .expect("File is missing!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut co2 = oxygen.clone();
    let mut idx = 0;
    while oxygen.len() > 1 {
        let mut oxyg1 = Vec::new();
        let mut oxyg0 = Vec::new();
        for line in oxygen {
            if line[idx] == '1' {
                oxyg1.push(line);
            } else {
                oxyg0.push(line);
            }
        }
        if oxyg1.len() >= oxyg0.len() {
            oxygen = oxyg1;
        } else {
            oxygen = oxyg0;
        }
        idx += 1;
    }
    let oxygen = oxygen[0].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });

    let mut idx = 0;
    while co2.len() > 1 {
        let mut c1 = Vec::new();
        let mut c0 = Vec::new();
        for line in co2 {
            if line[idx] == '1' {
                c1.push(line);
            } else {
                c0.push(line);
            }
        }
        if c1.len() < c0.len() {
            co2 = c1;
        } else {
            co2 = c0;
        }
        idx += 1;
    }

    let co2 = co2[0].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });
    co2 * oxygen
}

fn single_loop(path: &str) -> usize {
    let lines: Vec<Vec<char>> = read_to_string(path)
        .expect("File is missing!")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut oxygen_idx = (0..lines.len()).map(|x| x).collect::<Vec<usize>>();
    let mut co2_indexes = (0..lines.len()).map(|x| x).collect::<Vec<usize>>();
    for (current, _) in lines[0].iter().enumerate() {
        if oxygen_idx.len() > 1 {
            let mut oxygen_0s = Vec::<usize>::new();
            let mut oxygen_1s = Vec::<usize>::new();
            oxygen_idx.iter().for_each(|&idx| {
                if lines[idx][current] == '1' {
                    oxygen_1s.push(idx);
                } else {
                    oxygen_0s.push(idx);
                }
            });
            if oxygen_1s.len() >= oxygen_0s.len() {
                oxygen_idx = oxygen_1s
            } else {
                oxygen_idx = oxygen_0s
            }
        }
        if co2_indexes.len() > 1 {
            let mut co2_0s = Vec::<usize>::new();
            let mut co2_1s = Vec::<usize>::new();
            co2_indexes.iter().for_each(|&idx| {
                if lines[idx][current] == '1' {
                    co2_1s.push(idx);
                } else {
                    co2_0s.push(idx);
                }
            });
            if co2_1s.len() >= co2_0s.len() {
                co2_indexes = co2_0s
            } else {
                co2_indexes = co2_1s
            }
        }
    }
    let ox = lines[oxygen_idx[0]].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });
    let co2 = lines[co2_indexes[0]].iter().fold(0, |mut acc, &c| {
        acc = (acc << 1) + if c == '1' { 1 } else { 0 };
        acc
    });
    ox * co2
}
