use std::{
    fs::File, 
    io,
    collections::HashSet,
    cmp::Ordering,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Solver {
    map: Vec<Vec<u8>>,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Self { map }
    }

    fn longest_path(&self, mut past_positions: Vec<Position>) -> u64 {
        let current_pos = past_positions.last().unwrap();
        if current_pos.x == self.map.len()-1 {
            return past_positions.len() as u64;
        }
        let mut new_positions = Vec::new();
        if current_pos.x>0 { self.check_pos(current_pos.x-1, current_pos.y, &mut new_positions, &past_positions, b'^'); }
        self.check_pos(current_pos.x+1, current_pos.y, &mut new_positions, &past_positions, b'v');
        self.check_pos(current_pos.x, current_pos.y-1, &mut new_positions, &past_positions, b'<');
        self.check_pos(current_pos.x, current_pos.y+1, &mut new_positions, &past_positions, b'>');
        if new_positions.is_empty() { return 0; }
        let mut max_len = 0;
        for i in 1..new_positions.len() {
            let mut positions = past_positions.clone();
            positions.push(new_positions[i].clone());
            max_len = max_len.max(self.longest_path(past_positions.clone()));
        }
        past_positions.push(new_positions[0].clone());
        max_len = max_len.max(self.longest_path(past_positions));
        return max_len;  
    }

    fn check_pos(&self, x: usize, y: usize, new_positions: &mut Vec<Position>, past_positions: &Vec<Position>, optional: u8 ) {
        if (self.map[x][y] == b'.'  || self.map[x][y] == optional) && !past_positions.contains(&Position{x, y}) {
            new_positions.push(Position{x, y});
        }
    }

}

fn parse_input(lines: io::Lines<io::BufReader<File>>) -> Solver {
    let mut map = Vec::new();
    for l in lines {
        let line = l.unwrap();
        map.push(line.as_bytes().to_vec());
    }
    Solver::new(map)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines);
    let length = solver.longest_path(vec![Position{x:0, y:1}]);
    format!("{length}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines);
    format!("none")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_23_1() {
        let lines = read_lines("data/2023/23.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "439");
    }

    #[test]
    fn test_2023_23_2() {
        let lines = read_lines("data/2023/23.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "43056");
    }
}
