use advent::{Direction, DirectionType, Position};
use std::collections::HashSet;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Solver {
    map: Vec<Vec<u8>>,
    n: usize,
    m: usize,
    peaks: HashSet<Position>,
}

impl Solver {
    const DIRECTIONS: &[DirectionType] = &[
        DirectionType::Up,
        DirectionType::Left,
        DirectionType::Down,
        DirectionType::Right,
    ];

    fn create_from_input(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut map = Vec::new();
        for line in lines {
            let row: Vec<u8> = line.unwrap().as_bytes().iter().map(|x| *x - b'0').collect();
            map.push(row);
        }
        let m = map.len();
        let n = map[0].len();
        Self {
            map,
            n,
            m,
            peaks: HashSet::new(),
        }
    }

    fn find_paths(&mut self, pos: Position) {
        let val = self.map[pos.y as usize][pos.x as usize];
        if val == 9 {
            self.peaks.insert(pos);
            return;
        }
        for dir in Self::DIRECTIONS {
            let dir = Direction::from_type(*dir);
            if let Some(new_pos) = pos.try_move_by(&dir, self.n, self.m) {
                if self.map[new_pos.y as usize][new_pos.x as usize] == val + 1 {
                    self.find_paths(new_pos);
                }
            }
        }
    }

    fn solve(&mut self) -> usize {
        let mut score = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.map[i][j] == 0 {
                    self.peaks.clear();
                    self.find_paths(Position {
                        x: j as i32,
                        y: i as i32,
                    });
                    score += self.peaks.len();
                }
            }
        }
        score
    }

    fn find_paths2(&self, pos: Position) -> usize {
        let mut score = 0;
        let val = self.map[pos.y as usize][pos.x as usize];
        if val == 9 {
            return 1;
        }
        for dir in Self::DIRECTIONS {
            let dir = Direction::from_type(*dir);
            if let Some(new_pos) = pos.try_move_by(&dir, self.n, self.m) {
                if self.map[new_pos.y as usize][new_pos.x as usize] == val + 1 {
                    score += self.find_paths2(new_pos);
                }
            }
        }
        score
    }

    fn solve2(&self) -> usize {
        let mut score = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.map[i][j] == 0 {
                    score += self.find_paths2(Position {
                        x: j as i32,
                        y: i as i32,
                    });
                }
            }
        }
        score
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = Solver::create_from_input(lines);
    let score = solver.solve();
    format!("{score}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let score = solver.solve2();
    format!("{score}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_10_1() {
        let lines = read_lines("data/2024/10.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "744");
    }

    #[test]
    fn test_2024_10_2() {
        let lines = read_lines("data/2024/10.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1651");
    }
}
