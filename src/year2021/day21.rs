use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use regex::Regex;
use std::collections::HashMap;

fn read_parameter(lines: io::Lines<io::BufReader<File>>) -> (i32, i32) {
    let re = Regex::new(r"Player [12] starting position: ([0-9]*)").unwrap();
    let mut line_iter = lines.into_iter().filter_map(|x| x.ok());
    let pos1 = re
        .captures_iter(&line_iter.next().unwrap())
        .into_iter()
        .map(|x| x.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .next()
        .unwrap();

    let pos2 = re
        .captures_iter(&line_iter.next().unwrap())
        .into_iter()
        .map(|x| x.get(1).unwrap().as_str().parse::<i32>().unwrap())
        .next()
        .unwrap();

    (pos1, pos2)
}

fn three_dices(dice_value: &mut i32) -> i32 {
    let mut total = *dice_value;
    *dice_value += 1;
    if *dice_value > 100 {
        *dice_value = 1
    }
    total += *dice_value;
    *dice_value += 1;
    if *dice_value > 100 {
        *dice_value = 1
    }
    total += *dice_value;
    *dice_value += 1;
    if *dice_value > 100 {
        *dice_value = 1
    }
    total
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut pos1, mut pos2) = read_parameter(lines);
    let mut dice_count = 0;
    let mut dice_value = 1;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        pos1 += three_dices(&mut dice_value);
        if pos1 > 10 {
            pos1 = (pos1 - 1) % 10 + 1;
        }
        dice_count += 3;
        score1 += pos1;
        if score1 >= 1000 {
            break;
        }
        pos2 += three_dices(&mut dice_value);
        if pos2 > 10 {
            pos2 = (pos2 - 1) % 10 + 1;
        }
        dice_count += 3;
        score2 += pos2;
        if score2 >= 1000 {
            break;
        }
    }
    format!("{}", dice_count * score1.min(score2))
}

pub fn solve(
    pos1: i32,
    pos2: i32,
    score1: usize,
    score2: usize,
    cache: &mut HashMap<(i32, i32, usize, usize), (usize, usize)>,
) -> (usize, usize) {
    let k = (pos1, pos2, score1, score2);
    if cache.contains_key(&k) {
        return cache[&k];
    }
    let mut oc = (0, 0);
    let dices: Vec<usize> = vec![
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];
    for dice in dices {
        let mut new_pos1 = pos1 as usize + dice;
        new_pos1 = (new_pos1 - 1) % 10 + 1;
        let new_score1 = score1 + new_pos1 as usize;
        if new_score1 >= 21 {
            oc.0 += 1;
        } else {
            let (dy, dx) = solve(pos2, new_pos1 as i32, score2, new_score1, cache);
            oc.0 += dx;
            oc.1 += dy;
        }
    }
    cache.insert(k, oc);
    oc
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let (start_pos1, start_pos2) = read_parameter(lines);
    let mut cache = HashMap::new();
    let (win1, win2) = solve(start_pos1, start_pos2, 0, 0, &mut cache);
    format!("{}", win1.max(win2))
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2021_21_1() {
        let lines = read_lines("data/2021/21.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "1002474");
    }

    #[test]
    fn test_2021_21_2() {
        let lines = read_lines("data/2021/21.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "919758187195363");
    }
}
