use std::{
    fmt::{self, Display},
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

struct StaticInfo {
    damaged_pattern: Vec<u32>,
}

impl Display for StaticInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pattern: {:?}", self.damaged_pattern)
    }
}

#[derive(Clone, Debug)]
struct State {
    map: Vec<u8>,
    pos: usize,
    remaining: i32,
    pattern_pos: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "map: {:?}, ", String::from_utf8_lossy(&self.map))?;
        write!(
            f,
            "pos: {}, remaining: {}, pattern_pos: {}",
            self.pos, self.remaining, self.pattern_pos
        )
    }
}

fn match_damage_block(pos: usize, map: &[u8], d_len: usize) -> bool {
    if pos + d_len > map.len() {
        return false;
    }
    for i in pos..pos + d_len {
        if map[i] == b'.' {
            return false;
        }
    }
    if pos + d_len < map.len() && map[pos + d_len] == b'#' {
        return false;
    }
    true
}

fn is_valid(map: &[u8], pattern: &[u32]) -> bool {
    let mut count = 0;
    let mut idx = 0;
    for m in map {
        if count > 0 && *m != b'#' {
            if count != pattern[idx] {
                return false;
            }
            count = 0;
            idx += 1;
        }
        if *m == b'#' {
            count += 1;
        }
    }
    if count > 0 {
        pattern[idx] == count
    } else {
        true
    }
}

fn solve(mut state: State, info: &StaticInfo) -> usize {
    if state.remaining < 0 {
        return 0;
    }
    if state.remaining == 0 {
        if state.pattern_pos != info.damaged_pattern.len() {
            if !is_valid(&state.map, &info.damaged_pattern) {
                return 0;
            }
        }
        return 1;
    }
    let mut pos = state.pos;
    let map = &state.map;
    while pos < map.len() && map[pos] == b'.' {
        pos += 1;
    }
    if pos >= map.len() {
        return 0;
    }
    let mut sum = if match_damage_block(
        pos,
        &state.map,
        info.damaged_pattern[state.pattern_pos] as usize,
    ) {
        let d_len = info.damaged_pattern[state.pattern_pos] as usize;
        let mut new_map = map.clone();
        let mut remaining = state.remaining;
        for i in 0..d_len {
            if new_map[pos + i] == b'?' {
                remaining -= 1;
            }
            new_map[pos + i] = b'#';
        }
        if new_map.len() > pos + d_len {
            new_map[pos + d_len] = b'.';
        }
        solve(
            State {
                map: new_map,
                pos: pos + d_len + 1,
                remaining: remaining,
                pattern_pos: state.pattern_pos + 1,
            },
            info,
        )
    } else {
        0
    };
    let map = &mut state.map;
    if map[pos] != b'#' {
        if map[pos] == b'?' {
            map[pos] = b'.';
        }
        state.pos = pos;
        sum += solve(state, info);
    }
    sum
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.trim().split(' ').collect();
        let map = parts[0].as_bytes().to_vec();
        let pattern: Vec<u32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
        let mut count: u32 = 0;
        for m in &map {
            if *m == b'#' {
                count += 1;
            }
        }
        let total_damaged: u32 = pattern.iter().map(|p| *p).sum();
        let state = State {
            map,
            pos: 0,
            remaining: (total_damaged - count) as i32,
            pattern_pos: 0,
        };
        let info = &StaticInfo {
            damaged_pattern: pattern,
        };
        let combinations = solve(state, info);
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
        let pattern: Vec<u32> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

        let mut count: u32 = 0;
        for m in &map {
            if *m == b'#' {
                count += 1;
            }
        }
        let total_damaged: u32 = pattern.iter().map(|p| *p).sum();
        let combinations = solve(
            State {
                map: map.clone(),
                pos: 0,
                remaining: (total_damaged - count) as i32,
                pattern_pos: 0,
            },
            &StaticInfo {
                damaged_pattern: pattern.clone(),
            },
        );
        let mut nmap = map.clone();
        nmap.push(b'#');
        let combinations2 = solve(
            State {
                map: nmap,
                pos: 0,
                remaining: (total_damaged - count - 1) as i32,
                pattern_pos: 0,
            },
            &StaticInfo {
                damaged_pattern: pattern,
            },
        );
        println!("combinations: {combinations}, combinations2: {combinations2}");
        if combinations == 0 {
            panic!("no solution for pattern {:?}", parts);
        }
        sum += combinations * combinations * combinations * combinations * combinations;
    }
    format!("{sum}")
}
