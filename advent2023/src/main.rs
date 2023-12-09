#![feature(iter_advance_by)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day1;
//pub mod day10;
//pub mod day11;
//pub mod day12;
//pub mod day13;
//pub mod day14;
//pub mod day15;
//pub mod day16;
//pub mod day17;
//pub mod day18;
//pub mod day19;
pub mod day2;
//pub mod day20;
//pub mod day21;
//pub mod day22;
//pub mod day23;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
//pub mod day24;
//pub mod day25;

use day1::*;
//use day10::*;
//use day11::*;
//use day12::*;
//use day13::*;
//use day14::*;
//use day15::*;
//use day16::*;
//use day17::*;
//use day18::*;
//use day19::*;
use day2::*;
//use day20::*;
//use day21::*;
//use day22::*;
//use day23::*;
use day3::*;
use day4::*;
use day5::*;
use day6::*;
use day7::*;
use day8::*;
use day9::*;
//use day24::*;
//use day25::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("I expect exactly one argument (the number of the riddle, eg. 1_2 for the first days second puzzle");
        return;
    }
    let riddle = &args[1];
    if let Ok(lines) = read_lines(format!("data/input_{}.txt", riddle)) {
        match riddle.as_str() {
            "1_1" => riddle_1_1(lines),
            "1_2" => riddle_1_2(lines),
            "2_1" => riddle_2_1(lines),
            "2_2" => riddle_2_2(lines),
            "3_1" => riddle_3_1(lines),
            "3_2" => riddle_3_2(lines),
            "4_1" => riddle_4_1(lines),
            "4_2" => riddle_4_2(lines),
            "5_1" => riddle_5_1(lines),
            "5_2" => riddle_5_2(lines),
            "6_1" => riddle_6_1(lines),
            "6_2" => riddle_6_2(lines),
            "7_1" => riddle_7_1(lines),
            "7_2" => riddle_7_2(lines),
            "8_1" => riddle_8_1(lines),
            "8_2" => riddle_8_2(lines),
            "9_1" => riddle_9_1(lines),
            "9_2" => riddle_9_2(lines),
            //            "10_1" => riddle_10_1(lines),
            //            "10_2" => riddle_10_2(lines),
            //            "11_1" => riddle_11_1(lines),
            //            "11_2" => riddle_11_2(lines),
            //            "12_1" => riddle_12_1(lines),
            //            "12_2" => riddle_12_2(lines),
            //            "13_1" => riddle_13_1(lines),
            //            "13_2" => riddle_13_2(lines),
            //            "14_1" => riddle_14_1(lines),
            //            "14_2" => riddle_14_2(lines),
            //            "15_1" => riddle_15_1(lines),
            //            "15_2" => riddle_15_2(lines),
            //            "16_1" => riddle_16_1(lines),
            //            "16_2" => riddle_16_2(lines),
            //            "17_1" => riddle_17_1(lines),
            //            "17_2" => riddle_17_2(lines),
            //            "18_1" => riddle_18_1(lines),
            //            "18_2" => riddle_18_2(lines),
            //            "19_1" => riddle_19_1(lines),
            //            "19_2" => riddle_19_2(lines),
            //            "20_1" => riddle_20_1(lines),
            //            "20_2" => riddle_20_2(lines),
            //            "21_1" => riddle_21_1(lines),
            //            "21_2" => riddle_21_2(lines),
            //            "22_1" => riddle_22_1(lines),
            //            "22_2" => riddle_22_2(lines),
            //            "23_1" => riddle_23_1(lines),
            //            "23_2" => riddle_23_2(lines),
            //            "24_1" => riddle_24_1(lines),
            //            "24_2" => riddle_24_2(lines),
            //            "25_1" => riddle_25_1(lines),
            // "25_2" => riddle_25_2(lines),
            _ => println!("Solution to this riddle is not yet implemented"),
        }
    } else {
        eprintln!("Couldn't read input file data/input_{riddle}.txt");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
