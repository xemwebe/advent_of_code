use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

const DIGITS: usize = 12;

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let numbers: Vec<i32> = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .filter_map(|s| i32::from_str_radix(&s, 2).ok())
        .collect();
    numbers
}

fn calc_counts(numbers: &[i32]) -> Vec<usize> {
    let mut counts = vec![0; DIGITS];
    for n in numbers {
        let mut num = *n;
        for b in 0..DIGITS {
            if num & 1 == 1 {
                counts[b] += 1;
            }
            num >>= 1;
        }
    }
    counts
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);

    let counts = calc_counts(&numbers);

    let total = numbers.len();

    let half = total / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for b in 0..DIGITS {
        gamma <<= 1;
        epsilon <<= 1;
        if counts[DIGITS - b - 1] > half {
            gamma += 1;
        } else if counts[DIGITS - b - 1] < half {
            epsilon += 1;
        } else {
            panic!("Warning: undetermined!");
        }
    }
    format!(
        "gamma: {}, epsilon: {}, power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}

fn filter_nums(numbers: &Vec<i32>, bit_criteria: bool) -> Option<i32> {
    let mut numbers = numbers.clone();
    for b in (0..DIGITS).rev() {
        let bit = (1 << b) as i32;
        let counts = calc_counts(&numbers);
        let half = (numbers.len() as f64) / 2.0;
        let valid_bit = (if bit_criteria && (counts[b] as f64) >= half {
            bit
        } else if !bit_criteria && (counts[b] as f64) < half {
            bit
        } else {
            0
        }) as i32;
        let new_numbers: Vec<i32> = numbers
            .into_iter()
            .filter(|x| x & bit == valid_bit)
            .collect();
        if new_numbers.len() == 1 {
            return Some(new_numbers[0]);
        }
        numbers = new_numbers;
    }
    None
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);

    let oxygen = filter_nums(&numbers, true).unwrap();
    let carbon = filter_nums(&numbers, false).unwrap();
    format!(
        "oxygen: {}, carbon: {}, life: {}",
        oxygen,
        carbon,
        oxygen * carbon
    )
}
