use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_map(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.bytes().collect::<Vec<u8>>())
        .collect()
}

fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        println!("{}", String::from_utf8((*row).clone()).unwrap());
    }
}

// 'v' = 118
// '>' = 62
// '.' = 46
fn one_step(map: &Vec<Vec<u8>>) -> (bool, Vec<Vec<u8>>) {
    let rows = map.len();
    let cols = map[0].len();
    let mut new_map = map.clone();
    let mut has_changed = false;
    // Move '>'
    for i in 0..rows {
        for j in 0..cols {
            if map[i][j] == 62 {
                let next_j = if j + 1 == cols { 0 } else { j + 1 };
                if map[i][next_j] == 46 {
                    new_map[i][j] = 46;
                    new_map[i][next_j] = 62;
                    has_changed = true;
                }
            }
        }
    }
    // Move '>'
    let mut final_map = new_map.clone();
    for i in 0..rows {
        for j in 0..cols {
            if new_map[i][j] == 118 {
                let next_i = if i + 1 == rows { 0 } else { i + 1 };
                if new_map[next_i][j] == 46 {
                    final_map[i][j] = 46;
                    final_map[next_i][j] = 118;
                    has_changed = true;
                }
            }
        }
    }
    (has_changed, final_map)
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = read_map(lines);
    let mut count = 1;
    loop {
        let (has_changed, new_map) = one_step(&map);
        if !has_changed {
            break;
        }
        count += 1;
        map = new_map;
    }
    print_map(&map);
    format!("{}", count)
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2021_25_1() {
        let lines = read_lines("data/2021/25.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "295");
    }
}
