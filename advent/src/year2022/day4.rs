use super::*;

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

pub fn riddle_4_1(lines: io::Lines<io::BufReader<File>>) {
    let total_overlaps: usize = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_ranges)
        .filter(do_fully_overlap)
        .count();

    println!("{:?}", total_overlaps);
}

pub fn riddle_4_2(lines: io::Lines<io::BufReader<File>>) {
    let total_overlaps: usize = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_ranges)
        .filter(do_overlap)
        .count();

    println!("{:?}", total_overlaps);
}
