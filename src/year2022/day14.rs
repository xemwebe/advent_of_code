use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn read_map(
    mut lines: io::Lines<io::BufReader<File>>,
    variant2: bool,
) -> (Vec<Vec<u8>>, usize) {
    let mut rocks = Vec::new();
    let mut minx = u16::MAX;
    let mut maxx = 0;
    let mut maxy = 0;
    while let Some(Ok(s)) = lines.next() {
        let mut parts = s.split(" -> ");
        let mut points = Vec::new();
        while let Some(sub) = parts.next() {
            let mut coords = sub.split(",");
            let point = (
                coords.next().unwrap().parse::<u16>().unwrap(),
                coords.next().unwrap().parse::<u16>().unwrap(),
            );
            minx = minx.min(point.0);
            maxx = maxx.max(point.0);
            maxy = maxy.max(point.1);
            points.push(point)
        }
        rocks.push(points);
    }
    let offset = if variant2 { 0 } else { minx - 1 };
    let sand = (500 - offset) as usize;
    let lenx = if variant2 {
        1000
    } else {
        (maxx - minx + 3) as usize
    };
    let mut map = Vec::new();
    for _ in 0..=maxy {
        map.push(vec![0; lenx]);
    }
    if variant2 {
        map.push(vec![0; lenx]);
        map.push(vec![8; lenx]);
    }
    for rock in rocks {
        for i in 1..rock.len() {
            let sx = (rock[i - 1].0 - offset) as usize;
            let sy = rock[i - 1].1 as usize;
            let ex = (rock[i].0 - offset) as usize;
            let ey = rock[i].1 as usize;
            for x in sx.min(ex)..=ex.max(sx) {
                for y in sy.min(ey)..=ey.max(sy) {
                    map[y][x] = 8;
                }
            }
        }
    }
    map[0][sand] = 1;

    (map, sand)
}

fn print_map(m: &Vec<Vec<u8>>) {
    for line in m {
        for p in line {
            match p {
                8 => print!("#"),
                1 => print!("+"),
                2 => print!("o"),
                _ => print!("."),
            }
        }
        println!();
    }
}

fn simulate_sand(map: &mut Vec<Vec<u8>>, sand: usize) -> i32 {
    let mut sand_point = (sand, 0);
    let maxy = map.len() - 1;
    let mut count = 0;
    loop {
        if sand_point.1 == maxy || map[0][sand] != 1 {
            break;
        }
        match map[sand_point.1 + 1][sand_point.0] {
            0 => sand_point.1 += 1,
            2 | 8 => {
                if map[sand_point.1 + 1][sand_point.0 - 1] == 0 {
                    sand_point = (sand_point.0 - 1, sand_point.1 + 1);
                } else if map[sand_point.1 + 1][sand_point.0 + 1] == 0 {
                    sand_point = (sand_point.0 + 1, sand_point.1 + 1);
                } else {
                    map[sand_point.1][sand_point.0] = 2;
                    sand_point = (sand, 0);
                    count += 1;
                }
            }
            _ => {}
        }
    }
    map[sand_point.1][sand_point.0] = 2;
    count
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut map, sand) = read_map(lines, false);
    let count = simulate_sand(&mut map, sand);
    print_map(&map);
    format!("{count}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut map, sand) = read_map(lines, true);
    let count = simulate_sand(&mut map, sand);
    print_map(&map);
    format!("{count}")
}
