use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    dist: usize,
    val: u8,
}

fn read_grid(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<Node>> {
    let zero = "0".bytes().next().unwrap();

    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| {
            x.bytes()
                .map(|x| Node {
                    dist: usize::MAX,
                    val: x - zero,
                })
                .collect::<Vec<Node>>()
        })
        .collect()
}

fn update_cell(grid: &mut Vec<Vec<Node>>, x: usize, y: usize) -> bool {
    let mut has_changed = false;
    let my_dist = grid[y][x].dist;
    if x > 0 {
        if grid[y][x - 1].dist > grid[y][x - 1].val as usize + my_dist {
            grid[y][x - 1].dist = grid[y][x - 1].val as usize + my_dist;
            has_changed = true
        }
    }
    if x + 1 < grid[0].len() {
        if grid[y][x + 1].dist > grid[y][x + 1].val as usize + my_dist {
            grid[y][x + 1].dist = grid[y][x + 1].val as usize + my_dist;
            has_changed = true
        }
    }
    if y > 0 {
        if grid[y - 1][x].dist > grid[y - 1][x].val as usize + my_dist {
            grid[y - 1][x].dist = grid[y - 1][x].val as usize + my_dist;
            has_changed = true
        }
    }
    if y + 1 < grid.len() {
        if grid[y + 1][x].dist > grid[y + 1][x].val as usize + my_dist {
            grid[y + 1][x].dist = grid[y + 1][x].val as usize + my_dist;
            has_changed = true
        }
    }
    has_changed
}

fn low_risk_path(grid: &mut Vec<Vec<Node>>) {
    let xlen = grid[0].len();

    grid[0][0].dist = 0;
    let mut has_changed = true;
    while has_changed {
        has_changed = false;
        for y in 0..grid.len() {
            for x in 0..xlen {
                has_changed |= update_cell(grid, x, y);
            }
        }
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = read_grid(lines);
    low_risk_path(&mut grid);
    format!("{}", grid.last().unwrap().last().unwrap().dist)
}

fn increas_val(grid: &mut Vec<Vec<Node>>) {
    for v in grid.iter_mut() {
        for n in v.iter_mut() {
            if n.val == 9 {
                n.val = 1;
            } else {
                n.val += 1;
            }
        }
    }
}

fn enlarge_grid(grid: &mut Vec<Vec<Node>>) {
    let mut tmp_grid = grid.clone();
    let ylen = tmp_grid.len();
    for d in 1..=8 {
        increas_val(&mut tmp_grid);
        let xxmin = if d > 4 { d - 4 } else { 0 };
        for xx in xxmin..=d.min(4) {
            if xx == 0 {
                for v in tmp_grid.iter() {
                    grid.push(v.clone())
                }
            } else {
                let yy = (d - xx) * ylen;
                for y in 0..tmp_grid.len() {
                    grid[y + yy].extend(tmp_grid[y].iter())
                }
            }
        }
    }
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut grid = read_grid(lines);
    enlarge_grid(&mut grid);
    low_risk_path(&mut grid);
    format!("{}", grid.last().unwrap().last().unwrap().dist)
}
