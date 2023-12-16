use std::{fs::File, io, vec};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn as_u8(&self) -> u8 {
        match self {
            Up => 1,
            Right => 2,
            Down => 4,
            Left => 8, 
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
}

use Direction::*;

struct Solver {
    map: Vec<Vec<u8>>,
    beams: Vec<Vec<u8>>,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>) -> Self {
        let x = map.len();
        let y = map[0].len();
        Self{ map, beams: vec![vec![0u8; y]; x]}
    }

    fn make_beams(&mut self, dir: Direction, x: usize, y: usize) {
        // check for map boundary
        if x>=self.map.len() { return; }
        if y>=self.map[0].len() { return; }
        // map cell
        if self.check_and_update(dir.clone(), x, y) {
            return;
        }
        // advanced boundary.
        // Since x and y can not be negative, this is more complicated
        // Using i64 instead of usize for x and y might be the better choice
        let tile = self.map[x][y];
        if x==0 {
            if dir == Up && (tile==b'.' || tile==b'|') { return; }
            if dir == Right && tile == b'/' { return; }
            if dir == Left && tile == b'\\' { return; }
        }
        if y==0 {
            if dir == Left && (tile==b'.' || tile==b'-') { return; }
            if dir == Down && tile == b'/' { return; }
            if dir == Up && tile == b'\\' { return; }
        }

        // proceed beam
        if tile == b'-' && (dir == Up || dir == Down) {
            self.make_beams(Right, x, y+1);
            self.make_beams(Left, x, y-1);
            return;
        }
        if tile == b'|' && (dir == Right || dir == Left) {
            self.make_beams(Down, x+1, y);
            self.make_beams(Up, x-1, y);
            return;
        }
        if tile == b'/' {
            match dir {
                Up => { self.make_beams(Right, x, y+1); },
                Down => { self.make_beams(Left, x, y-1); },
                Left => { self.make_beams(Down, x+1, y); },
                Right => { self.make_beams(Up, x-1, y); },
            }
            return;
        }
        if tile == b'\\' {
            match dir {
                Up => { self.make_beams(Left, x, y-1); },
                Down => { self.make_beams(Right, x, y+1); },
                Left => { self.make_beams(Up, x-1, y); },
                Right => { self.make_beams(Down, x+1, y); },
            }
            return;
        }
        let (x, y) = dir.next_pos(x, y);
        self.make_beams(dir, x, y);
    }

    fn check_and_update(&mut self, dir: Direction, x: usize, y: usize) -> bool {
        if (self.beams[x][y] & dir.as_u8()) != 0 {
            true
        } else {
            self.beams[x][y] |= dir.as_u8();
            false
        }
    }

    fn energized(&self) -> u64 {
        let mut sum = 0;
        for row in &self.beams {
            for b in row {
                if *b != 0 {
                    sum += 1;
                }
            }
        }
        sum
    }

    fn clear(&mut self) {
        self.beams = vec![vec![0u8; self.map[0].len()]; self.map.len()];
    }

    fn print(&self) {
        for row in &self.beams {
            for b in row {
                if *b != 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn riddle_1(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().to_vec());
    }
    let mut solver = Solver::new(map);
    solver.make_beams(Direction::Right, 0, 0);
    let solution = solver.energized();
    format!("{solution}")
}

fn riddle_2(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().to_vec());
    }
    let x_len = map.len();
    let y_len = map[0].len();
    let mut solution = 0;
    let mut solver = Solver::new(map.clone());
    for i in 0..x_len {
        solver.clear();
        solver.make_beams(Direction::Right, i, 0);
        solution = solution.max(solver.energized());
        solver.clear();
        solver.make_beams(Direction::Left, i, y_len-1);
        solution = solution.max(solver.energized());
    }
    for i in 0..y_len {
        solver.clear();
        solver.make_beams(Direction::Down, 0, i);
        solution = solution.max(solver.energized());
        solver.clear();
        solver.make_beams(Direction::Up, x_len-1, i);
        solution = solution.max(solver.energized());
    }
    format!("{solution}")
}
