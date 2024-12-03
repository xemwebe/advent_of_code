use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn parse_range(s: &str) -> (u32, u32) {
    let mut parts = s.split("-");
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn parse_ranges(s: String) -> ((u32, u32), (u32, u32)) {
    let mut parts = s.split(",");
    (
        parse_range(parts.next().unwrap()),
        parse_range(parts.next().unwrap()),
    )
}

fn do_fully_overlap((x, y): &((u32, u32), (u32, u32))) -> bool {
    if (x.0 <= y.0 && x.1 >= y.1) || (x.0 >= y.0 && x.1 <= y.1) {
        true
    } else {
        false
    }
}

fn do_overlap((x, y): &((u32, u32), (u32, u32))) -> bool {
    if x.1 < y.0 || x.0 > y.1 {
        false
    } else {
        true
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_overlaps: usize = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_ranges)
        .filter(do_fully_overlap)
        .count();

    format!("{total_overlaps}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let total_overlaps: usize = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_ranges)
        .filter(do_overlap)
        .count();

    format!("{total_overlaps}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_4_1() {
        let lines = read_lines("data/2022/4.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "595");
    }

    #[test]
    fn test_2022_4_2() {
        let lines = read_lines("data/2022/4.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "952");
    }
}
