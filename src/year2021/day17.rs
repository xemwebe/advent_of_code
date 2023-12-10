use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use regex::Regex;

fn read_parameter(lines: io::Lines<io::BufReader<File>>) -> ((i32, i32), (i32, i32)) {
    let line = lines.into_iter().filter_map(|x| x.ok()).next().unwrap();
    let re = Regex::new(r"target area: x=([-0-9]*)..([-0-9]*), y=([-0-9]*)..([-0-9]*)").unwrap();
    let numbers: Vec<i32> = re
        .captures_iter(&line)
        .next()
        .unwrap()
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
        .collect();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

fn simulate(start_vx: i32, start_vy: i32, target: ((i32, i32), (i32, i32))) -> i32 {
    let mut probe = target.1 .0 - 1;
    let mut vx = start_vx;
    let mut vy = start_vy;
    let mut pos = (0, 0);
    for _ in 0..10000 {
        pos.0 += vx;
        pos.1 += vy;
        probe = probe.max(pos.1);
        if pos.0 > target.0 .1 || pos.1 < target.1 .0 {
            return target.1 .0 - 1;
        }
        if pos.0 >= target.0 .0 && pos.1 <= target.1 .1 {
            return probe;
        }
        vx = (vx - 1).max(0);
        vy -= 1;
    }
    target.1 .0 - 1
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>)  -> String {
    let target = read_parameter(lines);
    let min_vx = (-0.5 + (2.0 * (target.0 .0 as f64) + 0.25)).sqrt().floor() as i32;
    let max_vx = target.0 .1;
    let mut height = 0;
    for start_vx in min_vx..=max_vx {
        for start_vy in 0..1000 {
            height = height.max(simulate(start_vx, start_vy, target));
        }
    }
    format!("{height}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>)  -> String {
    let target = read_parameter(lines);
    let min_vx = (-0.5 + (2.0 * (target.0 .0 as f64) + 0.25)).sqrt().floor() as i32;
    let max_vx = target.0 .1;
    let mut count = 0;
    for start_vx in min_vx..=max_vx {
        for start_vy in target.1 .0..1000 {
            if simulate(start_vx, start_vy, target) >= target.1 .0 {
                count += 1;
            }
        }
    }
    format!("{count}")
}
