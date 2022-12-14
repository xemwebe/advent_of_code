use super::*;

fn parse_letters(s: String) -> (String, String) {
    let mut parts = s.split(" ");
    (
        parts.next().unwrap_or("").to_string(),
        parts.next().unwrap_or("").to_string(),
    )
}

fn abc_to_num(s: String) -> i32 {
    match s.as_str() {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("invalid abc input"),
    }
}

fn xyz_to_num(s: String) -> i32 {
    match s.as_str() {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("invalid xyz input"),
    }
}

// 1 stone, 2 paper, 3 scissor
fn score(x: i32, y: i32) -> i32 {
    match (x, y) {
        (1, 2) | (2, 3) | (3, 1) => 6,
        (1, 1) | (2, 2) | (3, 3) => 3,
        _ => 0,
    }
}

fn score_strat2(x: i32, y: i32) -> i32 {
    let z = match y {
        1 => (x + 1) % 3 + 1,
        2 => x,
        _ => (x + 3) % 3 + 1,
    };
    score(x, z) + z
}

pub fn riddle_2_1(lines: io::Lines<io::BufReader<File>>) {
    let total_score: i32 = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_letters)
        .map(|(x, y)| (abc_to_num(x), xyz_to_num(y)))
        .map(|(x, y)| score(x, y) + y)
        .sum();

    println!("{:?}", total_score);
}

pub fn riddle_2_2(lines: io::Lines<io::BufReader<File>>) {
    let total_score: i32 = lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(parse_letters)
        .map(|(x, y)| (abc_to_num(x), xyz_to_num(y)))
        .map(|(x, y)| score_strat2(x, y))
        .sum();

    println!("{:?}", total_score);
}
