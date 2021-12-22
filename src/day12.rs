use aoc::read_lines_split;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Vertex {
    Upper(String),
    Lower(String),
}

type Graph = HashMap<Vertex, Vec<Vertex>>;

fn line_to_vertex(line: &str) -> Option<Vec<Vertex>> {
    Some(
        line.split('-')
            .filter_map(|v| {
                if v.chars().next()?.is_uppercase() {
                    Some(Vertex::Upper(v.to_string()))
                } else {
                    Some(Vertex::Lower(v.to_string()))
                }
            })
            .collect::<Vec<Vertex>>(),
    )
}

fn graph_insert(mut graph: Graph, mut vert: Vec<Vertex>) -> Graph {
    let dest = vert.pop().unwrap();
    let origin = vert.pop().unwrap();
    // Origin -> Dest
    let node = graph
        .entry(origin.clone())
        .or_insert_with(Vec::<Vertex>::new);
    node.push(dest.clone());
    // Dest -> Origin
    let node = graph.entry(dest).or_insert_with(Vec::<Vertex>::new);
    node.push(origin);
    graph
}

#[allow(dead_code)]
fn graph_dfs_rec(graph: &Graph, from: Vertex, end: Vertex, visited: HashSet<Vertex>) -> u32 {
    if from == end {
        return 1;
    }
    let mut count = 0;
    for dest in graph.get(&from).unwrap() {
        let mut visited = visited.clone();
        if let Vertex::Lower(_) = dest {
            if visited.contains(dest) {
                continue;
            } else {
                visited.insert(dest.clone());
            }
        }
        count += graph_dfs_rec(graph, dest.clone(), end.clone(), visited);
    }
    count
}

fn graph_dfs(graph: &Graph, start: Vertex, end: Vertex, twice: bool) -> u32 {
    let mut count = 0;

    // Next paths to check
    let mut explorer_queue = VecDeque::<(Vertex, HashSet<_>, bool)>::new();
    explorer_queue.push_back((start.clone(), vec![start.clone()].into_iter().collect(), false));

    // Check new paths
    while !explorer_queue.is_empty() {
        // Get the vertex to check
        let (from, visited, revisited) = explorer_queue.pop_front().unwrap();
        // Get all destinations for the vertex
        for dest in graph.get(&from).unwrap() {
            // Check if any destination is the desired one;
            if *dest == end {
                count += 1;
                continue;
            } else if *dest == start {
                continue;
            }
            // Clone both values from the queue
            let mut visited = visited.clone();
            let mut rv = revisited;
            // Check for lower vertex
            if let Vertex::Lower(_) = dest {
                // Check if already visited
                if visited.contains(dest) {
                    // If revisiting is allowed and still available
                    if twice && !rv {
                        rv = true;
                    } else {
                        continue;
                    }
                } else {
                    // Add the vertex to the set of visited
                    visited.insert(dest.clone());
                }
            }
            // Push the vertex to the explorer queue
            explorer_queue.push_back((dest.clone(), visited, rv));
        }
    }
    count
}

fn part1(path: &str) -> u32 {
    let graph = read_lines_split(path, line_to_vertex)
        .unwrap()
        .into_iter()
        .fold(Graph::new(), graph_insert);
    // graph_dfs_rec(
    //     &graph,
    //     Vertex::Lower("start".to_string()),
    //     Vertex::Lower(String::from("end")),
    //     vec![Vertex::Lower("start".to_string())]
    //         .into_iter()
    //         .collect(),
    // );
    graph_dfs(
        &graph,
        Vertex::Lower("start".to_string()),
        Vertex::Lower(String::from("end")),
        false,
    )
}

fn part2(path: &str) -> u32 {
    let graph = read_lines_split(path, line_to_vertex)
        .unwrap()
        .into_iter()
        .fold(Graph::new(), graph_insert);
    graph_dfs(
        &graph,
        Vertex::Lower("start".to_string()),
        Vertex::Lower(String::from("end")),
        true,
    )
}

fn main() {
    println!("Results for Part1 are: {:?}", part1("fixtures/day12.txt"));
    println!("Results for Part2 are: {:?}", part2("fixtures/day12.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("fixtures/day12-test.txt"), 10);
        assert_eq!(part1("fixtures/day12-test2.txt"), 19);
        assert_eq!(part1("fixtures/day12-test3.txt"), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("fixtures/day12-test.txt"), 36);
        // assert_eq!(part2("fixtures/day12-test2.txt"), 103);
        // assert_eq!(part2("fixtures/day12-test3.txt"), 3509);
    }
}
