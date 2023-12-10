use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn score_incorrect(s: &str) -> (usize, Vec<char>) {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.is_empty() {
                    break;
                }
                if Some('(') != stack.pop() {
                    return (3, stack);
                }
            }
            ']' => {
                if stack.is_empty() {
                    break;
                }
                if Some('[') != stack.pop() {
                    return (57, stack);
                }
            }
            '}' => {
                if stack.is_empty() {
                    break;
                }
                if Some('{') != stack.pop() {
                    return (1197, stack);
                }
            }
            '>' => {
                if stack.is_empty() {
                    break;
                }
                if Some('<') != stack.pop() {
                    return (25137, stack);
                }
            }
            _ => {
                return (0, stack);
            }
        }
    }
    (0, stack)
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>)  -> String {
    let score: usize = lines
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|s| score_incorrect(&s).0)
        .sum();

    format!("{score}")
}

pub fn score_stack(s: &[char]) -> usize {
    let mut score = 0;
    for c in s.iter().rev() {
        score = 5 * score
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };
    }
    score
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>)  -> String {
    let mut scores: Vec<usize> = lines
        .into_iter()
        .filter_map(|l| l.ok())
        .filter_map(|s| {
            let (score, stack) = score_incorrect(&s);
            match score {
                0 => Some(score_stack(&stack)),
                _ => None,
            }
        })
        .collect();

    scores.sort();
    let mid = scores.len() / 2;
    format!("{}", scores[mid])
}
