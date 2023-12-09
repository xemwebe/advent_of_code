use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers: Vec<i32> = lines
        .into_iter()
        .map_while(Result::ok)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for (idx, i) in numbers.iter().enumerate() {
        for j in numbers[idx + 1..].iter() {
            if i + j == 2020 {
                return format!("{}", i * j);
            }
        }
    }
    "No solution found".to_string()
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers: Vec<i32> = lines
        .into_iter()
        .map_while(Result::ok)
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    for (idx, i) in numbers.iter().enumerate() {
        for (jdx, j) in numbers[idx + 1..].iter().enumerate() {
            for k in numbers[idx + jdx + 1..].iter() {
                if i + j + k == 2020 {
                    return format!("{}", i * j * k);
                }
            }
        }
    }
    "No solution found".to_string()
}
