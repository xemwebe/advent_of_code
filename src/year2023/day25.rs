use std::{
    collections::{HashMap, HashSet, BTreeSet},
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
    connections: BTreeSet<Connection>,
}

impl Solver {
    fn new(nodes: HashMap<String, usize>, connections: BTreeSet<Connection>) -> Self {
        Self{
            nodes,
            connections,
        }
    }

    fn find_connected_groups(&self, exclusions: &[Connection;3]) -> Vec<usize> {
        let mut connections = self.connections.clone();
        for e in exclusions {
            connections.remove(e);
        }

        let mut group_sizes = Vec::new();
        while !connections.is_empty() {
            let mut group = HashSet::new();
            let p = connections.pop_first().unwrap();
            group.insert(p.a);
            group.insert(p.b);
            let mut found_pair = true;
            let mut found_pairs = HashSet::new();
            while found_pair {
                found_pair = false;
                for n in &group {
                    for p in &connections {
                        if p.a == *n || p.b == *n {
                            found_pair = true;
                            found_pairs.insert(p.clone());
                        }
                    }
                }
                for p in &found_pairs {
                    group.insert(p.a);
                    group.insert(p.b);
                    connections.remove(&p);
                }
            }
            group_sizes.push(group.len());
        }
        group_sizes
    }

    fn solve(&self) -> usize  {
        let c = Connection::new(0,0);
        let mut exclusions = [c.clone(),c.clone(),c];
        for c1 in &self.connections  {
            exclusions[0] = c1.clone();
            for c2 in &self.connections {
                if c1==c2 {
                    continue;
                }
                exclusions[1] = c2.clone();
                for c3 in &self.connections {
                    if c3==c1 || c3==c2 {
                        continue;
                    }
                    exclusions[2] = c3.clone();
                    let group_sizes = self.find_connected_groups(&exclusions);
                    if group_sizes.len() == 2 {
                        return group_sizes.iter().product();
                    }
                }
            }
        }
        0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Connection {
    a: usize,
    b: usize,
}

impl Connection {
    fn new(a: usize, b: usize) -> Self {
        Self { a: a.min(b), b: a.max(b) }
    }
}
impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.a==other.a {
            return self.b.cmp(&other.b);
        }
        self.a.cmp(&other.a)
    }
}

fn parse_input(lines: io::Lines<io::BufReader<File>>) -> Solver {
    let mut nodes = HashMap::new();
    let mut connections = BTreeSet::new();
    let mut idx = 0;
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(": ").collect();
        let links: Vec<&str> = parts[1].split(' ').collect();
        if !nodes.contains_key(parts[0]) {
            nodes.insert(parts[0].to_string(), idx);
            idx += 1;
        }
        let n_idx = nodes[parts[0]];
        for s in links {
            if !nodes.contains_key(s) {
                nodes.insert(s.to_string(), idx);
                idx += 1;
            }
            let l_idx = nodes[s];
            connections.insert(Connection::new(n_idx, l_idx));
        }

    }
    Solver::new(nodes, connections)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines);
    println!("{solver:?}");
    let solution: usize = solver.solve();
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines);
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
