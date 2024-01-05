use std::{
    collections::HashMap,
    fs::File,
    io,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Solver {
    nodes: HashMap<String, usize>,
    connections: Vec<Vec<usize>>,
}

impl Solver {
    fn new(nodes: HashMap<String, usize>, connections: Vec<Vec<usize>>) -> Self {
        Self{
            nodes,
            connections,
        }
    }
}

fn parse_input(lines: io::Lines<io::BufReader<File>>, avoid_slippery: bool) -> Solver {
    let mut nodes = HashMap::new();
    let mut connections = Vec::new();
    let mut idx = 0;
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(": ").collect();
        let links: Vec<&str> = parts[0].split(' ').collect();
        if !nodes.contains_key(parts[0]) {
            nodes.insert(parts[0].to_string(), idx);
            connections.push(Vec::new());
            idx += 1;
        }
        let n_idx = nodes[parts[0]];
        for s in links {
            if !nodes.contains_key(s) {
                nodes.insert(s.to_string(), idx);
                connections.push(Vec::new());
                idx += 1;
            }
            let l_idx = nodes[s];
            connections[n_idx].push(l_idx);
            connections[l_idx].push(n_idx);
        }

    }
    Solver::new(nodes, connections)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines, true);
    println!("{solver:?}");
    let solution = 0;
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    format!("not yet implemented")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_24_1() {
        let lines = read_lines("data/2023/24.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "21785");
    }

    #[test]
    fn test_2023_24_2() {
        let lines = read_lines("data/2023/24.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "6522");
    }
}
