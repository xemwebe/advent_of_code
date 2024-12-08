use std::collections::{HashMap, HashSet};
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug)]
struct Solver {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

impl Solver {
    fn create_from_input(mut lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
        loop {
            let line = lines.next().unwrap().unwrap();
            if line == "" {
                break;
            }
            let mut parts = line.split("|");
            let left = parts.next().unwrap().parse::<u32>().unwrap();
            let right = parts.next().unwrap().parse::<u32>().unwrap();
            if rules.contains_key(&left) {
                rules.get_mut(&left).unwrap().insert(right);
            } else {
                let mut set = HashSet::new();
                set.insert(right);
                rules.insert(left, set);
            }
        }
        let mut updates = Vec::new();
        for line in lines {
            updates.push(
                line.unwrap()
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
            )
        }
        Self { rules, updates }
    }

    fn solve1(&self) -> u32 {
        let mut count = 0;
        for u in &self.updates {
            let mut valid = true;
            for i in 1..u.len() {
                for j in 0..i {
                    if let Some(&ref followers) = self.rules.get(&u[i]) {
                        if followers.contains(&u[j]) {
                            valid = false;
                            break;
                        }
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                count += u[u.len() / 2];
            }
        }
        count
    }

    fn solve2(&self) -> u32 {
        let mut count = 0;
        for u in &self.updates {
            let mut valid = true;
            for i in 1..u.len() {
                for j in 0..i {
                    if let Some(&ref followers) = self.rules.get(&u[i]) {
                        if followers.contains(&u[j]) {
                            valid = false;
                            break;
                        }
                    }
                }
                if !valid {
                    break;
                }
            }
            if !valid {
                let mut sorted_update = u.clone();
                sorted_update.sort_by(|a, b| {
                    if let Some(&ref followers_a) = self.rules.get(a) {
                        if followers_a.contains(&b) {
                            return std::cmp::Ordering::Less;
                        }
                    }
                    if let Some(&ref followers_b) = self.rules.get(b) {
                        if followers_b.contains(&a) {
                            return std::cmp::Ordering::Greater;
                        }
                    }
                    std::cmp::Ordering::Equal
                });
                count += sorted_update[sorted_update.len() / 2];
            }
        }
        count
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let count = solver.solve1();
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let count = solver.solve2();
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_5_1() {
        let lines = read_lines("data/2024/5.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "5588");
    }

    #[test]
    fn test_2024_5_2() {
        let lines = read_lines("data/2024/5.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "5331");
    }
}
