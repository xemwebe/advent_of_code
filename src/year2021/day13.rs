use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
struct Instructions {
    points: Vec<(i32, i32)>,
    folds: Vec<Fold>,
}

fn fold(points: &Vec<(i32, i32)>, fold: &Fold) -> Vec<(i32, i32)> {
    let new_points: HashSet<(i32, i32)> = points
        .iter()
        .map(|xy| match fold {
            Fold::X(x) => {
                if xy.0 < *x {
                    *xy
                } else {
                    (*x - (xy.0 - *x), xy.1)
                }
            }
            Fold::Y(y) => {
                if xy.1 < *y {
                    *xy
                } else {
                    (xy.0, *y - (xy.1 - *y))
                }
            }
        })
        .collect();
    new_points.into_iter().collect()
}

fn read_instructions(lines: io::Lines<io::BufReader<File>>) -> Instructions {
    let mut instructions = Instructions {
        points: Vec::new(),
        folds: Vec::new(),
    };
    let mut read_points = true;
    for line in lines.into_iter().filter_map(|x| x.ok()) {
        if line.is_empty() {
            read_points = false;
            continue;
        }
        if read_points {
            let points: Vec<i32> = line
                .split(",")
                .into_iter()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            instructions.points.push((points[0], points[1]));
        } else {
            let fold_num: i32 = line[13..].parse().unwrap();
            if &line[11..12] == "x" {
                instructions.folds.push(Fold::X(fold_num));
            } else {
                instructions.folds.push(Fold::Y(fold_num));
            }
        }
    }
    instructions
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let instructions = read_instructions(lines);
    let points = fold(&instructions.points, &instructions.folds[0]);
    format!("{}", points.len())
}

fn print_points(points: &Vec<(i32, i32)>) {
    let mut xmax = 0;
    let mut ymax = 0;
    points.iter().for_each(|xy| {
        xmax = xmax.max(xy.0);
        ymax = xmax.max(xy.1);
    });
    let mut paper = Vec::new();
    (0..=ymax).for_each(|_| paper.push(vec![' '; (xmax + 1) as usize]));
    points
        .iter()
        .for_each(|xy| paper[xy.0 as usize][xy.1 as usize] = '#');
    for i in 0..=xmax {
        for j in 0..=ymax {
            print!("{}", paper[j as usize][i as usize]);
        }
        println!();
    }
    println!();
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let instructions = read_instructions(lines);
    let mut points = instructions.points.clone();
    for f in instructions.folds {
        points = fold(&points, &f);
    }
    print_points(&points);
    "see output above".to_string()
}
