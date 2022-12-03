use super::*;
use std::collections::{HashMap, HashSet};

fn read_notes(lines: io::Lines<io::BufReader<File>>) -> Vec<(Vec<String>, Vec<String>)> {
    lines
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split(" | ")
                .into_iter()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .filter(|x| x.len() == 2)
        .map(|x| {
            (
                x[0].split(" ")
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
                x[1].split(" ")
                    .into_iter()
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>(),
            )
        })
        .collect()
}

pub fn riddle_8_1(lines: io::Lines<io::BufReader<File>>) {
    let notes = read_notes(lines);
    let count = notes
        .into_iter()
        .map(|x| x.1.into_iter())
        .flatten()
        .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
        .count();
    println!("We have {} uniqe readings", count);
}

fn map_phase2(map: &mut HashMap<String, i32>, x: &str) {
    let mut rev_map = HashMap::new();
    for (k, v) in map.iter() {
        rev_map.insert(v, k.to_owned());
    }

    let mut new_map = HashMap::new();
    if x.len() == 6 {
        let mut found = false;
        if rev_map.contains_key(&4) {
            if contains_x(&rev_map[&4], x) == 4 {
                new_map.insert(x.to_owned(), 9);
                found = true;
            }
        }
        if !found && rev_map.contains_key(&1) {
            if contains_x(&rev_map[&1], x) == 2 {
                new_map.insert(x.to_owned(), 0);
                found = true;
            }
        }
        if !found {
            new_map.insert(x.to_owned(), 6);
        } 
    }
    if x.len() == 5 {
        let mut found = false;
        if rev_map.contains_key(&1) {
            if contains_x(&rev_map[&1], x)==2 {
                new_map.insert(x.to_owned(), 3);
                found = true;
            }
        }
        if rev_map.contains_key(&4) {
            if contains_x(&rev_map[&4], x)==2 {
                new_map.insert(x.to_owned(), 2);
                found = true;
            }
        }
        if !found {
            new_map.insert(x.to_owned(), 5);
        }
    }

    map.extend(new_map.into_iter().map(|(k, v)| (k.clone(), v.clone())));
}

fn contains_x(x: &str, y: &str) -> i32 {
    let mut sum = 0;
    for c in x.chars() {
        if y.contains(c) {
            sum += 1;
        }
    }
    sum
}

fn sorted_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}

fn find_mapping(note: &(Vec<String>, Vec<String>)) -> HashMap<String, i32> {
    let strings: HashSet<String> = note
        .0
        .iter()
        .chain(note.1.iter())
        .map(|x| sorted_string(x.to_string()))
        .collect();

    let mut mapping = HashMap::new();
    strings.iter().for_each(|x| {
        match x.len() {
            2 => mapping.insert(x.to_owned(), 1),
            3 => mapping.insert(x.to_owned(), 7),
            4 => mapping.insert(x.to_owned(), 4),
            7 => mapping.insert(x.to_owned(), 8),
            _ => None,
        };
    });

    strings.iter().for_each(|x| map_phase2(&mut mapping, x));

    assert_eq!(mapping.len(), 10);
    mapping
}

fn analyze_note(note: (Vec<String>, Vec<String>)) -> i32 {
    let mapping = find_mapping(&note);
    let mut num = 0;
    for s in note.1 {
        num = 10 * num + mapping[&sorted_string(s)];
    }
    num
}

pub fn riddle_8_2(lines: io::Lines<io::BufReader<File>>) {
    let notes = read_notes(lines);
    let sum: i32 = notes.into_iter().map(|x| analyze_note(x)).sum();
    println!("total sum: {:?}", sum);
}
