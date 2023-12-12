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
    for l in lines {
        let dims: Vec<u64> = l
            .unwrap()
            .split('x')
            .map(|s| str::parse(s).unwrap())
            .collect();
        let a = dims[0] * dims[1];
        let b = dims[1] * dims[2];
        let c = dims[2] * dims[0];
        sum += 2 * (a + b + c) + a.min(b).min(c);
    }
    format!("{sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    for l in lines {
        let dims: Vec<u64> = l
            .unwrap()
            .split('x')
            .map(|s| str::parse(s).unwrap())
            .collect();
        let v = dims[0] * dims[1] * dims[2];
        let b = (dims[0] + dims[1])
            .min(dims[1] + dims[2])
            .min(dims[2] + dims[0]);
        sum += 2 * b + v;
    }
    format!("{sum}")
}
