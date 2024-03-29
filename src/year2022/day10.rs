use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug)]
enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn parse(s: String) -> Self {
        let mut parts = s.split(" ");
        if parts.next().unwrap() == "noop" {
            Command::Noop
        } else {
            Command::Addx(parts.next().unwrap().parse().unwrap())
        }
    }

    fn cycles(&self) -> i32 {
        match &self {
            Command::Noop => 1,
            Command::Addx(_) => 2,
        }
    }

    fn add_on(&self) -> i32 {
        match &self {
            Command::Noop => 0,
            Command::Addx(x) => *x,
        }
    }
}

fn process(vc: &Vec<Command>, sc: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut x = 1;
    let mut cycle = 0;
    let mut sc_idx = 0;
    for c in vc {
        let duration = c.cycles();
        if cycle + duration >= sc[sc_idx] {
            sum += sc[sc_idx] * x;
            sc_idx += 1;
            if sc_idx >= sc.len() {
                break;
            }
        }
        cycle += duration;
        x += c.add_on();
    }
    sum
}

fn draw_pixel(cycle: i32, x: i32) {
    let pos = cycle % 40;
    if pos - 1 <= x && pos + 1 >= x {
        print!("#");
    } else {
        print!(".");
    }
    if pos == 39 {
        println!("");
    }
}

fn draw_screen(vc: &Vec<Command>) {
    let mut x = 1;
    let mut cycle = 0;
    for c in vc {
        match c {
            Command::Noop => draw_pixel(cycle, x),
            Command::Addx(y) => {
                draw_pixel(cycle, x);
                cycle += 1;
                draw_pixel(cycle, x);
                x += y;
            }
        }
        cycle += 1;
    }
}

fn parse_commands(lines: io::Lines<io::BufReader<File>>) -> Vec<Command> {
    lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(Command::parse)
        .collect()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let commands = parse_commands(lines);
    let signal = process(&commands, vec![20, 60, 100, 140, 180, 220]);
    format!("{signal}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let commands = parse_commands(lines);
    draw_screen(&commands);
    "no single value output".to_string()
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_10_1() {
        let lines = read_lines("data/2022/10.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_10_2() {
        let lines = read_lines("data/2022/10.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
