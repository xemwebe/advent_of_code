use std::{
    fmt::{self, Display},
    fs::File,
    io,
    collections::HashMap,
    hash::Hash,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

struct StaticInfo {
    map: Vec<u8>,
    damaged_pattern: Vec<usize>,
}

impl Display for StaticInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pattern: {:?}", self.damaged_pattern)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    pos: usize,
    remaining: usize,
    pattern_pos: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos: {}, remaining: {}, pattern_pos: {}",
            self.pos, self.remaining, self.pattern_pos
        )
    }
}

struct Solver {
    cache: HashMap<State, usize>,
    info: StaticInfo,
}

impl Solver {
    fn new(info: StaticInfo ) -> Self {
        Self  { cache: HashMap::new(), info }
    }

    fn solve(&mut self, state: State,) -> usize {
        //println!("state: {state}");
        if state.pattern_pos == self.info.damaged_pattern.len() {
            return 1;
        }
        if state.pos >= self.info.map.len() {
            return 0;
        }
        if let Some(score) = self.cache.get(&state) {
            return *score;
        }

        let mut pos = state.pos;
        let mut total_score = 0;
        let pattern_pos = state.pattern_pos;
        let pattern_len = self.info.damaged_pattern[pattern_pos];
        let remaining = state.remaining;
        while pos <= self.info.map.len()-state.remaining as usize {
            if let Some((new_pos, used)) = self.find_next_pattern(pos, pattern_len, remaining) {
                let score = self.solve(State{pos: new_pos+pattern_len+1, pattern_pos: pattern_pos+1, remaining: remaining-used});
                if score == 0 {
                    pos += 1;
                    continue;
                }
                total_score += score;
                if self.info.map[new_pos] == b'#' {
                    break;
                }
                pos = new_pos + 1;
            } else {
                break;
            }
        }
        self.cache.insert(state, total_score);
        
        total_score
    }

    fn find_next_pattern(&self, pos: usize, len: usize, remaining: usize) -> Option<(usize, usize)> {
        let map = &self.info.map;
        
        let mut last_round = false;
        for i in pos..=(map.len()-len) {
            // check for match
            let mut joker = 0;
            let mut is_match = true;
            if map[i] == b'#' {
                last_round = true;
            }
            for j in i..i+len {
                if map[j] == b'?' {
                    joker += 1;
                }
                if map[j] == b'.' {
                    is_match = false;
                }
            }
            if is_match {
                if joker > remaining {
                    continue;
                }
                // check that pattern is not folloed by '#'
                if i+len < map.len() && map[i+len] == b'#' {
                    continue;
                }
                return Some((i, joker));
            }
            if last_round {
                break;
            }
        }
        None
    }

}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.trim().split(' ').collect();
        println!("row: {:?}", parts);
        let map = parts[0].as_bytes().to_vec();
        let pattern: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
        let mut count: usize = 0;
        for m in &map {
            if *m == b'#' {
                count += 1;
            }
        }
        let total_damaged: usize = pattern.iter().map(|p| *p).sum();
        let mut solver = Solver::new(StaticInfo {
            map,
            damaged_pattern: pattern,
        });
        let state = State {
            pos: 0,
            remaining: total_damaged - count,
            pattern_pos: 0,
        };
        let combinations = solver.solve(state);
        println!("combinations: {combinations}");
        if combinations == 0 {
            panic!("no solution for pattern {:?}", parts);
        }
        sum += combinations;
    }
    format!("{sum}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.trim().split(' ').collect();
        let map = parts[0].as_bytes().to_vec();
        let pattern: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

        let mut full_map = map.clone();
        let mut full_pattern = pattern.clone();
        for _ in 0..4 {
            full_map.push(b'?');
            full_map.extend(&map);
            full_pattern.extend(&pattern);
        }
        let total_damaged: usize = full_pattern.iter().map(|p| *p).sum();
        let mut count: usize = 0;
        for m in &full_map {
            if *m == b'#' {
                count += 1;
            }
        }
        let mut solver = Solver::new(StaticInfo {
            map: full_map,
            damaged_pattern: full_pattern,
        });
        let state = State {
            pos: 0,
            remaining: total_damaged - count,
            pattern_pos: 0,
        };
        let combinations = solver.solve(state);
        if combinations == 0 {
            panic!("no solution for pattern {:?}", parts);
        }
        sum += combinations;
    }
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_12_1() {
        let lines = read_lines("data/2023/12.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2023_12_2() {
        let lines = read_lines("data/2023/12.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
