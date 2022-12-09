use super::*;
use std::collections::HashSet;

fn parse_move(s: String) -> (String, i32) {
    let mut parts = s.split(" ");
    (
        parts.next().unwrap().to_owned(),
        parts.next().unwrap().parse().unwrap(),
    )
}

struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0,0));
        Self {
            head: (0,0),
            tail: (0,0),
            visited,
        }
    }

    fn make_move(&mut self, m: &(String, i32)) {
        let step = match m.0.as_str() {
            "R" => (1,0),
            "L" => (-1,0),
            "D" => (0,-1),
            "U" => (0,1),
            _ => panic!("invalid move")
        };
        for _ in 0..m.1 {
            self.head.0 += step.0;
            self.head.1 += step.1;
            self.move_tail()
        }
    }

    fn move_tail(&mut self) {
        let mut dist = (self.head.0-self.tail.0, self.head.1-self.tail.1);
        if (dist.0.abs() + dist.1.abs() <= 1) 
        || (dist.0.abs()==1 && dist.1.abs() == 1) {
            return;
        }
        if dist.0.abs() == 2 {
            dist.0 /= 2;
        }
        if dist.1.abs() == 2 {
            dist.1 /= 2;
        }
        self.tail = (self.tail.0 + dist.0, self.tail.1 + dist.1);
        self.visited.insert(self.tail);
    }

    fn count(&self) -> usize {
        self.visited.len()
    }
}
pub fn riddle_9_1(lines: io::Lines<io::BufReader<File>>) {
    let moves: Vec<(String, i32)> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_move)
        .collect();

    let mut rope = Rope::new();
    for m in moves {
        rope.make_move(&m);
    }

    println!("{:?}", rope.count());
}

