use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn is_valid(i: usize, v: &[u8], count: usize) -> bool {
    for j in 0..count {
        for k in j + 1..count {
            if v[i - j] == v[i - k] {
                return false;
            }
        }
    }
    true
}

fn find_start(s: &str, count: usize) -> usize {
    let v = s.as_bytes();
    for i in count..v.len() {
        if is_valid(i, v, count) {
            return i + 1;
        }
    }
    return 0;
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let input = lines.into_iter().next().unwrap().unwrap();
    let start = find_start(&input, 4);
    format!("{start}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let input = lines.into_iter().next().unwrap().unwrap();
    let start = find_start(&input, 14);
    format!("{start}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_6_1() {
        let lines = read_lines("data/2022/6.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_6_2() {
        let lines = read_lines("data/2022/6.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
