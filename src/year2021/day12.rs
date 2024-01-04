use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::{HashMap, HashSet};

fn read_pairs(lines: io::Lines<io::BufReader<File>>) -> Vec<(String, String)> {
    lines
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|s| {
            s.split("-")
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|s| (s[0].clone(), s[1].clone()))
        .collect()
}

#[derive(Debug)]
struct Node {
    name: String,
    is_small: bool,
    connections: Vec<usize>,
}

fn is_lower_case(s: &str) -> bool {
    let zed = "Z".bytes().next().unwrap();
    if s.bytes().next().unwrap() > zed {
        true
    } else {
        false
    }
}

fn prepare_nodes(pairs: Vec<(String, String)>) -> (usize, Vec<Node>) {
    let mut unique_names = HashSet::new();
    for p in pairs.iter() {
        unique_names.insert(p.0.to_owned());
        unique_names.insert(p.1.to_owned());
    }

    let mut nodes: Vec<Node> = unique_names
        .into_iter()
        .map(|x| {
            let is_small = is_lower_case(&x);
            Node {
                name: x,
                is_small,
                connections: Vec::new(),
            }
        })
        .collect();

    let mut start = 0;
    let mut node_map = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if n.name == "start" {
            start = i;
        }
        node_map.insert(n.name.clone(), i);
    }

    for p in pairs {
        let idx1 = node_map[&p.0];
        let idx2 = node_map[&p.1];
        nodes[idx1].connections.push(idx2);
        nodes[idx2].connections.push(idx1);
    }

    (start, nodes)
}

fn find_all_paths(
    current_path: Vec<usize>,
    double_chance: bool,
    nodes: &Vec<Node>,
    visited: HashSet<usize>,
    paths: &mut Vec<Vec<usize>>,
) {
    let idx = current_path[current_path.len() - 1];

    // end of path store and return
    if nodes[idx].name == "end" {
        paths.push(current_path);
        return;
    }
    let mut double_chance = double_chance;
    // exclude small caves already visited
    if visited.contains(&idx) {
        if double_chance == true && nodes[idx].name != "start" {
            double_chance = false;
        } else {
            return;
        }
    }

    let mut visited = visited.clone();
    // remember small caves
    if nodes[idx].is_small {
        visited.insert(idx);
    }

    for i in &nodes[idx].connections {
        let mut new_path = current_path.clone();
        new_path.push(*i);
        find_all_paths(new_path, double_chance, nodes, visited.clone(), paths);
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let pairs = read_pairs(lines);
    let (start, nodes) = prepare_nodes(pairs);

    let mut paths = Vec::new();
    let visited = HashSet::new();
    let start_path = vec![start];
    find_all_paths(start_path, false, &nodes, visited, &mut paths);

    format!("{:?}", paths.len())
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let pairs = read_pairs(lines);
    let (start, nodes) = prepare_nodes(pairs);

    let mut paths = Vec::new();
    let visited = HashSet::new();
    let start_path = vec![start];
    find_all_paths(start_path, true, &nodes, visited, &mut paths);

    format!("{}", paths.len())
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2021_12_1() {
        let lines = read_lines("data/2021/12.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "3497");
    }

    #[test]
    fn test_2021_12_2() {
        let lines = read_lines("data/2021/12.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "93686");
    }
}
