use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let number_str = lines.into_iter().next().unwrap().unwrap();
    number_str
        .split(",")
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn collect_in_histo(numbers: &[i32]) -> Vec<usize> {
    let mut histo: Vec<usize> = vec![0; 9];
    numbers.into_iter().for_each(|n| histo[*n as usize] += 1);
    histo
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);
    let mut hist = collect_in_histo(&numbers);
    for _ in 0..80 {
        let new_fish = hist[0];
        for j in 1..9 {
            hist[j - 1] = hist[j];
        }
        hist[8] = new_fish;
        hist[6] += new_fish;
    }
    let total_sum: usize = hist.into_iter().sum();
    format!("{total_sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let numbers = read_numbers(lines);
    let mut hist = collect_in_histo(&numbers);
    for _ in 0..256 {
        let new_fish = hist[0];
        for j in 1..9 {
            hist[j - 1] = hist[j];
        }
        hist[8] = new_fish;
        hist[6] += new_fish;
    }
    let total_sum: usize = hist.into_iter().sum();
    format!("{total_sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2021_6_1() {
        let lines = read_lines("data/2021/6.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "362639");
    }

    #[test]
    fn test_2021_6_2() {
        let lines = read_lines("data/2021/6.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1639854996917");
    }
}
