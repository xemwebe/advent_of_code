use std::collections::HashSet;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn parse_move(s: String) -> (String, i32) {
    let mut parts = s.split(" ");
    (
        parts.next().unwrap().to_owned(),
        parts.next().unwrap().parse().unwrap(),
    )
}

struct Rope {
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(length: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        let mut knots = Vec::new();
        for _ in 0..length {
            knots.push((0, 0));
        }
        Self { knots, visited }
    }

    fn make_move(&mut self, m: &(String, i32)) {
        let step = match m.0.as_str() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "D" => (0, -1),
            "U" => (0, 1),
            _ => panic!("invalid move"),
        };
        for _ in 0..m.1 {
            self.knots[0].0 += step.0;
            self.knots[0].1 += step.1;
            self.move_tail()
        }
    }

    fn move_tail(&mut self) {
        for i in 1..self.knots.len() {
            let mut dist = (
                self.knots[i - 1].0 - self.knots[i].0,
                self.knots[i - 1].1 - self.knots[i].1,
            );
            if (dist.0.abs() + dist.1.abs() <= 1) || (dist.0.abs() == 1 && dist.1.abs() == 1) {
                return;
            }
            if dist.0.abs() == 2 {
                dist.0 /= 2;
            }
            if dist.1.abs() == 2 {
                dist.1 /= 2;
            }
            self.knots[i] = (self.knots[i].0 + dist.0, self.knots[i].1 + dist.1);
        }
        self.visited.insert(*self.knots.last().unwrap());
    }

    fn count(&self) -> usize {
        self.visited.len()
    }
}

fn parse_moves(lines: io::Lines<io::BufReader<File>>) -> Vec<(String, i32)> {
    lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_move)
        .collect()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let moves = parse_moves(lines);

    let mut rope = Rope::new(2);
    for m in moves {
        rope.make_move(&m);
    }

    format!("{}", rope.count())
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let moves = parse_moves(lines);

    let mut rope = Rope::new(10);
    for m in moves {
        rope.make_move(&m);
    }

    format!("{}", rope.count())
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_9_1() {
        let lines = read_lines("data/2022/9.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_9_2() {
        let lines = read_lines("data/2022/9.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
