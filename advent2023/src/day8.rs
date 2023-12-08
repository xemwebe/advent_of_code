use super::*;
use num::Integer;
use std::collections::HashMap;

pub fn riddle_8_1(lines: io::Lines<io::BufReader<File>>) {
    let mut lines = lines.into_iter();
    let line = lines.next().unwrap().unwrap().clone();
    let directions = line.trim().as_bytes();
    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let l = line.unwrap();
        map.insert(
            l[0..3].to_string(),
            (l[7..10].to_string(), l[12..15].to_string()),
        );
    }
    let mut current: &str = "AAA";
    let mut solution = 0;
    let d_len = directions.len();
    while current != "ZZZ" {
        current = match directions[solution % d_len] {
            b'L' => &map[current].0,
            b'R' => &map[current].1,
            _ => panic!("invalid directions"),
        };
        solution += 1;
    }
    println!("The solution is: {}", solution);
}

struct State<'a> {
    current: &'a str,
    cycle: HashMap<&'a str, usize>,
    solution: usize,
}

impl<'a> State<'a> {
    fn check_cycle(&mut self, count: usize) {
        if !self.cycle.contains_key(self.current) {
            self.cycle.insert(self.current, count);
        } else {
            self.solution = self.cycle[self.current];
        }
    }
}

fn states_complete(states: &[State]) -> bool {
    for s in states {
        if s.solution == 0 {
            return false;
        }
    }
    true
}

// Remark: this could relies on the observation, that at least
// for my input, a location with 'Z' at the ends coincides
// always with the maximum index of my directions
pub fn riddle_8_2(lines: io::Lines<io::BufReader<File>>) {
    let mut lines = lines.into_iter();
    let line = lines.next().unwrap().unwrap().clone();
    let directions = line.trim().as_bytes();
    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let l = line.unwrap();
        map.insert(
            l[0..3].to_string(),
            (l[7..10].to_string(), l[12..15].to_string()),
        );
    }

    let mut states = Vec::new();
    for k in map.keys() {
        if k.as_bytes()[2] == b'A' {
            states.push(State {
                current: k.as_str(),
                cycle: HashMap::new(),
                solution: 0,
            });
        }
    }
    let mut count = 0;
    let d_len = directions.len();
    let s_len = states.len();
    while !states_complete(&states) {
        for i in 0..s_len {
            states[i].current = match directions[count % d_len] {
                b'L' => &map[states[i].current].0,
                b'R' => &map[states[i].current].1,
                _ => panic!("invalid directions"),
            };
            if states[i].current.as_bytes()[2] == b'Z' {
                states[i].check_cycle(count + 1);
            }
        }
        count += 1;
    }
    let solution = states
        .iter()
        .map(|s| s.solution)
        .reduce(|acc, e| acc.lcm(&e))
        .unwrap();
    println!("Solution is {solution}");
}
