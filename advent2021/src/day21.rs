use super::*;
use regex::Regex;
use std::collections::HashMap;

fn read_parameter(lines: io::Lines<io::BufReader<File>>) ->(i32,i32) {
    let re = Regex::new(r"Player [12] starting position: ([0-9]*)").unwrap();
    let mut line_iter =  lines.into_iter().filter_map(|x| x.ok());
    let pos1 = re.captures_iter(&line_iter.next().unwrap())
    .into_iter().map(|x| x.get(1).unwrap().as_str().parse::<i32>().unwrap() ).next().unwrap();

    let pos2 = re.captures_iter(&line_iter.next().unwrap())
    .into_iter().map(|x| x.get(1).unwrap().as_str().parse::<i32>().unwrap() ).next().unwrap();

    (pos1, pos2)
}

fn three_dices(dice_value: &mut i32) -> i32 {
    let mut total = *dice_value;
    *dice_value += 1;
    if *dice_value>100 { *dice_value = 1}
    total += *dice_value;
    *dice_value += 1;
    if *dice_value>100 { *dice_value = 1}
    total += *dice_value;
    *dice_value += 1;
    if *dice_value>100 { *dice_value = 1}
    total
}

pub fn riddle_21_1(lines: io::Lines<io::BufReader<File>>) {
    let (mut pos1, mut pos2) = read_parameter(lines);
    let mut dice_count = 0;
    let mut dice_value = 1;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        pos1 += three_dices(&mut dice_value);
        if pos1 > 10 { pos1 = (pos1-1)%10+1;}
        dice_count += 3;
        score1 += pos1;
        if score1 >=1000 { break; }
        pos2 += three_dices(&mut dice_value);
        if pos2 > 10 { pos2 = (pos2-1)%10+1;}
        dice_count += 3;
        score2 += pos2;
        if score2 >=1000 { break; }
    }
    println!("score1: {}, score2: {}, dice_count: {}, solution: {}", score1, score2, dice_count, dice_count*score1.min(score2) );
}

fn index(pos1: i32, pos2: i32, score1: i32, score2: i32) -> usize {
    ((((pos1-1) as usize)*10 + ((pos2-1) as usize))*21 + (score1 as usize))*21 + (score2 as usize)
}

pub fn solve_21_2(start_pos1: i32, start_pos2: i32) -> (usize,usize) {
    let probs = vec![1,3,6,7,6,3,1];
    let mut pos_score = vec![0;10*21*10*21];
    pos_score[index(start_pos1, start_pos2, 0, 0)] = 1;
    let mut win1: usize = 0;
    let mut win2: usize = 0;
    for _ in 0..21 {
        let mut new_pos_score = vec![0;10*21*10*21];
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for dice in 0..7 {
                    let new_pos1 = (pos1+dice+2) % 10 + 1;
                    for score1 in 0..=20 {
                        for score2 in 0..=20 {
                            let new_score1 = score1+new_pos1;
                            if new_score1 >=21 {
                                win1 += probs[dice as usize] * pos_score[index(pos1, pos2, score1, score2)];
                            } else {
                                new_pos_score[index(new_pos1, pos2, new_score1, score2)] += probs[dice as usize] * pos_score[index(pos1, pos2, score1, score2)];
                            }
                        }
                    }                   
                }
            }
        }
        pos_score = new_pos_score;
        let mut new_pos_score = vec![0;10*21*10*21];
        for pos1 in 1..=10 {
            for pos2 in 1..=10 {
                for dice in 0..7 {
                    let new_pos2 = (pos2+dice+2) % 10 + 1;
                    for score1 in 0..=20 {
                        for score2 in 0..=20 {
                            let new_score2 = score2+new_pos2;
                            if new_score2 >=21 {
                                win2 += probs[dice as usize] * pos_score[index(pos1, pos2, score1, score2)];
                            } else {
                                new_pos_score[index(pos1, new_pos2, score1, new_score2)] += probs[dice as usize] * pos_score[index(pos1, pos2, score1, score2)];
                            }
                        }
                    }
                }
            }
        }
        pos_score = new_pos_score;
    }
    (win1,win2)
}

pub fn solve_21_3(pos1: i32, pos2: i32, score1: usize, score2: usize, cache: &mut HashMap<(i32, i32, usize, usize), (usize, usize) >) -> (usize,usize) {
    let k = (pos1, pos2, score1, score2);
    if cache.contains_key(&k) {
        return cache[&k];
    }
    let mut oc = (0,0);
    let dices: Vec<usize> = vec![3,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6,7,7,7,7,7,7,8,8,8,9];
    for dice in dices {
        let mut new_pos1 = pos1 as usize + dice;
        new_pos1 = (new_pos1-1) % 10 + 1;
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
    

pub fn riddle_21_2(lines: io::Lines<io::BufReader<File>>) {
    let (start_pos1, start_pos2) = read_parameter(lines);
    //let (win1, win2) = solve_21_2(start_pos1, start_pos2);
    //println!("win1: {}, win2: {}, max {}", win1, win2, win1.max(win2));
    let mut cache = HashMap::new();
    let (win1, win2) = solve_21_3(start_pos1, start_pos2, 0, 0, &mut cache);
    println!("win1: {}, win2: {}, max {}", win1, win2, win1.max(win2));
}
