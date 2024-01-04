use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn parse_init_stack(lines: Vec<String>) -> Vec<Vec<u8>> {
    let mut stacks = Vec::new();
    for i in 0..9 {
        let mut stack = Vec::new();
        let len = lines.len();
        let idx = i * 4 + 1;
        for j in 0..len {
            let bytes = lines[len - j - 1].as_bytes();
            if bytes[idx] != 32 {
                stack.push(bytes[idx]);
            }
        }
        stacks.push(stack);
    }
    stacks
}

fn parse_move(s: String) -> (usize, usize, usize) {
    let mut parts = s.split(" ");
    parts.advance_by(1).unwrap();
    let count: usize = parts.next().unwrap().parse().unwrap();
    parts.advance_by(1).unwrap();
    let from: usize = parts.next().unwrap().parse().unwrap();
    parts.advance_by(1).unwrap();
    let to: usize = parts.next().unwrap().parse().unwrap();
    (count, from, to)
}

fn make_move(stacks: Vec<Vec<u8>>, m: (usize, usize, usize)) -> Vec<Vec<u8>> {
    let mut stacks = stacks;
    for _ in 0..m.0 {
        let c = stacks[m.1 - 1].pop().unwrap();
        stacks[m.2 - 1].push(c);
    }
    stacks
}

fn make_move_9001(stacks: Vec<Vec<u8>>, m: (usize, usize, usize)) -> Vec<Vec<u8>> {
    let mut stacks = stacks;
    let len = stacks[m.1 - 1].len();
    for i in len - m.0..len {
        let c = stacks[m.1 - 1][i].clone();
        stacks[m.2 - 1].push(c);
    }
    for _ in 0..m.0 {
        stacks[m.1 - 1].pop();
    }
    stacks
}

fn get_tops(stacks: &mut Vec<Vec<u8>>) -> String {
    let mut tops = Vec::new();
    for v in stacks {
        let c = v.pop().unwrap();
        tops.push(c);
    }
    String::from_utf8(tops).unwrap()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut lines_iter = lines.into_iter();

    let mut init_stack_lines = Vec::new();
    for _ in 0..8 {
        init_stack_lines.push(lines_iter.next().unwrap().unwrap());
    }
    lines_iter.advance_by(2).unwrap();
    let stacks = parse_init_stack(init_stack_lines);

    let mut stacks = lines_iter
        .map(|x| parse_move(x.unwrap()))
        .fold(stacks, make_move);

    let top_crates = get_tops(&mut stacks);
    format!("{top_crates}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut lines_iter = lines.into_iter();

    let mut init_stack_lines = Vec::new();
    for _ in 0..8 {
        init_stack_lines.push(lines_iter.next().unwrap().unwrap());
    }
    lines_iter.advance_by(2).unwrap();
    let stacks = parse_init_stack(init_stack_lines);

    let mut stacks = lines_iter
        .map(|x| parse_move(x.unwrap()))
        .fold(stacks, make_move_9001);

    let top_crates = get_tops(&mut stacks);
    format!("{top_crates}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_5_1() {
        let lines = read_lines("data/2022/5.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_5_2() {
        let lines = read_lines("data/2022/5.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
