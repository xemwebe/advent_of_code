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
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut last = numbers[0];
    let count = numbers
        .iter()
        .filter(|x| {
            let res = **x > last;
            last = **x;
            res
        })
        .count();
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers: Vec<i32> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut sum = numbers[0] + numbers[1] + numbers[2];
    let mut count = 0;
    for i in 1..numbers.len() - 2 {
        let new_sum = sum + numbers[i + 2] - numbers[i - 1];
        if new_sum > sum {
            count += 1;
        }
        sum = new_sum;
    }
    format!("{count}")
}
