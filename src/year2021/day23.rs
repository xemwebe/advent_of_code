use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::cmp::Ordering;
use std::collections::HashMap;

fn read_game(lines: io::Lines<io::BufReader<File>>, n: usize) -> Vec<(u8, u8)> {
    let map: Vec<Vec<char>> = lines
        .into_iter()
        .filter_map(|x| x.ok())
        .skip(1)
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut positions = vec![(0, 0); 4 * n];
    let mut counta = 0;
    let mut countb = 0;
    let mut countc = 0;
    let mut countd = 0;
    for i in 1..=n {
        for j in 0..4 {
            let col = j * 2 + 3;
            match map[i][col] {
                'A' => {
                    positions[counta] = (col as u8, i as u8);
                    counta += 1;
                }
                'B' => {
                    positions[n + countb] = (col as u8, i as u8);
                    countb += 1;
                }
                'C' => {
                    positions[2 * n + countc] = (col as u8, i as u8);
                    countc += 1;
                }
                'D' => {
                    positions[3 * n + countd] = (col as u8, i as u8);
                    countd += 1;
                }
                _ => panic!("Unexpected kind of Amphipod"),
            };
        }
    }
    positions
}

#[derive(Debug, Clone, Eq, Hash)]
struct Game {
    positions: Vec<(u8, u8)>,
}

impl Game {
    fn n(&self) -> u8 {
        self.positions.len() as u8 / 4
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        for (i, p) in (&self.positions).iter().enumerate() {
            if *p != other.positions[i] {
                return false;
            }
        }
        true
    }
}

fn pair_cmp(i: (u8, u8), j: (u8, u8)) -> Ordering {
    if i.0 == j.0 {
        i.1.cmp(&j.1)
    } else {
        i.0.cmp(&j.0)
    }
}

fn slice_cmp(first: &[(u8, u8)], second: &[(u8, u8)]) -> Ordering {
    if first.len() != second.len() {
        panic!("unequal length in slice compare");
    } else if first.len() == 1 {
        pair_cmp(first[0], second[0])
    } else {
        slice_cmp(&first[1..], &second[1..])
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        slice_cmp(&self.positions, &other.positions)
    }
}

fn is_finished(game: &Game) -> bool {
    let mut finished = true;
    for (i, p) in (&game.positions).iter().enumerate() {
        if p.0 != get_target(i, game.n()) {
            finished = false;
            break;
        }
    }
    finished
}

fn who_is_at(x: u8, y: u8, game: &Game) -> Option<usize> {
    for (i, p) in (&game.positions).iter().enumerate() {
        if *p == (x, y) {
            return Some(i);
        }
    }
    None
}

fn is_right(x: u8, y: u8, game: &Game) -> bool {
    let n = game.n();
    let max_y = n as u8;
    if y > max_y {
        return true;
    }
    if y == 0 {
        return false;
    }
    if let Some(i) = who_is_at(x, y, game) {
        if game.positions[i].0 == get_target(i, n) && is_right(x, y + 1, game) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn move_out(i: usize, game: &Game) -> Vec<(Game, i32)> {
    let mut moves = Vec::new();
    let (ax, ay) = game.positions[i];
    if is_right(ax, ay, game) {
        return moves;
    }
    let mut minx = 1;
    let mut maxx = 11;
    for x in 1..12 {
        if who_is_at(x, 0, game).is_some() {
            if x < ax {
                minx = x + 1;
            } else if x > ax {
                maxx = x - 1;
                break;
            }
        }
    }
    let n = game.n();
    for x in minx..=maxx {
        if x == 3 || x == 5 || x == 7 || x == 9 {
            continue;
        }
        if ay != 0 || ax != x {
            let mut positions = game.positions.clone();
            positions[i] = (x, 0);
            let move_energy = if x < ax {
                energy(i, n) * (ax - x + ay) as i32
            } else {
                energy(i, n) * (x - ax + ay) as i32
            };
            moves.push((
                Game {
                    positions: positions,
                },
                move_energy,
            ));
        }
    }
    moves
}

fn get_target(i: usize, n: u8) -> u8 {
    (i as u8 / n) * 2 + 3
}

fn move_in(i: usize, game: &Game) -> Option<(Game, i32)> {
    let (ax, ay) = game.positions[i];
    let n = game.n();
    if is_right(ax, ay, game) {
        return None;
    }
    let target = get_target(i, n);
    let walk = if ax < target {
        (ax + 1..=target).collect::<Vec<u8>>()
    } else {
        (target..ax).rev().collect::<Vec<u8>>()
    };
    for x in walk {
        if !who_is_at(x, 0, game).is_none() {
            return None;
        }
    }
    let mut new_y = 0;
    while new_y < n && who_is_at(target, new_y + 1, game).is_none() {
        new_y += 1;
    }
    if new_y == 0 {
        return None;
    }
    if new_y == n || is_right(target, new_y + 1, game) {
        let mut positions = game.positions.clone();
        positions[i] = (target, new_y);
        return Some((
            Game { positions },
            energy(i, n)
                * (if target > ax {
                    target - ax
                } else {
                    ax - target
                } + new_y
                    + ay) as i32,
        ));
    }
    None
}

fn energy(i: usize, n: u8) -> i32 {
    10_i32.pow((i as u32) / n as u32)
}

fn get_list_of_moves(game: &Game) -> Vec<(Game, i32)> {
    let mut moves = Vec::new();
    for (i, p) in (&game.positions).iter().enumerate() {
        let (ax, ay) = *p;
        if is_right(ax, ay, game) {
            continue;
        }
        if ay > 1 {
            let mut can_move = true;
            for y in 1..ay {
                can_move = can_move && who_is_at(ax, y, game).is_none();
            }
            if !can_move {
                continue;
            }
        }
        if let Some(m) = move_in(i, game) {
            moves.push(m);
        } else if ay > 0 {
            moves.append(&mut move_out(i, game));
        }
    }
    moves
}

fn solve(game: &Game, total_energy: i32, solutions: &mut HashMap<Game, i32>) -> i32 {
    if is_finished(game) {
        solutions.insert(game.clone(), total_energy);
        return total_energy;
    }
    if solutions.contains_key(game) {
        return solutions[game];
    }
    let moves = get_list_of_moves(&game);
    let mut min_energy = i32::MAX;
    for m in &moves {
        let curr_energy = solve(&m.0, m.1 + total_energy, solutions);
        solutions.insert(m.0.clone(), curr_energy);
        min_energy = min_energy.min(curr_energy);
    }
    min_energy
}

fn print_moves(moves: &Vec<(Game, i32)>) {
    for m in moves {
        let n = m.0.n();
        for y in 0..=n {
            for x in 0..13 {
                if let Some(i) = who_is_at(x, y, &m.0) {
                    if i < n as usize {
                        print!("A");
                    } else if i < 2 * n as usize {
                        print!("B");
                    } else if i < 3 * n as usize {
                        print!("C");
                    } else {
                        print!("D");
                    }
                } else {
                    if y > 0 && (x % 2) == 0 && x > 1 && x < 11 {
                        print!("#")
                    } else if y > 1 && (x < 2 || x > 10) {
                        print!(" ");
                    } else if y == 1 && (x < 2 || x > 10) {
                        print!("#");
                    } else if y == 0 && (x == 0 || x == 12) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!("  #########\n");
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let game = Game {
        positions: read_game(lines, 2),
    };
    let mut solutions = HashMap::new();
    format!("{}", solve(&game, 0, &mut solutions))
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let game = Game {
        positions: read_game(lines, 4),
    };
    print_moves(&vec![(game.clone(), 0)]);
    let mut solutions = HashMap::new();
    format!("{}", solve(&game, 0, &mut solutions))
}
