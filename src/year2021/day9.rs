use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_map(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.bytes()
                .into_iter()
                .map(|b| b - { "0".bytes().into_iter().next() }.unwrap())
                .collect::<Vec<u8>>()
        })
        .collect()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let map = read_map(lines);
    let nx = map.len();
    let ny = map[0].len();
    let mut sum = 0;
    for x in 0..nx {
        for y in 0..ny {
            let height = map[x][y];
            if x > 0 && map[x - 1][y] <= height {
                continue;
            }
            if y > 0 && map[x][y - 1] <= height {
                continue;
            }
            if x + 1 < nx && map[x + 1][y] <= height {
                continue;
            }
            if y + 1 < ny && map[x][y + 1] <= height {
                continue;
            }
            sum += height as i32 + 1;
        }
    }
    format!("{sum}")
}

fn calc_bassin(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    if map[x][y] == 9 {
        return 0;
    }
    let mut count = 1;
    map[x][y] = 9;
    if x > 0 {
        count += calc_bassin(map, x - 1, y)
    }
    if y > 0 {
        count += calc_bassin(map, x, y - 1)
    }
    if x + 1 < map.len() {
        count += calc_bassin(map, x + 1, y)
    }
    if y + 1 < map[0].len() {
        count += calc_bassin(map, x, y + 1)
    }
    count
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = read_map(lines);
    let nx = map.len();
    let ny = map[0].len();
    let mut bassins = Vec::new();
    for x in 0..nx {
        for y in 0..ny {
            let bassin = calc_bassin(&mut map, x, y);
            if bassin > 0 {
                bassins.push(bassin);
            }
        }
    }

    bassins.sort_by(|a, b| b.cmp(a));
    format!("{}", bassins[0] * bassins[1] * bassins[2])
}
