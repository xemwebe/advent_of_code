use std::collections::HashMap;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Panel {
    grid: Vec<Vec<u8>>,
}

impl Panel {
    fn roll_rock_north(&mut self, i: usize, j: usize) {
        let mut k = i;
        while k > 0 {
            if self.grid[k - 1][j] != b'.' {
                break;
            }
            k -= 1;
        }
        if k != i {
            self.grid[k][j] = b'O';
            self.grid[i][j] = b'.';
        }
    }

    fn roll_rock_west(&mut self, i: usize, j: usize) {
        let mut k = j;
        while k > 0 {
            if self.grid[i][k - 1] != b'.' {
                break;
            }
            k -= 1;
        }
        if k != j {
            self.grid[i][k] = b'O';
            self.grid[i][j] = b'.';
        }
    }

    fn roll_rock_south(&mut self, i: usize, j: usize) {
        let mut k = i;
        let len = self.grid.len() - 1;
        while k < len {
            if self.grid[k + 1][j] != b'.' {
                break;
            }
            k += 1;
        }
        if k != i {
            self.grid[k][j] = b'O';
            self.grid[i][j] = b'.';
        }
    }

    fn roll_rock_east(&mut self, i: usize, j: usize) {
        let mut k = j;
        let len = self.grid[0].len() - 1;
        while k < len {
            if self.grid[i][k + 1] != b'.' {
                break;
            }
            k += 1;
        }
        if k != j {
            self.grid[i][k] = b'O';
            self.grid[i][j] = b'.';
        }
    }

    fn move_north(&mut self) {
        for i in 1..self.grid.len() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == b'O' {
                    self.roll_rock_north(i, j);
                }
            }
        }
    }

    fn move_west(&mut self) {
        for j in 1..self.grid[0].len() {
            for i in 0..self.grid.len() {
                if self.grid[i][j] == b'O' {
                    self.roll_rock_west(i, j);
                }
            }
        }
    }

    fn move_south(&mut self) {
        for i in (0..self.grid.len() - 1).rev() {
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == b'O' {
                    self.roll_rock_south(i, j);
                }
            }
        }
    }

    fn move_east(&mut self) {
        for j in (0..self.grid[0].len() - 1).rev() {
            for i in 0..self.grid.len() {
                if self.grid[i][j] == b'O' {
                    self.roll_rock_east(i, j);
                }
            }
        }
    }

    fn calc_load(&self) -> u64 {
        let mut sum = 0;
        for i in 0..self.grid.len() {
            let mut count = 0;
            for j in 0..self.grid[i].len() {
                if self.grid[i][j] == b'O' {
                    count += 1;
                }
            }
            sum += (self.grid.len() - i) as u64 * count;
        }
        sum
    }

    fn cycle(&mut self) {
        self.move_north();
        self.move_west();
        self.move_south();
        self.move_east();
    }
}

impl std::fmt::Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in &self.grid {
            write!(f, "{}\n", String::from_utf8_lossy(row))?
        }
        Ok(())
    }
}
fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = Vec::new();
    for l in lines {
        let line = l.unwrap();
        grid.push(line.trim().as_bytes().to_vec());
    }
    let mut panel = Panel { grid };
    panel.move_north();
    let solution = panel.calc_load();
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = Vec::new();
    for l in lines {
        let line = l.unwrap();
        grid.push(line.trim().as_bytes().to_vec());
    }
    let mut panel = Panel { grid };
    let mut hashes: HashMap<Panel, usize> = HashMap::new();
    for i in 0..1000000000 {
        panel.cycle();
        if hashes.contains_key(&panel) {
            let first = hashes[&panel];
            let idx = (1000000000 - first) % (i - first) + first - 1;
            for (p, k) in &hashes {
                if *k == idx {
                    let solution = p.calc_load();
                    return format!("{solution}");
                }
            }
        }
        hashes.insert(panel.clone(), i);
    }
    "no solution found".to_string()
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_14_1() {
        let lines = read_lines("data/2023/14.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "113424");
    }

    #[test]
    fn test_2023_14_2() {
        let lines = read_lines("data/2023/14.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "96003");
    }
}
