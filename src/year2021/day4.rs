use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::HashSet;

fn split_in_numbers(s: String, delim: &str) -> Vec<i32> {
    let nums: Vec<i32> = s
        .split(delim)
        .into_iter()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    nums
}

fn board_solved(board: &[(i32, bool)]) -> Option<i32> {
    for i in 0..5 {
        let mut found_row_false = false;
        let mut found_col_false = false;
        for j in 0..5 {
            if !board[i * 5 + j].1 {
                found_row_false = true;
            }
            if !board[i + j * 5].1 {
                found_col_false = true;
            }
            if found_row_false && found_col_false {
                break;
            }
        }
        if !found_row_false || !found_col_false {
            return Some(score_board(board));
        }
    }
    None
}

fn score_board(board: &[(i32, bool)]) -> i32 {
    board.iter().map(|x| if !x.1 { x.0 } else { 0 }).sum()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut lines_iter = lines.into_iter();
    let draws_string = lines_iter.next().unwrap().unwrap();
    let draws: Vec<i32> = split_in_numbers(draws_string, ",");

    let mut numbers: Vec<(i32, bool)> = lines_iter
        .filter_map(|x| x.ok())
        .filter(|s| !s.is_empty())
        .map(|s| split_in_numbers(s, " ").into_iter())
        .flatten()
        .map(|s| (s, false))
        .collect();

    let mut solutions = Vec::new();
    let num_boards = numbers.len() / 25;

    for x in draws {
        numbers.iter_mut().for_each(|n| {
            if n.0 == x {
                n.1 = true
            }
        });
        for i in 0..num_boards {
            if let Some(score) = board_solved(&numbers[i * 25..(i + 1) * 25]) {
                solutions.push((i, score * x));
            }
        }
        if !solutions.is_empty() {
            break;
        }
    }

    format!("{:?}", solutions.last().unwrap().1)
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut lines_iter = lines.into_iter();
    let draws_string = lines_iter.next().unwrap().unwrap();
    let draws: Vec<i32> = split_in_numbers(draws_string, ",");

    let mut numbers: Vec<(i32, bool)> = lines_iter
        .filter_map(|x| x.ok())
        .filter(|s| !s.is_empty())
        .map(|s| split_in_numbers(s, " ").into_iter())
        .flatten()
        .map(|s| (s, false))
        .collect();

    let mut solutions = Vec::new();
    let num_boards = numbers.len() / 25;
    let mut solved = HashSet::new();

    for (round, x) in draws.into_iter().enumerate() {
        numbers.iter_mut().for_each(|n| {
            if n.0 == x {
                n.1 = true
            }
        });
        for i in 0..num_boards {
            if !solved.contains(&i) {
                let board = &numbers[i * 25..(i + 1) * 25];
                if let Some(score) = board_solved(board) {
                    solutions.push((round, i, score * x));
                    solved.insert(i);
                }
            }
        }
    }

    format!("{:?}", solutions.last().unwrap().2)
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2021_4_1() {
        let lines = read_lines("data/2021/4.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "5685");
    }

    #[test]
    fn test_2021_4_2() {
        let lines = read_lines("data/2021/4.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "21070");
    }
}

