use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut floor = 0;
    let line = lines.next().unwrap().unwrap().to_string();
    for l in line.as_bytes() {
        match l {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
    }
    format!("{floor}")
}

pub fn riddle_2(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let mut floor = 0;
    let line = lines.next().unwrap().unwrap().to_string();
    let mut count = 1;
    for l in line.as_bytes() {
        match l {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => {}
        }
        if floor < 0 {
            break;
        }
        count += 1;
    }
    format!("{count}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2015_1_1() {
        let lines = read_lines("data/2015/1.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "232");
    }

    #[test]
    fn test_2015_1_2() {
        let lines = read_lines("data/2015/1.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1783");
    }
}

