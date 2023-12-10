use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_grid(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .map(|s| {
            s.unwrap()
                .as_bytes()
                .into_iter()
                .map(|x| *x - 48)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn find_invisible(grid: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    let xdim = grid[0].len();
    let ydim = grid.len();
    for x in 1..xdim - 1 {
        for y in 1..ydim - 1 {
            let mut invisible = false;
            let height = grid[y][x];
            for i in 0..x {
                if height <= grid[y][i] {
                    invisible = true;
                    break;
                }
            }
            if !invisible {
                continue;
            }
            invisible = false;
            for i in 0..y {
                if height <= grid[i][x] {
                    invisible = true;
                    break;
                }
            }
            if !invisible {
                continue;
            }
            invisible = false;
            for i in x + 1..xdim {
                if height <= grid[y][i] {
                    invisible = true;
                    break;
                }
            }
            if !invisible {
                continue;
            }
            invisible = false;
            for i in y + 1..ydim {
                if height <= grid[i][x] {
                    invisible = true;
                    break;
                }
            }
            if invisible {
                count += 1;
            }
        }
    }
    xdim * ydim - count
}

fn calc_max_scenic_score(grid: &Vec<Vec<u8>>) -> usize {
    let mut score = 0;
    let xdim = grid[0].len();
    let ydim = grid.len();
    for x in 1..xdim - 1 {
        for y in 1..ydim - 1 {
            let height = grid[y][x];

            let mut left = 0;
            for i in (0..x).rev() {
                if height >= grid[y][i] {
                    left += 1;
                }
                if height <= grid[y][i] {
                    break;
                }
            }

            let mut top = 0;
            for i in (0..y).rev() {
                if height >= grid[i][x] {
                    top += 1;
                }
                if height <= grid[i][x] {
                    break;
                }
            }

            let mut right = 0;
            for i in x + 1..xdim {
                if height >= grid[y][i] {
                    right += 1;
                }
                if height <= grid[y][i] {
                    break;
                }
            }

            let mut bottom = 0;
            for i in y + 1..ydim {
                if height >= grid[i][x] {
                    bottom += 1;
                }
                if height <= grid[i][x] {
                    break;
                }
            }
            score = score.max(left * right * top * bottom);
        }
    }
    score
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let grid = read_grid(lines);
    let count = find_invisible(&grid);
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let grid = read_grid(lines);
    let score = calc_max_scenic_score(&grid);
    format!("{score}")
}
