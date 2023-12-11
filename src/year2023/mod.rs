use std::{fs::File, io};

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn execute(day: u32, part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match day {
        1 => day1::execute(part, lines),
        2 => day2::execute(part, lines),
        3 => day3::execute(part, lines),
        4 => day4::execute(part, lines),
        5 => day5::execute(part, lines),
        6 => day6::execute(part, lines),
        7 => day7::execute(part, lines),
        8 => day8::execute(part, lines),
        9 => day9::execute(part, lines),
        10 => day10::execute(part, lines),
        11 => day11::execute(part, lines),
        _ => format!("Error: day {day} not found"),
    }
}
