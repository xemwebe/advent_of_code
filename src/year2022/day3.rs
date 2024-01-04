use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn byte_to_priority(byte: u8) -> i32 {
    if byte >= 97 {
        (byte - 96) as i32
    } else {
        (byte - 38) as i32
    }
}

fn find_double(bytes: Vec<u8>) -> u8 {
    let len = bytes.len();
    let mid = len / 2;
    for i in mid..len {
        for j in 0..mid {
            if bytes[i] == bytes[j] {
                return bytes[i];
            }
        }
    }
    0
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_priorities: i32 = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.into_bytes())
        .map(find_double)
        .map(byte_to_priority)
        .sum();

    format!("{total_priorities}")
}

fn group_priority(one: &[u8], two: &[u8], three: &[u8]) -> i32 {
    for i in 0..one.len() {
        for j in 0..two.len() {
            if one[i] == two[j] {
                for k in 0..three.len() {
                    if one[i] == three[k] {
                        return byte_to_priority(one[i]);
                    }
                }
            }
        }
    }
    0
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let all_rucksacks: Vec<Vec<u8>> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.into_bytes())
        .collect();

    let mut sum = 0;
    for i in 0..all_rucksacks.len() / 3 {
        let idx = i * 3;
        sum += group_priority(
            &all_rucksacks[idx],
            &all_rucksacks[idx + 1],
            &all_rucksacks[idx + 2],
        );
    }

    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_3_1() {
        let lines = read_lines("data/2022/3.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7597");
    }

    #[test]
    fn test_2022_3_2() {
        let lines = read_lines("data/2022/3.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "2607");
    }
}
