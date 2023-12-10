use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn test_num(i: usize, j: usize, chart: &[Vec<u8>]) -> (usize, usize) {
    let mut num = 0;
    let mut k = j;
    while chart[i][k].is_ascii_digit() {
        num = num * 10 + (chart[i][k] - b'0') as usize;
        k += 1;
    }
    for l in j - 1..=k {
        if chart[i - 1][l] != b'.' && !chart[i - 1][l].is_ascii_digit() {
            return (k, num);
        }
        if chart[i + 1][l] != b'.' && !chart[i + 1][l].is_ascii_digit() {
            return (k, num);
        }
    }
    if chart[i][j - 1] != b'.' && !chart[i][j - 1].is_ascii_digit() {
        return (k, num);
    }
    if chart[i][k] != b'.' && !chart[i][k].is_ascii_digit() {
        return (k, num);
    }
    (k, 0)
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut chart = vec![vec![0u8; 1]];

    for line_result in lines {
        chart.push(format!(".{}.", line_result.unwrap()).as_bytes().to_vec());
    }
    let xsize = chart[1].len();
    chart[0] = vec![b'.'; xsize];
    chart.push(vec![b'.'; xsize]);
    let mut sum = 0;
    for i in 1..chart.len() - 1 {
        let mut j = 1;
        while j < chart[i].len() - 1 {
            if chart[i][j].is_ascii_digit() {
                let (k, num) = test_num(i, j, &chart);
                j = k;
                sum += num;
            } else {
                j += 1;
            }
        }
    }
    format!("{sum}")
}

#[derive(Debug)]
struct NumInfo {
    next: usize,
    num: usize,
}

fn read_num(i: usize, j: usize, chart: &[Vec<u8>]) -> NumInfo {
    let mut j_start = j - 1;
    while chart[i][j_start].is_ascii_digit() {
        j_start -= 1;
    }
    j_start += 1;
    let mut k = j_start;
    let mut num = 0;
    while chart[i][k].is_ascii_digit() {
        num = num * 10 + (chart[i][k] - b'0') as usize;
        k += 1;
    }
    NumInfo { next: k, num }
}

fn get_gear(i: usize, j: usize, chart: &[Vec<u8>]) -> usize {
    let mut gear = 1;
    let mut gears_found = 0;
    let mut k = j - 1;
    while k <= j + 1 {
        if chart[i - 1][k].is_ascii_digit() {
            let info = read_num(i - 1, k, chart);
            gears_found += 1;
            k = info.next;
            gear *= info.num;
        } else {
            k += 1;
        }
    }
    let mut k = j - 1;
    while k <= j + 1 {
        if chart[i + 1][k].is_ascii_digit() {
            let info = read_num(i + 1, k, chart);
            gears_found += 1;
            k = info.next;
            gear *= info.num;
        } else {
            k += 1;
        }
    }
    if chart[i][j - 1].is_ascii_digit() {
        let info = read_num(i, j - 1, chart);
        gears_found += 1;
        gear *= info.num;
    }
    if chart[i][j + 1].is_ascii_digit() {
        let info = read_num(i, j + 1, chart);
        gears_found += 1;
        gear *= info.num;
    }
    if gears_found == 2 {
        gear
    } else {
        0
    }
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut chart = vec![vec![0u8; 1]];

    for line_result in lines {
        chart.push(format!(".{}.", line_result.unwrap()).as_bytes().to_vec());
    }
    let xsize = chart[1].len();
    chart[0] = vec![b'.'; xsize];
    chart.push(vec![b'.'; xsize]);
    let mut sum = 0;
    for i in 1..chart.len() - 1 {
        for j in 0..chart[i].len() - 1 {
            if chart[i][j] == b'*' {
                sum += get_gear(i, j, &chart);
            }
        }
    }
    format!("{sum}")
}
