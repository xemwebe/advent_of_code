use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

use regex::Regex;

#[derive(Debug, Clone)]
enum Move {
    TurnRight,
    TurnLeft,
    Forward(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Orientation {
    Right,
    Left,
    Up,
    Down,
}

impl Orientation {
    fn as_num(&self) -> usize {
        match self {
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
            Orientation::Up => 3,
        }
    }

    fn get_offset(&self) -> (i64, i64) {
        match self {
            Orientation::Right => (1, 0),
            Orientation::Left => (-1, 0),
            Orientation::Up => (0, -1),
            Orientation::Down => (0, 1),
        }
    }
}

#[derive(Debug)]
struct State {
    row: usize,
    col: usize,
    dir: Orientation,
}

impl State {
    fn calc_password(&self) -> usize {
        1000 * (self.row + 1) + 4 * (self.col + 1) + self.dir.as_num()
    }

    fn go_steps(&mut self, map: &Map, steps: i32) {
        let offset = self.dir.get_offset();
        for _ in 0..steps {
            let mut c = (self.col as i64) + offset.0;
            if c < 0 {
                c = map[self.row].len() as i64 - 1;
            }
            if c >= map[self.row].len() as i64 {
                c = 0;
            }
            while map[self.row][c as usize] == VOID {
                c += offset.0;
                if c < 0 {
                    c = map[self.row].len() as i64 - 1;
                }
                if c >= map[self.row].len() as i64 {
                    c = 0;
                }
            }
            let c = c as usize;
            let mut r = (self.row as i64) + offset.1;
            if r < 0 {
                r = map.len() as i64 - 1;
            }
            if r >= map.len() as i64 {
                r = 0;
            }
            while c >= map[r as usize].len() || map[r as usize][c] == VOID {
                r += offset.1;
                if r < 0 {
                    r = map.len() as i64 - 1;
                }
                if r >= map.len() as i64 {
                    r = 0;
                }
            }
            if map[r as usize][c] == WALL {
                return;
            }
            self.row = r as usize;
            self.col = c as usize;
        }
    }

    // Hard-coded for my specific map layout
    fn cubic_wrap(r: &mut i64, c: &mut i64, dir: &mut Orientation) {
        if *c < 0 && *r >= 100 {
            if *r >= 150 {
                *c = *r - 100;
                *r = 0;
                *dir = Orientation::Down;
            } else {
                *c = 50;
                *r = 149 - *r;
                *dir = Orientation::Right;
            }
        } else if *c == 49 && *r < 100 {
            if *r < 50 {
                *c = 0;
                *r = 149 - *r;
                *dir = Orientation::Right;
            } else {
                *c = *r - 50;
                *r = 100;
                *dir = Orientation::Down;
            }
        } else if *c == 150 {
            *c = 99;
            *r = 149 - *r;
            *dir = Orientation::Left;
        } else if *c == 100 && *r >= 50 && *r < 150 {
            if *r >= 100 {
                *c = 149;
                *r = 149 - *r;
                *dir = Orientation::Left;
            } else {
                *c = *r + 50;
                *r = 49;
                *dir = Orientation::Up;
            }
        } else if *c == 50 && *r >= 150 {
            *c = *r - 100;
            *r = 149;
            *dir = Orientation::Up;
        } else if *r < 0 {
            if *c < 100 {
                *r = *c + 100;
                *c = 0;
                *dir = Orientation::Right;
            } else {
                *c -= 100;
                *r = 199;
            }
        } else if *r == 99 && *c < 50 {
            *r = *c + 50;
            *c = 50;
            *dir = Orientation::Right;
        } else if *r == 200 {
            *c += 100;
            *r = 0;
        } else if *r == 150 && *c >= 50 {
            *r = *c + 100;
            *c = 49;
            *dir = Orientation::Left;
        } else if *r == 50 && *c >= 100 {
            *r = *c - 50;
            *c = 99;
            *dir = Orientation::Left
        }
    }

    fn go_cubic_steps(&mut self, map: &Map, steps: i32) {
        for _ in 0..steps {
            let offset = self.dir.get_offset();
            let mut dir = self.dir.clone();
            let mut c = (self.col as i64) + offset.0;
            let mut r = (self.row as i64) + offset.1;
            Self::cubic_wrap(&mut r, &mut c, &mut dir);
            if map[r as usize][c as usize] == WALL {
                return;
            }
            self.row = r as usize;
            self.col = c as usize;
            self.dir = dir;
        }
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Orientation::Right => Orientation::Up,
            Orientation::Up => Orientation::Left,
            Orientation::Left => Orientation::Down,
            Orientation::Down => Orientation::Right,
        };
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Orientation::Right => Orientation::Down,
            Orientation::Up => Orientation::Right,
            Orientation::Left => Orientation::Up,
            Orientation::Down => Orientation::Left,
        };
    }

    fn walk(&mut self, map: &Map, moves: &[Move]) {
        for m in moves {
            match m {
                Move::TurnLeft => {
                    self.turn_left();
                }
                Move::TurnRight => {
                    self.turn_right();
                }
                Move::Forward(x) => {
                    self.go_steps(map, *x);
                }
            }
        }
    }

    fn cubic_walk(&mut self, map: &Map, moves: &[Move]) {
        for m in moves {
            match m {
                Move::TurnLeft => {
                    self.turn_left();
                }
                Move::TurnRight => {
                    self.turn_right();
                }
                Move::Forward(x) => {
                    self.go_cubic_steps(map, *x);
                }
            }
        }
    }
}
const EMPTY: u8 = 46;
const VOID: u8 = 32;
const WALL: u8 = 35;

type Map = Vec<Vec<u8>>;

fn read_map_and_moves(lines: io::Lines<io::BufReader<File>>) -> (Map, Vec<Move>) {
    let re = Regex::new("([0-9]+)|([RL]{1})").unwrap();
    let mut map = Vec::new();
    let mut moves = Vec::new();
    let mut map_mode = true;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            map_mode = false;
            continue;
        }
        if map_mode {
            map.push(line.as_bytes().to_owned());
        } else {
            for cap in re.captures_iter(&line) {
                moves.push(match &cap[0] {
                    "R" => Move::TurnRight,
                    "L" => Move::TurnLeft,
                    num => Move::Forward(num.parse::<i32>().unwrap()),
                });
            }
        }
    }
    (map, moves)
}

fn find_start(map: &Map) -> State {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == EMPTY {
                return State {
                    row,
                    col,
                    dir: Orientation::Right,
                };
            }
        }
    }
    panic!("Found no starting point");
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let (map, moves) = read_map_and_moves(lines);
    let mut state = find_start(&map);
    state.walk(&map, &moves);
    let password = state.calc_password();
    format!("{password}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let (map, moves) = read_map_and_moves(lines);
    let mut state = find_start(&map);
    state.cubic_walk(&map, &moves);
    let password = state.calc_password();
    format!("{password}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2022_22_1() {
        let lines = read_lines("data/2022/22.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_22_2() {
        let lines = read_lines("data/2022/22.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}

