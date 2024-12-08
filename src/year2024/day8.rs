use std::collections::{HashMap, HashSet};
use std::{fs::File, io};

use advent::Position;

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Solver {
    antennas: HashMap<u8, Vec<Position>>,
    n: usize,
    m: usize,
}

impl Solver {
    fn create_from_input(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut n = 0;
        let mut m = 0;
        let mut antennas: HashMap<u8, Vec<Position>> = HashMap::new();
        for line in lines {
            let row = line.unwrap().as_bytes().to_vec();
            m = row.len();
            for (i, element) in row.iter().enumerate() {
                if *element != b'.' {
                    if let std::collections::hash_map::Entry::Vacant(e) = antennas.entry(*element) {
                        e.insert(vec![Position::new(i as i32, n as i32)]);
                    } else {
                        antennas
                            .get_mut(element)
                            .unwrap()
                            .push(Position::new(i as i32, n as i32));
                    }
                }
            }
            n += 1;
        }
        Self { antennas, n, m }
    }

    fn solve1(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        for frequency in self.antennas.keys() {
            for i in 0..self.antennas[frequency].len() {
                for j in i + 1..self.antennas[frequency].len() {
                    let dir = &self.antennas[frequency][i] - &self.antennas[frequency][j];
                    let pos = &self.antennas[frequency][i] + &dir;
                    if pos.check_on_grid(self.n, self.m) {
                        antinodes.insert(pos);
                    }
                    let pos = &self.antennas[frequency][j] - &dir;
                    if pos.check_on_grid(self.n, self.m) {
                        antinodes.insert(pos);
                    }
                }
            }
        }
        antinodes.len()
    }

    fn solve2(&mut self) -> usize {
        let mut antinodes = HashSet::new();
        for frequency in self.antennas.keys() {
            for i in 0..self.antennas[frequency].len() {
                for j in i + 1..self.antennas[frequency].len() {
                    let dir = &self.antennas[frequency][i] - &self.antennas[frequency][j];
                    let mut pos = self.antennas[frequency][i].clone();
                    loop {
                        if !pos.check_on_grid(self.n, self.m) {
                            break;
                        }
                        antinodes.insert(pos.clone());
                        pos = &pos + &dir;
                    }
                    let mut pos = self.antennas[frequency][j].clone();
                    loop {
                        if !pos.check_on_grid(self.n, self.m) {
                            break;
                        }
                        antinodes.insert(pos.clone());
                        pos = &pos - &dir;
                    }
                }
            }
        }
        antinodes.len()
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = Solver::create_from_input(lines);
    let count = solver.solve1();
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = Solver::create_from_input(lines);
    let count = solver.solve2();
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_8_1() {
        let lines = read_lines("data/2024/8.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "247");
    }

    #[test]
    fn test_2024_8_2() {
        let lines = read_lines("data/2024/8.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "861");
    }
}
