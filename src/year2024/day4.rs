use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

struct Solver {
    grid: Vec<Vec<u8>>,
    n: i32,
    m: i32,
}

impl Solver {
    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];

    fn new(grid: Vec<Vec<u8>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Self {
            grid,
            n: n as i32,
            m: m as i32,
        }
    }

    fn find_xmas(&self, i: i32, j: i32, dir: (i32, i32), idx: usize) -> u32 {
        let rest = (Self::XMAS.len() - idx) as i32;
        let i_end = i + dir.0 * rest;
        let j_end = j + dir.1 * rest;
        if i_end < 0 || i_end >= self.n || j_end < 0 || j_end >= self.m {
            return 0;
        }
        let i = i + dir.0;
        let j = j + dir.1;
        let mut count = 0;
        if self.grid[i as usize][j as usize] == Self::XMAS[idx] {
            if idx == Self::XMAS.len() - 1 {
                count += 1;
            } else {
                count += self.find_xmas(i, j, dir, idx + 1);
            }
        }
        count
    }

    fn solve1(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                if self.grid[i as usize][j as usize] != Self::XMAS[0] {
                    continue;
                }
                for d in Self::DIRECTIONS {
                    count += self.find_xmas(i, j, d, 1);
                }
            }
        }
        count
    }

    fn solve2(&self) -> u32 {
        let mut count = 0;
        for i in 1..self.n as usize - 1 {
            for j in 1..self.m as usize - 1 {
                if self.grid[i][j] != Self::XMAS[2] {
                    continue;
                }
                if ((self.grid[i - 1][j - 1] == Self::XMAS[1]
                    && self.grid[i + 1][j + 1] == Self::XMAS[3])
                    || (self.grid[i - 1][j - 1] == Self::XMAS[3]
                        && self.grid[i + 1][j + 1] == Self::XMAS[1]))
                    && ((self.grid[i - 1][j + 1] == Self::XMAS[1]
                        && self.grid[i + 1][j - 1] == Self::XMAS[3])
                        || (self.grid[i - 1][j + 1] == Self::XMAS[3]
                            && self.grid[i + 1][j - 1] == Self::XMAS[1]))
                {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = Vec::new();
    for line in lines {
        let line = line.unwrap();
        grid.push(line.as_bytes().to_vec());
    }
    let solver = Solver::new(grid);
    let count = solver.solve1();
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = Vec::new();
    for line in lines {
        let line = line.unwrap();
        grid.push(line.as_bytes().to_vec());
    }
    let solver = Solver::new(grid);
    let count = solver.solve2();
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_4_1() {
        let lines = read_lines("data/2024/4.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "2569");
    }

    #[test]
    fn test_2024_4_2() {
        let lines = read_lines("data/2024/4.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1998");
    }
}
