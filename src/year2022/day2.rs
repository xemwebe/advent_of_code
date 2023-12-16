use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn parse_letters(s: String) -> (String, String) {
    let mut parts = s.split(' ');
    (
        parts.next().unwrap_or("").to_string(),
        parts.next().unwrap_or("").to_string(),
    )
}

fn abc_to_num(s: String) -> i32 {
    match s.as_str() {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("invalid abc input"),
    }
}

fn xyz_to_num(s: String) -> i32 {
    match s.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("invalid xyz input"),
    }
}

// 1 stone, 2 paper, 3 scissor
fn score(x: i32, y: i32) -> i32 {
    match (x, y) {
        (1, 2) | (2, 3) | (3, 1) => 6,
        (1, 1) | (2, 2) | (3, 3) => 3,
        _ => 0,
    }
}

fn score_strat2(x: i32, y: i32) -> i32 {
    let z = match y {
        1 => (x + 1) % 3 + 1,
        2 => x,
        _ => (x + 3) % 3 + 1,
    };
    score(x, z) + z
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_score: i32 = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_letters)
        .map(|(x, y)| (abc_to_num(x), xyz_to_num(y)))
        .map(|(x, y)| score(x, y) + y)
        .sum();

    format!("{total_score}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_score: i32 = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_letters)
        .map(|(x, y)| (abc_to_num(x), xyz_to_num(y)))
        .map(|(x, y)| score_strat2(x, y))
        .sum();

    format!("{total_score}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2022_2_1() {
        let lines = read_lines("data/2022/2.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "13009");
    }

    #[test]
    fn test_2022_2_2() {
        let lines = read_lines("data/2022/2.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "10398");
    }
}