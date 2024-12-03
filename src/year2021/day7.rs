use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let number_str = lines.into_iter().next().unwrap().unwrap();
    number_str
        .split(",")
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    for x in min..=max {
        let fuel = numbers.iter().map(|h| (h - x).abs()).sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    format!("{}", min_fuel)
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    for x in min..=max {
        let fuel = numbers
            .iter()
            .map(|h| {
                let n = (h - x).abs();
                n * (n + 1) / 2
            })
            .sum();
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    format!("{}", min_fuel)
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2021_7_1() {
        let lines = read_lines("data/2021/7.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "343468");
    }

    #[test]
    fn test_2021_7_2() {
        let lines = read_lines("data/2021/7.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "96086265");
    }
}
