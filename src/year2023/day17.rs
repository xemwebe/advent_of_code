use std::{
    fs::File, io,
    collections::BinaryHeap,
    hash::Hash,
    cmp::Ordering
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn from_num(n: u8) -> Self {
        match n {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("invalid direction")
        }
    }

    fn as_idx(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3
        }
    }

    fn next_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Up => (x-1, y),
            Down => (x+1, y),
            Right => (x, y+1),
            Left => (x, y-1)
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Right => Down,
            Up => Right,
            Left => Up,
            Down => Left,
        }
    }

}

use Direction::*;

#[derive(Debug, Clone, Copy)]
struct NodeInfo {
    visited: bool,
    cost: u64,
}

impl NodeInfo {
    fn new() -> Self {
        Self{ visited: false, cost: 0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    dir: Direction,
    count: i32,
    x: usize,
    y: usize
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x != other.x {
            self.x.cmp(&other.x)
        } else if self.y != other.y {
            self.y.cmp(&other.y)
        } else if self.count != other.count {
            self.count.cmp(&other.count)
        } else {
            self.dir.as_idx().cmp(&other.dir.as_idx())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CostlyNode {
    node: Node,
    cost: u64,
}

impl PartialOrd for CostlyNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CostlyNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost != other.cost {
            other.cost.cmp(&self.cost)
        } else {
            self.node.cmp(&other.node)
        }
    }
}

#[derive(Debug)]
struct Solver {
    nodes: Vec<Vec<Vec<[NodeInfo; 4]>>>,
    map: Vec<Vec<u8>>,
    x_len: usize,
    y_len: usize,
    current: Node,
    queue: BinaryHeap<CostlyNode>,
    min_turn: i32,
    max_straight: i32,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>, max_straight: i32, min_turn: i32) -> Self {
        let x = map.len();
        let y = map[0].len();
        let current =  Node{ dir: Right, count: 0, x: 0, y: 0};
        let mut nodes = vec![vec![vec![[NodeInfo::new(); 4]; max_straight as usize]; y]; x];
        Self {
            nodes,
            map,
            x_len: x,
            y_len: y,
            current,
            queue: BinaryHeap::new(),
            max_straight,
            min_turn,
        }
    }

    fn get_next_state(&self, node: &Node, dir: Direction) -> Option<Node> {
        if (dir == Up && node.x == 0)
        || (dir == Down && node.x == self.x_len-1)
        || (dir == Left && node.y == 0)
        || (dir == Right && node.y == self.y_len-1) {
            return None;
        }
        let (x,y) = dir.next_pos(node.x, node.y);
        if dir == node.dir {
            Some(Node{ dir, count: node.count+1, x, y })           
        } else {
            Some(Node{ dir, count: 1, x, y })           
        }
    }

    fn min_cost(&self) -> u64 {
        let mut min_cost = u64::MAX;
        for i in self.min_turn..=self.max_straight {
            for j in 0..4 {
                min_cost = min_cost.min(
                    self.nodes[self.x_len-1][self.y_len-1][(i-1) as usize][j].cost
                );
            }
        }
        min_cost
    }

    fn all_final_nodes_visited(&self) -> bool {
        for i in self.min_turn..=self.max_straight {
            for j in 0..4 {
                if !self.nodes[self.x_len-1][self.y_len-1][(i-1) as usize][j].visited {
                    return false;
            }
        }
        true
    }

    fn update_node(&mut self, node: &Node, dir: Direction) {
        if let Some(new_node) = self.get_next_state(&node, dir) {
            let node_info = &self.nodes[new_node.x][new_node.y][new_node.count-1][new_node.dir.as_idx()];
            if !node_info.visited {
                    let new_cost = self.nodes[node.x][node.y][node.count-1][node.dir.as_idx()].cost
                        + self.map[new_node.x][new_node.y] as u64;
                    if node_info.cost > new_cost {
                        let node_info = self.nodes.get_mut(&new_node).unwrap();
                        node_info.cost = new_cost;
                    }  
                    self.queue.insert(CostlyNode new_node);
                }
            }
        }
    }

    fn find_path(&mut self) -> u64 {
        loop {
            let node = self.current.clone();
            if node.count < self.max_straight {
                self.update_node(&node, node.dir.clone());
            }
            if node.count >= self.min_turn {
                self.update_node(&node, node.dir.clone().turn_left());

                self.update_node(&node, node.dir.clone().turn_right());
            }

            self.queue.remove(&self.current);
            let node_info = self.nodes.get_mut(&self.current).unwrap();
            node_info.visited = true;

            // find unvisited node in queue with smallest cost
            let mut min_cost = u64::MAX;
            for node in &self.queue {
                if min_cost > self.nodes[node].cost {
                    min_cost = self.nodes[node].cost;
                    self.current = node.clone();
                }
            }

            if self.queue.is_empty() || self.all_final_nodes_visited() {
                break;
            }
        }
        if !self.all_final_nodes_visited() {
            println!("Warning: not all final nodes have been visited!");
        }
        self.min_cost()
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().iter().map(|b| b-b'0').collect());
    }
    let mut solver = Solver::new(map, 3, 0);
    let cost = solver.find_path();
    format!("{cost}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().iter().map(|b| b-b'0').collect());
    }
    let mut solver = Solver::new(map, 10, 4);
    let cost = solver.find_path();
    format!("{cost}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_17_1() {
        let lines = read_lines("data/2023/17.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "698");
    }

    #[test]
    fn test_2023_17_2() {
        let lines = read_lines("data/2023/17.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "8148");
    }
}
