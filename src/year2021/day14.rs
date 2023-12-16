use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::HashMap;

fn read_input(lines: io::Lines<io::BufReader<File>>) -> (String, HashMap<String, String>) {
    let mut lines = lines.into_iter().filter_map(|x| x.ok());
    let polymere = lines.next().unwrap();
    let rules: HashMap<String, String> = lines
        .skip(1)
        .map(|x| {
            let s: Vec<String> = x.split(" -> ").into_iter().map(|x| x.to_owned()).collect();
            (s[0].clone(), s[1].clone())
        })
        .collect();
    (polymere, rules)
}

fn calc_new_polymere(polymere: String, rules: &HashMap<String, String>) -> String {
    let mut new_polymere = Vec::new();
    let elements: Vec<u8> = polymere.bytes().into_iter().collect();
    for i in 0..elements.len() - 1 {
        new_polymere.push(elements[i]);
        let insertion = rules[&String::from_utf8(elements[i..=i + 1].to_vec()).unwrap()]
            .bytes()
            .next()
            .unwrap();
        new_polymere.push(insertion);
    }
    new_polymere.push(*elements.last().unwrap());
    String::from_utf8(new_polymere).unwrap()
}

fn diff_most_least(polymere: &str) -> usize {
    let mut histo = HashMap::new();
    polymere.bytes().into_iter().for_each(|x| {
        if histo.contains_key(&x) {
            *histo.get_mut(&x).unwrap() += 1;
        } else {
            histo.insert(x, 1);
        }
    });
    histo.values().max().unwrap() - histo.values().min().unwrap()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut polymere, rules) = read_input(lines);
    for _ in 0..10 {
        polymere = calc_new_polymere(polymere, &rules);
    }
    let solution = diff_most_least(&polymere);
    format!("{solution}")
}

fn read_as_vec(lines: io::Lines<io::BufReader<File>>) -> (Vec<u8>, HashMap<(u8, u8), u8>) {
    let mut lines = lines.into_iter().filter_map(|x| x.ok());
    let polymere = lines.next().unwrap().bytes().collect();
    let rules: HashMap<(u8, u8), u8> = lines
        .skip(1)
        .map(|x| {
            let s: Vec<String> = x.split(" -> ").into_iter().map(|x| x.to_owned()).collect();
            let mut b0 = s[0].bytes();
            (
                (b0.next().unwrap(), b0.next().unwrap()),
                s[1].bytes().next().unwrap(),
            )
        })
        .collect();
    (polymere, rules)
}

pub fn init_score(polymere: &[u8]) -> HashMap<(u8, u8), usize> {
    let mut score = HashMap::new();
    for i in 0..polymere.len() - 1 {
        let key = (polymere[i], polymere[i + 1]);
        if score.contains_key(&key) {
            *score.get_mut(&key).unwrap() += 1;
        } else {
            score.insert(key, 1);
        }
    }
    score
}

fn update_val(score: &mut HashMap<(u8, u8), usize>, key: (u8, u8), val: usize) {
    if let Some(entry) = score.get_mut(&key) {
        *entry += val;
    } else {
        score.insert(key, val);
    }
}

fn update_score(
    score: &HashMap<(u8, u8), usize>,
    rules: &HashMap<(u8, u8), u8>,
) -> HashMap<(u8, u8), usize> {
    let mut new_score = HashMap::new();
    for (key, val) in score {
        let key1 = (key.0, rules[key]);
        update_val(&mut new_score, key1, *val);
        let key2 = (rules[key], key.1);
        update_val(&mut new_score, key2, *val);
    }
    new_score
}

fn update_count(count: &mut HashMap<u8, usize>, key: u8, val: usize) {
    if let Some(entry) = count.get_mut(&key) {
        *entry += val;
    } else {
        count.insert(key, val);
    }
}

fn score_polymere(polymere: &[u8], score: &HashMap<(u8, u8), usize>) -> usize {
    let mut counts = HashMap::new();
    for (key, val) in score {
        update_count(&mut counts, key.0, *val);
        update_count(&mut counts, key.1, *val);
    }
    *counts.get_mut(&polymere[0]).unwrap() += 1;
    *counts.get_mut(&polymere.last().unwrap()).unwrap() += 1;

    (counts.values().max().unwrap() - counts.values().min().unwrap()) / 2
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let (polymere, rules) = read_as_vec(lines);
    let mut score = init_score(&polymere);
    for _ in 0..40 {
        score = update_score(&score, &rules);
    }
    let solution = score_polymere(&polymere, &score);
    format!("{solution}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2021_14_1() {
        let lines = read_lines("data/2021/14.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "5656");
    }

    #[test]
    fn test_2021_14_2() {
        let lines = read_lines("data/2021/14.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "12271437788530");
    }
}

