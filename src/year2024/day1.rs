use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split("   ");
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    format!("{sum}")
}

pub fn count_frequency(l: i32, right: &Vec<i32>) -> i32 {
    let mut count = 0;
    for r in right {
        if l == *r {
            count += 1;
        }
    }
    count
}

pub fn riddle_2(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split("   ");
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    let mut sum = 0;
    for l in left {
        let count = count_frequency(l, &right);
        sum += l * count;
    }
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_1_1() {
        let lines = read_lines("data/2024/1.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "2742123");
    }

    #[test]
    fn test_2024_1_2() {
        let lines = read_lines("data/2024/1.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "21328497");
    }
}
