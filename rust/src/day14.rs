use std::collections::HashMap;

use aoc::read_two_splits;

fn set_from_template(pairs: &[String]) -> HashMap<String, u64> {
    let mut map = HashMap::<String, u64>::new();
    pairs.iter().for_each(|pair| {
        let v = map.entry(pair.clone()).or_insert(0);
        *v += 1;
    });
    map
}

fn set_from_rules(rules: &[(String, Vec<String>)]) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    rules.iter().for_each(|pair| {
        map.entry(pair.0.clone()).or_insert_with(|| pair.1.clone());
    });
    map
}

fn apply_step(rules: &HashMap<String, Vec<String>>, pairs: &mut HashMap<String, u64>) {
    let keys = pairs.clone();
    // Pairs on the hash
    for (key, value) in keys {
        // Targets from the rules for each pair
        if let Some(targets) = rules.get(&key) {
            // For each target
            targets.iter().for_each(|t| {
                // Incr it on the pair list
                let p = pairs.entry(t.clone()).or_insert(0);
                *p += value;
            });
            // Decr the current key
            if let Some(v) = pairs.get_mut(&key) {
                // If key, add it to 1
                if *v > value {
                    *v -= value
                } else {
                    pairs.remove(&key);
                }
            }
        }
    }
}

fn part1(path: &str, steps: u64) -> u64 {
    // Read the file
    let (template, rules) = read_two_splits(
        path,
        "\n\n",
        |line| {
            Some(
                line.chars()
                    .collect::<Vec<char>>()
                    .windows(2)
                    .map(|v| {
                        let mut resp = v[0].to_string();
                        resp.push(v[1]);
                        resp
                    })
                    .collect::<Vec<String>>(),
            )
        },
        |line| {
            let letters: Vec<char> = line.replace(" -> ", "").chars().collect();
            let (c1, c2, c3) = (
                letters.get(0).unwrap(),
                letters.get(1).unwrap(),
                letters.get(2).unwrap(),
            );
            let mut pair = c1.to_string();
            pair.push(*c2);
            let mut comb1 = c1.to_string();
            comb1.push(*c3);
            let mut comb2 = c3.to_string();
            comb2.push(*c2);

            Some((pair, vec![comb1, comb2]))
        },
    )
    .unwrap();
    let mut template = set_from_template(template.first().unwrap());
    let rules = set_from_rules(&rules);
    for _ in 0..steps {
        apply_step(&rules, &mut template);
    }
    let mut max_count = 0;
    let mut min_count = u64::MAX;
    template
        .iter()
        .fold(HashMap::new(), |mut hash, (key, value)| {
            key.chars().for_each(|c| {
                let v = hash.entry(c).or_insert(0);
                *v += value;
            });
            hash
        })
        .values()
        .for_each(|v| {
            let v = if v % 2 == 0 { v / 2 } else { v / 2 + 1 };
            if v > max_count {
                max_count = v
            }
            if v < min_count {
                min_count = v
            }
        });
    max_count - min_count
}

fn main() {
    println!(
        "Results for Part1 are: {:?}",
        part1("fixtures/day14.txt", 10)
    );
    println!(
        "Results for Part2 are: {:?}",
        part1("fixtures/day14.txt", 40)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day14-test.txt", 10), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1("fixtures/day14-test.txt", 40), 2188189693529);
    }
}
