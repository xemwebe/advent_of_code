use std::{
    collections::HashSet,
    fs::File,
    io,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Position {
    x: usize,
    y: usize,
}

/// Simple path without branches
#[derive(Debug, PartialEq, Eq, Clone)]
struct SimplePath {
    // all positions of the path
    positions: Vec<Position>,
    dead_end: bool,
}

#[derive(Debug)]
struct Solver {
    map: Vec<Vec<u8>>,
    paths: Vec<SimplePath>,
    start: Position,
    end: Position,
    avoid_slippery: bool,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>, avoid_slippery: bool) -> Self {
        let mut start = Position { x: 0, y: 0 };
        for y in 0..map[0].len() {
            if map[0][y] == b'.' {
                start = Position { x: 0, y }
            }
        }
        let mut end = Position { x: 0, y: 0 };
        let xend = map.len() - 1;
        for y in 0..map[xend].len() {
            if map[xend][y] == b'.' {
                end = Position { x: xend, y }
            }
        }
        Self {
            map,
            paths: Vec::new(),
            start,
            end,
            avoid_slippery,
        }
    }

    // SimplePath must contain two positions, the starting position and the first step
    fn find_simple_path(&self, path: &mut SimplePath) -> Vec<Position> {
        let mut pos = path.positions[1].clone();
        let mut new_pos = Vec::with_capacity(4);
        loop {
            new_pos.clear();
            if pos.x > 0 {
                self.check_pos(pos.x - 1, pos.y, path, b'^', &mut new_pos)
            };
            if pos.x + 1 < self.map.len() {
                self.check_pos(pos.x + 1, pos.y, path, b'v', &mut new_pos);
            }
            self.check_pos(pos.x, pos.y - 1, path, b'<', &mut new_pos);
            self.check_pos(pos.x, pos.y + 1, path, b'>', &mut new_pos);
            if new_pos.len() == 1 {
                let position = new_pos[0].clone();
                path.positions.push(position.clone());
                pos = position;
                if pos == self.end {
                    return Vec::new();
                }
            } else {
                if new_pos.len() > 1 {
                    return new_pos;
                } else {
                    path.dead_end = true;
                    return new_pos;
                }
            }
        }
    }

    fn check_pos(
        &self,
        x: usize,
        y: usize,
        path: &SimplePath,
        optional: u8,
        new_positions: &mut Vec<Position>,
    ) {
        if (self.map[x][y] == b'.' || self.map[x][y] == optional) || (!self.avoid_slippery && self.map[x][y]!=b'#') {
            if path.positions.len() > 1 {
                let prev_pos = &path.positions[path.positions.len() - 2];
                if prev_pos.x == x && prev_pos.y == y {
                    return;
                }
            }
            new_positions.push(Position { x, y });
        }
    }

    fn find_all_simple_path(&mut self) {
        let mut path = SimplePath {
            positions: Vec::new(),
            dead_end: false,
        };
        path.positions.push(self.start.clone());
        path.positions.push(Position {
            x: 1,
            y: self.start.y,
        });

        let mut path = SimplePath {
            positions: Vec::new(),
            dead_end: false,
        };
        path.positions.push(self.start.clone());
        path.positions.push(Position {
            x: 1,
            y: self.start.y,
        });
        let mut path_list = vec![path];
        while !path_list.is_empty() {
            path = path_list.pop().unwrap();
            let next_positions = self.find_simple_path(&mut path);
            if !path.dead_end {
                self.paths.push(path.clone());
                if !next_positions.is_empty() {
                    for next in next_positions {
                        let mut new_path = SimplePath {
                            positions: Vec::new(),
                            dead_end: false,
                        };
                        let path_len = path.positions.len();
                        new_path
                            .positions
                            .push(path.positions[path_len - 1].clone());
                        new_path.positions.push(next);
                        if !self.is_duplicate(&new_path) {
                            path_list.push(new_path);
                        }
                    }
                }
            }
        }
    }

    fn is_duplicate(&self, path: &SimplePath) -> bool {
        for p in &self.paths {
            if p.positions[0] == path.positions[0] && p.positions[1] == path.positions[1] {
                return true;
            }
        }
        false
    }

    fn eliminate_dead_ends(&mut self) {
        let mut no_dead_ends = false;
        while !no_dead_ends {
            no_dead_ends = true;
            let mut paths = vec![self.paths[0].clone()];
            for i in 1..self.paths.len() {
                let last_i_pos = self.paths[i].positions.last().unwrap().clone();
                if last_i_pos == self.end {
                    paths.push(self.paths[i].clone());
                    continue;
                }
                let mut following = 0;
                for j in i+1..self.paths.len() {
                    if self.paths[j].positions[0] == last_i_pos {
                        following += 1;
                    }
                }
                if following > 0 {
                    paths.push(self.paths[i].clone());
                } else {
                    no_dead_ends = false;
                }
            }
            self.paths = paths;
        }
    }

    fn find_longest_path(&self, idx: usize, visited: HashSet<Position>) -> usize {
        let last_idx_pos = self.paths[idx].positions.last().unwrap().clone();
        if visited.contains(&last_idx_pos) {
            return 0;
        }
        //println!("idx: {idx}");
        let mut visited = visited;
        visited.insert(last_idx_pos.clone());
        if last_idx_pos == self.end {
            return self.paths[idx].positions.len()-1;
        }
        let mut max_len = 0;
        for j in 1..self.paths.len() {
            if j!=idx && self.paths[j].positions[0] == last_idx_pos {
                max_len = max_len.max(self.find_longest_path(j, visited.clone()));
            }
        }
        if max_len == 0 { return 0; }
        max_len + self.paths[idx].positions.len() -1
}
    
}

fn parse_input(lines: io::Lines<io::BufReader<File>>, avoid_slippery: bool) -> Solver {
    let mut map = Vec::new();
    for l in lines {
        let line = l.unwrap();
        map.push(line.as_bytes().to_vec());
    }
    Solver::new(map, avoid_slippery)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines, true);
    solver.find_all_simple_path();
    //println!("path count: {}", solver.paths.len());
    //solver.eliminate_dead_ends();
    let solution = solver.find_longest_path(0, HashSet::new());
    // for p in &solver.paths {
    //     println!("{:?}", p.positions);
    // }
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines, false);
    solver.find_all_simple_path();
    println!("path count: {}", solver.paths.len());
    solver.eliminate_dead_ends();
    println!("path count: {}", solver.paths.len());
    let solution = solver.find_longest_path(0, HashSet::new());
    for p in &solver.paths {
        println!("{:?}", p.positions);
    }
    format!("{solution}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_23_1() {
        let lines = read_lines("data/2023/23.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "2210");
    }

    #[test]
    fn test_2023_23_2() {
        let lines = read_lines("data/2023/23.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "43056");
    }
}
