use std::{fs::File, io};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
        12 => day12::execute(part, lines),
        13 => day13::execute(part, lines),
        14 => day14::execute(part, lines),
        15 => day15::execute(part, lines),
        16 => day16::execute(part, lines),
        17 => day17::execute(part, lines),
        18 => day18::execute(part, lines),
        19 => day19::execute(part, lines),
        20 => day20::execute(part, lines),
        21 => day21::execute(part, lines),
        22 => day22::execute(part, lines),
        23 => day23::execute(part, lines),
        24 => day24::execute(part, lines),
        25 => day24::execute(part, lines),
        _ => format!("Error: day {day} not found"),
    }
}
