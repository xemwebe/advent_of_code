use std::collections::HashSet;
use std::{fs::File, io};

use advent::{Direction, DirectionType, Position};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Solver {
    grid: Vec<Vec<u8>>,
    pos: Position,
    n: usize,
    m: usize,
}

impl Solver {
    fn create_from_input(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut grid = Vec::new();
        let mut pos = Position::new(0, 0);
        for line in lines {
            let mut row = line.unwrap().as_bytes().to_vec();
            for i in 0..row.len() {
                if row[i] == b'^' {
                    pos = Position::new(i as i32, grid.len() as i32);
                    row[i] = b'.';
                }
            }
            grid.push(row);
        }
        let n = grid[0].len();
        let m = grid.len();
        Self { grid, pos, n, m }
    }

    fn solve(&mut self) -> u32 {
        let mut count = 1;
        self.grid[self.pos.y as usize][self.pos.x as usize] = b'X';
        let mut dir = Direction::from_type(DirectionType::Up);
        loop {
            if !self.pos.check_move_by(&dir, self.n, self.m) {
                break;
            }
            let next_pos = &self.pos + &dir;
            if self.grid[next_pos.y as usize][next_pos.x as usize] == b'#' {
                dir.turn_right();
            } else if self.grid[next_pos.y as usize][next_pos.x as usize] == b'.' {
                count += 1;
                self.grid[next_pos.y as usize][next_pos.x as usize] = b'X';
                self.pos = next_pos;
            } else {
                self.pos = next_pos;
            }
        }
        count
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = Solver::create_from_input(lines);
    let count = solver.solve();
    format!("{count}")
}

struct MetaSolver {
    orig_solver: Solver,
}

#[derive(Debug, Clone)]
struct TestSolver {
    grid: Vec<Vec<u8>>,
    visited: Vec<Vec<HashSet<DirectionType>>>,
    pos: Position,
    n: usize,
    m: usize,
}

impl TestSolver {
    fn new(solver: &Solver) -> Self {
        let mut visited = Vec::new();
        for _ in 0..solver.m {
            let mut row = Vec::new();
            for _ in 0..solver.n {
                row.push(HashSet::new());
            }
            visited.push(row);
        }
        Self {
            grid: solver.grid.clone(),
            visited,
            pos: solver.pos.clone(),
            n: solver.n,
            m: solver.m,
        }
    }

    fn set_obstacle(&mut self, x: usize, y: usize) {
        self.grid[y][x] = b'#';
    }

    fn has_loop(&mut self) -> bool {
        let mut has_loop = false;
        let mut dir = Direction::from_type(DirectionType::Up);
        let mut dir_type = dir.to_type().unwrap();
        self.visited[self.pos.y as usize][self.pos.x as usize].insert(dir_type);
        loop {
            if !self.pos.check_move_by(&dir, self.n, self.m) {
                break;
            }
            let next_pos = &self.pos + &dir;
            if self.grid[next_pos.y as usize][next_pos.x as usize] == b'#' {
                dir.turn_right();
                dir_type = dir.to_type().unwrap();
            } else if self.visited[next_pos.y as usize][next_pos.x as usize].contains(&dir_type) {
                has_loop = true;
                break;
            } else {
                self.visited[next_pos.y as usize][next_pos.x as usize].insert(dir_type);
                self.pos = next_pos;
            }
        }
        has_loop
    }
}

impl MetaSolver {
    fn solve(&self) -> u32 {
        let mut map_solver = self.orig_solver.clone();
        _ = map_solver.solve();
        let map = map_solver.grid;
        let mut count = 0;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if i == self.orig_solver.pos.y as usize && j == self.orig_solver.pos.x as usize {
                    continue;
                }
                if map[i][j] == b'X' {
                    let mut test_solver = TestSolver::new(&self.orig_solver);
                    test_solver.set_obstacle(j, i);
                    if test_solver.has_loop() {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let meta_solver = MetaSolver {
        orig_solver: solver,
    };
    let count = meta_solver.solve();
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_3_1() {
        let lines = read_lines("data/2024/3.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "5312");
    }

    #[test]
    fn test_2024_3_2() {
        let lines = read_lines("data/2024/3.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1748");
    }
}
