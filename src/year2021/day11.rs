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
        .filter_map(|l| l.ok())
        .map(|s| {
            s.bytes()
                .map(|x| x - "0".bytes().next().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn flash(i: i32, j: i32, grid: &mut Vec<Vec<u8>>) {
    (i - 1..=i + 1)
        .map(|x| (j - 1..=j + 1).map(|y| (x, y)).collect::<Vec<(i32, i32)>>())
        .flatten()
        .filter(|xy| xy.0 >= 0 && xy.1 >= 0 && xy.0 < 10 && xy.1 < 10 && !(xy.0 == i && xy.1 == j))
        .map(|xy| (xy.0 as usize, xy.1 as usize))
        .for_each(|xy| {
            if grid[xy.0][xy.1] > 0 {
                grid[xy.0][xy.1] += 1
            }
        });
}

fn one_step(grid: &mut Vec<Vec<u8>>) -> i32 {
    grid.iter_mut()
        .for_each(|x| x.iter_mut().for_each(|x| *x += 1));
    let mut total_flashes = 0;
    let mut flashes = 0;
    loop {
        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 {
                    flashes += 1;
                    grid[i][j] = 0;
                    flash(i as i32, j as i32, grid);
                }
            }
        }
        total_flashes += flashes;
        if flashes == 0 {
            break;
        }
        flashes = 0;
    }
    total_flashes
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>)  -> String {
    let mut grid = read_grid(lines);
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += one_step(&mut grid);
    }

    format!("{total_flashes}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>)  -> String {
    let mut grid = read_grid(lines);
    let mut i = 0;
    loop {
        i += 1;
        if one_step(&mut grid) == 100 {
            break;
        }
    }

    format!("{i}")
}
