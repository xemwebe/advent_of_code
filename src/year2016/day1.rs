use std::collections::HashSet;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    dir: Direction,
}

impl Position {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Up,
        }
    }

    fn go(&mut self, steps: i64) {
        match self.dir {
            Up => {
                self.y += steps;
            }
            Right => {
                self.x += steps;
            }
            Down => {
                self.y -= steps;
            }
            Left => {
                self.x -= steps;
            }
        }
    }
}

use Direction::*;

pub fn riddle_1(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let line = lines.next().unwrap().unwrap().to_string();
    let directions: Vec<&[u8]> = line.split(", ").map(|s| s.as_bytes()).collect();
    let mut pos = Position::new();
    for mov in directions {
        match mov[0] {
            b'R' => {
                pos.dir.turn_right();
            }
            b'L' => {
                pos.dir.turn_left();
            }
            _ => panic!("Invalid move"),
        }
        let steps = String::from_utf8_lossy(&mov[1..]).parse().unwrap();
        pos.go(steps);
    }
    format!("{}", pos.x.abs() + pos.y.abs())
}

pub fn riddle_2(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let line = lines.next().unwrap().unwrap().to_string();
    let directions: Vec<&[u8]> = line.split(", ").map(|s| s.as_bytes()).collect();
    let mut pos = Position::new();
    let mut past_positions = HashSet::new();
    past_positions.insert((0, 0));
    for mov in directions {
        match mov[0] {
            b'R' => {
                pos.dir.turn_right();
            }
            b'L' => {
                pos.dir.turn_left();
            }
            _ => panic!("Invalid move"),
        }
        let steps = String::from_utf8_lossy(&mov[1..]).parse().unwrap();
        for _ in 0..steps {
            pos.go(1);
            if past_positions.contains(&(pos.x, pos.y)) {
                return format!("{}", pos.x.abs() + pos.y.abs());
            }
            past_positions.insert((pos.x, pos.y));
        }
    }
    "no solution found".to_string()
}
