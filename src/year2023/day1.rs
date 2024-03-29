use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn get_num(bytes: &[u8]) -> i32 {
    let mut num = 0;
    for b in bytes {
        if (*b).is_ascii_digit() {
            num = 10 * (b - b'0');
            break;
        }
    }
    for i in (0..bytes.len()).rev() {
        if bytes[i].is_ascii_digit() {
            return (num + bytes[i] - b'0').into();
        }
    }
    0
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for line in lines {
        let num = get_num(line.unwrap().as_bytes());
        sum += num;
    }
    format!("{sum}")
}

fn is_digit2(i: usize, b: &[u8]) -> Option<u8> {
    let digits: Vec<Vec<u8>> = vec![
        b"zero".to_vec(),
        b"one".to_vec(),
        b"two".to_vec(),
        b"three".to_vec(),
        b"four".to_vec(),
        b"five".to_vec(),
        b"six".to_vec(),
        b"seven".to_vec(),
        b"eight".to_vec(),
        b"nine".to_vec(),
    ];

    if b[i].is_ascii_digit() {
        return Some(b[i] - b'0');
    }
    for (j, digit) in digits.iter().enumerate().take(10) {
        let di = digit.len();
        if i + di > b.len() {
            continue;
        }
        if b[i..i + di] == digits[j] {
            return Some(j as u8);
        }
    }
    None
}

fn get_num2(bytes: &[u8]) -> i32 {
    let mut num = 0;
    for i in 0..bytes.len() {
        if let Some(d) = is_digit2(i, bytes) {
            num = 10 * d;
            break;
        }
    }
    for i in (0..bytes.len()).rev() {
        if let Some(d) = is_digit2(i, bytes) {
            return (num + d).into();
        }
    }
    0
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for line in lines {
        let num = get_num2(line.unwrap().as_bytes());
        sum += num;
    }
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_1_1() {
        let lines = read_lines("data/2023/1.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "54951");
    }

    #[test]
    fn test_2023_1_2() {
        let lines = read_lines("data/2023/1.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "55218");
    }
}
