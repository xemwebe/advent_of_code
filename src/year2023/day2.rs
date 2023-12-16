use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    let mut idx = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        idx += 1;
        let game: Vec<&str> = line.split(": ").collect();
        let game_info: Vec<&str> = game[0].split(' ').collect();
        let read_idx: usize = game_info[1].parse().unwrap();
        if idx != read_idx {
            eprintln!("*** Index error");
        }
        let draws: Vec<&str> = game[1].split("; ").collect();
        let mut rbg = (0, 0, 0);
        for draw in draws {
            let cubes: Vec<&str> = draw.split(", ").collect();
            for cube in cubes {
                let c: Vec<&str> = cube.split(' ').collect();
                let count: u32 = c[0].parse().unwrap();
                match c[1] {
                    "red" => {
                        rbg.0 = rbg.0.max(count);
                    }
                    "blue" => {
                        rbg.1 = rbg.1.max(count);
                    }
                    "green" => {
                        rbg.2 = rbg.2.max(count);
                    }
                    _ => {
                        eprintln!("*** invalid color");
                    }
                }
            }
        }
        if rbg.0 <= 12 && rbg.1 <= 14 && rbg.2 <= 13 {
            sum += idx;
        }
    }
    format!("{sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    let mut idx = 0;
    for line_result in lines {
        let line = line_result.unwrap();
        idx += 1;
        let game: Vec<&str> = line.split(": ").collect();
        let game_info: Vec<&str> = game[0].split(' ').collect();
        let read_idx: usize = game_info[1].parse().unwrap();
        if idx != read_idx {
            eprintln!("*** Index error");
        }
        let draws: Vec<&str> = game[1].split("; ").collect();
        let mut rbg = (0, 0, 0);
        for draw in draws {
            let cubes: Vec<&str> = draw.split(", ").collect();
            for cube in cubes {
                let c: Vec<&str> = cube.split(' ').collect();
                let count: u32 = c[0].parse().unwrap();
                match c[1] {
                    "red" => {
                        rbg.0 = rbg.0.max(count);
                    }
                    "blue" => {
                        rbg.1 = rbg.1.max(count);
                    }
                    "green" => {
                        rbg.2 = rbg.2.max(count);
                    }
                    _ => {
                        eprintln!("*** invalid color");
                    }
                }
            }
        }
        sum += rbg.0 * rbg.1 * rbg.2;
    }
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_2_1() {
        let lines = read_lines("data/2023/2.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "2317");
    }

    #[test]
    fn test_2023_2_2() {
        let lines = read_lines("data/2023/2.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "74804");
    }
}

