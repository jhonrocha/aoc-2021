use std::collections::HashMap;

use aoc::read_two_splits;

fn set_from_template(pairs: &[String]) -> HashMap<String, u32> {
    let mut map = HashMap::<String, u32>::new();
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

fn apply_step(rules: &HashMap<String, Vec<String>>, pairs: &mut HashMap<String, u32>) {
    let keys = pairs.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        if let Some(targets) = rules.get(&key) {
            targets.iter().for_each(|t| {
                let p = pairs.entry(t.clone()).or_insert(0);
                *p += 1;
                let v = pairs.get_mut(&key).unwrap();
                if *v > 1 {
                    *v -= 1
                } else {
                    pairs.remove(&key);
                }
            });
        }
    }
}

fn part1(path: &str, steps: u32) -> u32 {
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
            let r: Vec<String> = line.split(" -> ").take(1).map(String::from).collect();
            let results = line.replace(" -> ", "");
            let results = vec![results[..2].to_string(), results[1..].to_string()];
            Some((r.first().unwrap().to_owned(), results))
        },
    )
    .unwrap();
    let mut template = set_from_template(template.first().unwrap());
    println!("{:?}", template);
    let rules = set_from_rules(&rules);
    println!("{:?}", rules);
    for _ in 0..steps {
        apply_step(&rules, &mut template);
    }
    println!("{:?}", template);
    0
}

// fn part2(path: &str) -> u32 {
// }

fn main() {
    println!(
        "Results for Part1 are: {:?}",
        part1("fixtures/day14.txt", 1)
    );
    // println!(
    //     "Results for Part2 are: {:?}",
    //     part2("fixtures/day14-test.txt")
    // );
    // println!("Results for Part2 are: {:?}", part2("fixtures/day14.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day14-test.txt", 1), 17);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("fixtures/day14-test.txt"), 16);
    // }
}
