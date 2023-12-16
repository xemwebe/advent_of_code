use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers: Vec<i32> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    let mut max = 0;
    let mut current = 0;
    for n in numbers {
        if n == 0 {
            max = max.max(current);
            current = 0;
        } else {
            current += n;
        }
    }
    format!("{max}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers: Vec<i32> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect();

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current = 0;
    for n in numbers {
        if n == 0 {
            if current > third {
                if current > first {
                    third = second;
                    second = first;
                    first = current;
                } else if current > second {
                    third = second;
                    second = current;
                } else {
                    third = current;
                }
            }
            current = 0;
        } else {
            current += n;
        }
    }
    format!("{}", first + second + third)
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2022_1_1() {
        let lines = read_lines("data/2022/1.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "69912");
    }

    #[test]
    fn test_2022_1_2() {
        let lines = read_lines("data/2022/1.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "208180");
    }
}