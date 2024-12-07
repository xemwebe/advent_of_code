use std::{fs::File, io};

mod day1;
mod day2;
mod day3;
mod day4;

pub fn execute(day: u32, part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match day {
        1 => day1::execute(part, lines),
        2 => day2::execute(part, lines),
        3 => day3::execute(part, lines),
        4 => day4::execute(part, lines),
        _ => format!("Error: day {day} not found"),
    }
}
