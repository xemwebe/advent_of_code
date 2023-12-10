use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

use std::collections::{HashMap, HashSet};

fn read_map(mut lines: io::Lines<io::BufReader<File>>) -> HashSet<(i64, i64)> {
    let mut row = 0;
    let mut elves = HashSet::new();
    while let Some(Ok(s)) = lines.next() {
        let bytes = s.as_bytes();
        for col in 0..bytes.len() {
            if bytes[col] == 35 {
                elves.insert((row, col as i64));
            }
        }
        row += 1;
    }
    elves
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::West,
            Self::West => Self::East,
            Self::East => Self::North,
        }
    }
}

#[derive(Debug)]
struct State {
    elves: HashSet<(i64, i64)>,
    direction: Direction,
}

impl State {
    fn calc_moves(&mut self) -> HashMap<(i64, i64), (i64, i64)> {
        let mut moves = HashMap::new();
        for e in &self.elves {
            let mut dir = self.direction.clone();
            loop {
                if !(self.elves.contains(&(e.0 - 1, e.1 - 1))
                    || self.elves.contains(&(e.0 - 1, e.1))
                    || self.elves.contains(&(e.0 - 1, e.1 + 1))
                    || self.elves.contains(&(e.0, e.1 - 1))
                    || self.elves.contains(&(e.0, e.1 + 1))
                    || self.elves.contains(&(e.0 + 1, e.1 - 1))
                    || self.elves.contains(&(e.0 + 1, e.1))
                    || self.elves.contains(&(e.0 + 1, e.1 + 1)))
                {
                    break;
                }

                match dir {
                    Direction::North => {
                        if !(self.elves.contains(&(e.0 - 1, e.1 - 1))
                            || self.elves.contains(&(e.0 - 1, e.1))
                            || self.elves.contains(&(e.0 - 1, e.1 + 1)))
                        {
                            if let Some(_) = moves.insert((e.0 - 1, e.1), *e) {
                                moves.remove(&(e.0 - 1, e.1));
                            }
                            break;
                        }
                    }
                    Direction::South => {
                        if !(self.elves.contains(&(e.0 + 1, e.1 - 1))
                            || self.elves.contains(&(e.0 + 1, e.1))
                            || self.elves.contains(&(e.0 + 1, e.1 + 1)))
                        {
                            if let Some(_) = moves.insert((e.0 + 1, e.1), *e) {
                                moves.remove(&(e.0 + 1, e.1));
                            }
                            break;
                        }
                    }

                    Direction::West => {
                        if !(self.elves.contains(&(e.0 - 1, e.1 - 1))
                            || self.elves.contains(&(e.0, e.1 - 1))
                            || self.elves.contains(&(e.0 + 1, e.1 - 1)))
                        {
                            if let Some(_) = moves.insert((e.0, e.1 - 1), *e) {
                                moves.remove(&(e.0, e.1 - 1));
                            }
                            break;
                        }
                    }
                    Direction::East => {
                        if !(self.elves.contains(&(e.0 - 1, e.1 + 1))
                            || self.elves.contains(&(e.0, e.1 + 1))
                            || self.elves.contains(&(e.0 + 1, e.1 + 1)))
                        {
                            if let Some(_) = moves.insert((e.0, e.1 + 1), *e) {
                                moves.remove(&(e.0, e.1 + 1));
                            }
                            break;
                        }
                    }
                }
                dir = dir.next();
                if dir == self.direction {
                    break;
                }
            }
        }
        self.direction = self.direction.next();
        moves
    }

    fn move_elves(&mut self, moves: &HashMap<(i64, i64), (i64, i64)>) {
        for m in moves {
            self.elves.remove(m.1);
            self.elves.insert(*m.0);
        }
    }

    fn count_space(&self) -> i64 {
        let (xmin, xmax, ymin, ymax) = self.min_max();
        (xmax - xmin + 1) * (ymax - ymin + 1) - self.elves.len() as i64
    }

    fn min_max(&self) -> (i64, i64, i64, i64) {
        let mut xmin = i64::MAX;
        let mut xmax = i64::MIN;
        let mut ymin = i64::MAX;
        let mut ymax = i64::MIN;
        for e in &self.elves {
            xmin = xmin.min(e.0);
            xmax = xmax.max(e.0);
            ymin = ymin.min(e.1);
            ymax = ymax.max(e.1);
        }
        (xmin, xmax, ymin, ymax)
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut state = State {
        elves: read_map(lines),
        direction: Direction::North,
    };

    for _ in 0..10 {
        let moves = state.calc_moves();
        state.move_elves(&moves);
    }
    let result = state.count_space();
    format!("{result}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut state = State {
        elves: read_map(lines),
        direction: Direction::North,
    };

    let mut round = 1;
    loop {
        let moves = state.calc_moves();
        if moves.is_empty() {
            break;
        }
        state.move_elves(&moves);
        round += 1;
    }
    format!("{round}")
}
