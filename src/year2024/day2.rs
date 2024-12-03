use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report = line
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(report);
    }
    let mut count = 0;
    for report in reports {
        let diff = report[0] - report[1];
        let sign = diff.signum();
        if diff.abs() == 0 || diff.abs() > 3 {
            continue;
        }
        let mut is_valid = true;
        for i in 1..report.len() - 1 {
            let diff = report[i] - report[i + 1];
            if diff.signum() != sign || diff.abs() > 3 {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            count += 1;
        }
    }
    format!("{count}")
}

fn check_defect(report: &[i32], ignore: usize) -> bool {
    let first = if ignore == 0 { 1 } else { 0 };
    let second = if ignore == 0 || ignore == 1 { 2 } else { 1 };
    let diff = report[first] - report[second];
    let sign = diff.signum();
    if diff.abs() == 0 || diff.abs() > 3 {
        return false;
    }
    for i in second..report.len() - 1 {
        if i == ignore {
            continue;
        }
        let nexti = if i + 1 == ignore { i + 2 } else { i + 1 };
        if nexti == report.len() {
            break;
        };
        let diff = report[i] - report[nexti];
        if diff.signum() != sign || diff.abs() > 3 {
            return false;
        }
    }
    return true;
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let report = line
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        reports.push(report);
    }
    let mut count = 0;
    for report in reports {
        let mut is_valid = false;
        if check_defect(&report, report.len()) {
            is_valid = true;
        } else {
            for i in 0..report.len() {
                if check_defect(&report, i) {
                    is_valid = true;
                    break;
                }
            }
        }
        if is_valid {
            count += 1;
        }
    }
    format!("{count}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_2_1() {
        let lines = read_lines("data/2024/2.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "524");
    }

    #[test]
    fn test_2024_2_2() {
        let lines = read_lines("data/2024/2.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "569");
    }
}
