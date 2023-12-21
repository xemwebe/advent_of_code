use std::{
    fs::File, io,
    hash::Hash,
    cmp::Ordering
};
use priority_queue::PriorityQueue;

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
        Self{ visited: false, cost: u64::MAX }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Node {
    dir: Direction,
    count: i32,
    x: usize,
    y: usize
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Priority {
    cost: u64
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug)]
struct Solver {
    nodes: Vec<Vec<Vec<[NodeInfo; 4]>>>,
    map: Vec<Vec<u8>>,
    x_len: usize,
    y_len: usize,
    queue: PriorityQueue<Node, Priority>,
    min_turn: i32,
    max_straight: i32,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>, max_straight: i32, min_turn: i32) -> Self {
        let x = map.len();
        let y = map[0].len();
        let nodes = vec![vec![vec![[NodeInfo::new(); 4]; max_straight as usize]; y]; x];
        Self {
            nodes,
            map,
            x_len: x,
            y_len: y,
            queue: PriorityQueue::new(),
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
        }
        true
    }

    fn update_node(&mut self, node: &Node, dir: Direction, cost: u64) {
        if let Some(new_node) = self.get_next_state(&node, dir) {
            let node_info = &self.nodes[new_node.x][new_node.y][(new_node.count-1) as usize][new_node.dir.as_idx()];
            if !node_info.visited {
                let new_cost = cost + self.map[new_node.x][new_node.y] as u64;
                if node_info.cost > new_cost {
                    self.nodes[new_node.x][new_node.y][(new_node.count-1) as usize][new_node.dir.as_idx()].cost = new_cost;
                }  
                self.queue.push_increase(new_node, Priority{cost: new_cost});
            }
        }
    }

    fn find_path(&mut self) -> u64 {
        self.queue.push(Node { dir: Right, count: 1, x: 0, y: 1}, Priority{ cost: self.map[0][1] as u64});
        self.queue.push(Node { dir: Down, count: 1, x: 1, y: 0}, Priority{ cost: self.map[1][0] as u64});
        loop {
            let (node, prio) = self.queue.pop().unwrap();
            //println!("node: {:?}, prio {}", node, prio.cost);
            if node.count < self.max_straight {
                self.update_node(&node, node.dir.clone(), prio.cost);
            }
            if node.count >= self.min_turn {
                self.update_node(&node, node.dir.clone().turn_left(), prio.cost);

                self.update_node(&node, node.dir.clone().turn_right(), prio.cost);
            }

            self.nodes[node.x][node.y][(node.count-1) as usize][node.dir.as_idx()].visited = true;

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
    let mut solver = Solver::new(map, 3, 1);
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
        assert_eq!(result, "825");
    }
}
