use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;
use std::collections::HashMap;

fn index(pos1: i32, pos2: i32, score1: i32, score2: i32) -> usize {
    ((((pos1 - 1) as usize) * 10 + ((pos2 - 1) as usize)) * 21 + (score1 as usize)) * 21
        + (score2 as usize)
}

pub fn solve_21_2(start_pos1: i32, start_pos2: i32) -> (usize, usize) {
    let probs = vec![1, 3, 6, 7, 6, 3, 1];
    let mut pos_score = vec![0; 10 * 21 * 10 * 21];
    pos_score[index(start_pos1, start_pos2, 0, 0)] = 1;
    let mut win1: usize = 0;
    let mut win2: usize = 0;
    for _ in 0..21 {
        let mut new_pos_score = vec![0; 10 * 21 * 10 * 21];
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for dice in 0..7 {
                    let new_pos1 = (pos1 + dice + 2) % 10 + 1;
                    for score1 in 0..=20 {
                        for score2 in 0..=20 {
                            let new_score1 = score1 + new_pos1;
                            if new_score1 >= 21 {
                                win1 += probs[dice as usize]
                                    * pos_score[index(pos1, pos2, score1, score2)];
                            } else {
                                new_pos_score[index(new_pos1, pos2, new_score1, score2)] += probs
                                    [dice as usize]
                                    * pos_score[index(pos1, pos2, score1, score2)];
                            }
                        }
                    }
                }
            }
        }
        pos_score = new_pos_score;
        let mut new_pos_score = vec![0; 10 * 21 * 10 * 21];
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for dice in 0..7 {
                    let new_pos2 = (pos2 + dice + 2) % 10 + 1;
                    for score1 in 0..=20 {
                        for score2 in 0..=20 {
                            let new_score2 = score2 + new_pos2;
                            if new_score2 >= 21 {
                                win2 += probs[dice as usize]
                                    * pos_score[index(pos1, pos2, score1, score2)];
                            } else {
                                new_pos_score[index(pos1, new_pos2, score1, new_score2)] += probs
                                    [dice as usize]
                                    * pos_score[index(pos1, pos2, score1, score2)];
                            }
                        }
                    }
                }
            }
        }
        pos_score = new_pos_score;
    }
    (win1, win2)
}

pub fn solve_21_3(
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
            let (dy, dx) = solve_21_3(pos2, new_pos1 as i32, score2, new_score1, cache);
            oc.0 += dx;
            oc.1 += dy;
        }
    }
    cache.insert(k, oc);
    oc
}

fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("day 21.2", |b| {
        b.iter(|| solve_21_2(black_box(5), black_box(6)))
    });
}

fn criterion_benchmark2(c: &mut Criterion) {
    let mut cache = HashMap::new();
    c.bench_function("day 21.3", |b| {
        b.iter(|| solve_21_3(black_box(5), black_box(6), 0, 0, &mut cache))
    });
}

criterion_group!(benches, criterion_benchmark1, criterion_benchmark2);
criterion_main!(benches);
