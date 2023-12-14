use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn row_equal(i: usize, j: usize, grid: &[Vec<u8>]) -> bool {
    for k in 0..grid[i].len() {
        if grid[i][k] != grid[j][k] {
            return false;
        }
    }
    true
}

fn col_equal(i: usize, j: usize, grid: &[Vec<u8>]) -> bool {
    for row in grid {
        if row[i] != row[j] {
            return false;
        }
    }
    true
}

fn calc_score(grid: &[Vec<u8>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows - 1 {
        if row_equal(i, i + 1, grid) {
            let width = i.min(rows - i - 2);
            println!("width: {width}, i: {i}");
            let mut is_mirror = true;
            for w in 1..=width {
                if !row_equal(i - w, i + w + 1, grid) {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                return (i + 1) * 100;
            }
        }
    }
    for i in 0..cols - 1 {
        if col_equal(i, i + 1, grid) {
            let width = i.min(cols - i - 2);
            let mut is_mirror = true;
            for w in 1..=width {
                if !col_equal(i - w, i + w + 1, grid) {
                    is_mirror = false;
                    break;
                }
            }
            if is_mirror {
                return i + 1;
            }
        }
    }
    panic!("no mirror found!");
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    let mut grid = Vec::new();
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            sum += calc_score(&grid);
            grid.clear();
        } else {
            grid.push(line.trim().as_bytes().to_vec());
        }
    }
    if !grid.is_empty() {
        sum += calc_score(&grid);
    }
    format!("{sum}")
}

fn row_equal_with_error(i: usize, j: usize, grid: &[Vec<u8>]) -> u32 {
    let mut errors = 0;
    for k in 0..grid[i].len() {
        if grid[i][k] != grid[j][k] {
            errors += 1;
        }
    }
    errors
}

fn col_equal_with_error(i: usize, j: usize, grid: &[Vec<u8>]) -> u32 {
    let mut errors = 0;
    for row in grid {
        if row[i] != row[j] {
            errors += 1;
        }
    }
    errors
}

fn calc_score_with_smudge(grid: &[Vec<u8>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    for i in 0..rows - 1 {
        let mut errors = row_equal_with_error(i, i + 1, grid);
        if errors <= 1 {
            let width = i.min(rows - i - 2);
            for w in 1..=width {
                errors += row_equal_with_error(i - w, i + w + 1, grid);
                if errors > 1 {
                    break;
                }
            }
            if errors == 1 {
                return (i + 1) * 100;
            }
        }
    }
    for i in 0..cols - 1 {
        let mut errors = col_equal_with_error(i, i + 1, grid);
        if errors <= 1 {
            let width = i.min(cols - i - 2);
            for w in 1..=width {
                errors += col_equal_with_error(i - w, i + w + 1, grid);
                if errors > 1 {
                    break;
                }
            }
            if errors == 1 {
                return i + 1;
            }
        };
    }
    panic!("no mirror found!");
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    let mut grid = Vec::new();
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            sum += calc_score_with_smudge(&grid);
            grid.clear();
        } else {
            grid.push(line.trim().as_bytes().to_vec());
        }
    }
    if grid.len() > 0 {
        sum += calc_score_with_smudge(&grid);
    }
    format!("{sum}")
}
