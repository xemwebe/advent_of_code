use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

pub fn read_map(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<u8>> {
    lines
        .into_iter()
        .filter_map(|s| s.ok())
        .map(|s| s.into_bytes())
        .collect()
}

fn find_position(v: u8, m: &Vec<Vec<u8>>) -> (usize, usize) {
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == v {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn find_path(map: &mut Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut last_positions = vec![start];
    let xdim = map[0].len();
    let ydim = map.len();
    let mut dijtra = Vec::new();
    for _ in 0..ydim {
        dijtra.push(vec![-1; xdim])
    }
    dijtra[start.1][start.0] = 0;
    let mut step = 1;
    loop {
        let mut new_positions = Vec::new();
        for p in last_positions {
            let v = map[p.1][p.0] + 1;
            if p.1 > 0 && dijtra[p.1 - 1][p.0] == -1 {
                if map[p.1 - 1][p.0] <= v {
                    dijtra[p.1 - 1][p.0] = step;
                    new_positions.push((p.0, p.1 - 1))
                }
            }
            if p.1 < ydim - 1 && dijtra[p.1 + 1][p.0] == -1 {
                if map[p.1 + 1][p.0] <= v {
                    dijtra[p.1 + 1][p.0] = step;
                    new_positions.push((p.0, p.1 + 1))
                }
            }
            if p.0 > 0 && dijtra[p.1][p.0 - 1] == -1 {
                if map[p.1][p.0 - 1] <= v {
                    dijtra[p.1][p.0 - 1] = step;
                    new_positions.push((p.0 - 1, p.1))
                }
            }
            if p.0 < xdim - 1 && dijtra[p.1][p.0 + 1] == -1 {
                if map[p.1][p.0 + 1] <= v {
                    dijtra[p.1][p.0 + 1] = step;
                    new_positions.push((p.0 + 1, p.1))
                }
            }
        }
        if dijtra[end.1][end.0] >= 0 {
            return step;
        }
        step += 1;
        last_positions = new_positions;
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = read_map(lines);
    let start = find_position(83, &map);
    let end = find_position(69, &map);
    map[start.1][start.0] = 97;
    map[end.1][end.0] = 122;

    let path_length = find_path(&mut map, start, end);
    format!("{path_length}")
}

fn shortest_path(map: &mut Vec<Vec<u8>>, start: (usize, usize)) -> i32 {
    let mut last_positions = vec![start];
    let xdim = map[0].len();
    let ydim = map.len();
    let mut dijtra = Vec::new();
    for _ in 0..ydim {
        dijtra.push(vec![-1; xdim])
    }
    dijtra[start.1][start.0] = 0;
    let mut step = 1;
    loop {
        let mut new_positions = Vec::new();
        for p in last_positions {
            let v = map[p.1][p.0] - 1;
            if p.1 > 0 && dijtra[p.1 - 1][p.0] == -1 {
                if map[p.1 - 1][p.0] >= v {
                    if map[p.1 - 1][p.0] == 97 {
                        return step;
                    }
                    dijtra[p.1 - 1][p.0] = step;
                    new_positions.push((p.0, p.1 - 1))
                }
            }
            if p.1 < ydim - 1 && dijtra[p.1 + 1][p.0] == -1 {
                if map[p.1 + 1][p.0] >= v {
                    if map[p.1 + 1][p.0] == 97 {
                        return step;
                    }
                    dijtra[p.1 + 1][p.0] = step;
                    new_positions.push((p.0, p.1 + 1))
                }
            }
            if p.0 > 0 && dijtra[p.1][p.0 - 1] == -1 {
                if map[p.1][p.0 - 1] >= v {
                    if map[p.1][p.0 - 1] == 97 {
                        return step;
                    }
                    dijtra[p.1][p.0 - 1] = step;
                    new_positions.push((p.0 - 1, p.1))
                }
            }
            if p.0 < xdim - 1 && dijtra[p.1][p.0 + 1] == -1 {
                if map[p.1][p.0 + 1] >= v {
                    if map[p.1][p.0 + 1] == 97 {
                        return step;
                    }
                    dijtra[p.1][p.0 + 1] = step;
                    new_positions.push((p.0 + 1, p.1))
                }
            }
        }
        step += 1;
        last_positions = new_positions;
    }
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = read_map(lines);
    let start = find_position(83, &map);
    let end = find_position(69, &map);
    map[start.1][start.0] = 97;
    map[end.1][end.0] = 122;

    let path_length = shortest_path(&mut map, end);
    format!("{path_length}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_12_1() {
        let lines = read_lines("data/2022/12.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_12_2() {
        let lines = read_lines("data/2022/12.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}
