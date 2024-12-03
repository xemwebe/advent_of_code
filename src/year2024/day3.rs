use regex::Regex;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut sum: i32 = 0;
    for line in lines {
        let line = line.unwrap();
        sum += re
            .find_iter(&line)
            .map(|x| x.as_str())
            .map(|x| &x[4..x.len() - 1])
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .product::<i32>()
            })
            .sum::<i32>();
    }
    format!("{sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut sum: i32 = 0;
    let mut mul_active = true;
    for line in lines {
        let line = line.unwrap();
        let v: Vec<&str> = re.find_iter(&line).map(|x| x.as_str()).collect();
        for s in v {
            if s == "do()" {
                mul_active = true;
            } else if s == "don't()" {
                mul_active = false;
            } else if mul_active {
                sum += s[4..s.len() - 1]
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .product::<i32>();
            }
        }
    }
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_3_1() {
        let lines = read_lines("data/2024/3.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "173785482");
    }

    #[test]
    fn test_2024_3_2() {
        let lines = read_lines("data/2024/3.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "83158140");
    }
}
