use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operation {
    Plus,
    Mult,
    Concat,
}

#[derive(Debug)]
struct Solver {
    calculations: Vec<Vec<u64>>,
}

impl Solver {
    fn create_from_input(lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut calculations = Vec::new();
        for line in lines {
            let row = line
                .unwrap()
                .split(' ')
                .map(|x| {
                    if x.ends_with(":") {
                        x[..x.len() - 1].to_string()
                    } else {
                        x.to_string()
                    }
                })
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            calculations.push(row);
        }
        Self { calculations }
    }

    fn is_valid(op: Operation, value: u64, idx: usize, row: &[u64]) -> bool {
        if idx == row.len() {
            return value == row[0];
        }
        let value = match op {
            Operation::Plus => value + row[idx],
            Operation::Mult => value * row[idx],
            _ => value,
        };
        Self::is_valid(Operation::Plus, value, idx + 1, row)
            || Self::is_valid(Operation::Mult, value, idx + 1, row)
    }

    fn is_valid2(op: Operation, value: u64, idx: usize, row: &[u64]) -> bool {
        if idx == row.len() {
            return value == row[0];
        }
        let value = match op {
            Operation::Plus => value + row[idx],
            Operation::Mult => value * row[idx],
            Operation::Concat => {
                let exp = (row[idx] as f64).log10().floor() + 1.0;
                value * 10u64.pow(exp as u32) + row[idx]
            }
        };
        Self::is_valid2(Operation::Plus, value, idx + 1, row)
            || Self::is_valid2(Operation::Mult, value, idx + 1, row)
            || Self::is_valid2(Operation::Concat, value, idx + 1, row)
    }

    fn solve1(&self) -> u64 {
        let mut sum = 0;
        for row in &self.calculations {
            if Self::is_valid(Operation::Plus, row[1], 2, &row)
                || Self::is_valid(Operation::Mult, row[1], 2, &row)
            {
                sum += row[0];
            }
        }
        sum
    }

    fn solve2(&self) -> u64 {
        let mut sum = 0;
        for row in &self.calculations {
            if Self::is_valid2(Operation::Plus, row[1], 2, &row)
                || Self::is_valid2(Operation::Mult, row[1], 2, &row)
                || Self::is_valid2(Operation::Concat, row[1], 2, &row)
            {
                sum += row[0];
            }
        }
        sum
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let sum = solver.solve1();
    format!("{sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let sum = solver.solve2();
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_7_1() {
        let lines = read_lines("data/2024/7.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "5702958180383");
    }

    #[test]
    fn test_2024_7_2() {
        let lines = read_lines("data/2024/7.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "92612386119138");
    }
}
