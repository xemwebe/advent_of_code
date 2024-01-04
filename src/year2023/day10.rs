use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

fn find_start(map: &[Vec<u8>]) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Position {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn next(&mut self, map: &[Vec<u8>]) {
        match (map[self.x][self.y], &self.dir) {
            (b'|', Up) => {
                self.dir = Up;
                self.x -= 1;
            }
            (b'7', Up) => {
                self.dir = Left;
                self.y -= 1;
            }
            (b'F', Up) => {
                self.dir = Right;
                self.y += 1;
            }
            (b'7', Right) => {
                self.dir = Down;
                self.x += 1;
            }
            (b'-', Right) => {
                self.dir = Right;
                self.y += 1;
            }
            (b'J', Right) => {
                self.dir = Up;
                self.x -= 1;
            }
            (b'|', Down) => {
                self.dir = Down;
                self.x += 1;
            }
            (b'L', Down) => {
                self.dir = Right;
                self.y += 1;
            }
            (b'J', Down) => {
                self.dir = Left;
                self.y -= 1;
            }
            (b'L', Left) => {
                self.dir = Up;
                self.x -= 1;
            }
            (b'-', Left) => {
                self.dir = Left;
                self.y -= 1;
            }
            (b'F', Left) => {
                self.dir = Down;
                self.x += 1;
            }
            _ => {
                panic!("I am stuck")
            }
        }
    }
}

fn walk_loop(mut pos: Position, map: &[Vec<u8>]) -> u64 {
    let mut step = 1;
    while map[pos.x][pos.y] != b'S' {
        pos.next(&map);
        step += 1;
    }
    step
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = Vec::new();
    for l in lines {
        let line = l.unwrap();
        map.push(line.as_bytes().to_vec());
    }
    let start = find_start(&map);
    let pos = if [b'-', b'F', b'L'].contains(&map[start.0][start.1 - 1]) {
        Position::new(start.0, start.1 - 1, Left)
    } else if [b'|', b'F', b'7'].contains(&map[start.0 - 1][start.1]) {
        Position::new(start.0 - 1, start.1, Up)
    } else if [b'-', b'7', b'J'].contains(&map[start.0][start.1 + 1]) {
        Position::new(start.0, start.1 + 1, Right)
    } else {
        Position::new(start.0 + 1, start.1, Down)
    };
    let loop_length = walk_loop(pos, &map);
    format!("{}", loop_length / 2)
}

fn find_loop(mut pos: Position, map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut loop_map = Vec::new();
    for i in 0..map.len() {
        loop_map.push(vec![b'.'; map[i].len()]);
    }
    loop_map[pos.x][pos.y] = map[pos.x][pos.y];
    while map[pos.x][pos.y] != b'S' {
        pos.next(&map);
        loop_map[pos.x][pos.y] = map[pos.x][pos.y];
    }
    loop_map
}

fn mark(x: usize, y: usize, b: u8, map: &mut [Vec<u8>]) {
    if map[x][y] == b'.' {
        map[x][y] = b;
    }
}

fn set_marks(pos: &Position, map: &mut [Vec<u8>]) {
    let x = pos.x;
    let y = pos.y;
    match (map[x][y], &pos.dir) {
        (b'7', Right) => {
            mark(x - 1, y, b'O', map);
            mark(x - 1, y + 1, b'O', map);
            mark(x, y + 1, b'O', map);
            mark(x + 1, y - 1, b'#', map);
        }
        (b'7', Up) => {
            mark(x - 1, y, b'#', map);
            mark(x - 1, y + 1, b'#', map);
            mark(x, y + 1, b'#', map);
            mark(x + 1, y + 1, b'O', map);
        }
        (b'L', Left) => {
            mark(x + 1, y, b'O', map);
            mark(x + 1, y - 1, b'O', map);
            mark(x, y - 1, b'O', map);
            mark(x - 1, y + 1, b'#', map);
        }
        (b'L', Down) => {
            mark(x + 1, y, b'#', map);
            mark(x + 1, y - 1, b'#', map);
            mark(x, y - 1, b'#', map);
            mark(x - 1, y + 1, b'O', map);
        }
        (b'J', Down) => {
            mark(x, y + 1, b'O', map);
            mark(x + 1, y + 1, b'O', map);
            mark(x, y + 1, b'O', map);
            mark(x - 1, y - 1, b'#', map);
        }
        (b'J', Right) => {
            mark(x, y + 1, b'#', map);
            mark(x + 1, y + 1, b'#', map);
            mark(x, y + 1, b'#', map);
            mark(x - 1, y - 1, b'O', map);
        }
        (b'F', Up) => {
            mark(x, y - 1, b'O', map);
            mark(x - 1, y - 1, b'O', map);
            mark(x - 1, y, b'O', map);
            mark(x + 1, y + 1, b'#', map);
        }
        (b'F', Left) => {
            mark(x, y - 1, b'#', map);
            mark(x - 1, y - 1, b'#', map);
            mark(x - 1, y, b'#', map);
            mark(x + 1, y + 1, b'O', map);
        }
        (b'|', Up) => {
            mark(x, y - 1, b'O', map);
            mark(x, y + 1, b'#', map);
        }
        (b'|', Down) => {
            mark(x, y - 1, b'#', map);
            mark(x, y + 1, b'O', map);
        }
        (b'-', Right) => {
            mark(x - 1, y, b'O', map);
            mark(x + 1, y, b'#', map);
        }
        (b'-', Left) => {
            mark(x - 1, y, b'#', map);
            mark(x + 1, y, b'O', map);
        }
        _ => panic!("I am stuck"),
    }
}

fn mark_left_right(mut pos: Position, map: &mut [Vec<u8>]) {
    while map[pos.x][pos.y] != b'S' {
        set_marks(&pos, map);
        pos.next(&map);
    }
}

fn fill_gaps(map: &mut [Vec<u8>], mark: u8) {
    for i in 0..map.len() {
        for j in 1..map[i].len() {
            if map[i][j] == b'.' && map[i][j - 1] == mark {
                map[i][j] = mark;
            }
        }
    }
}

fn count_inner(map: &[Vec<u8>], mark: u8) -> u64 {
    let mut count = 0;
    for row in map {
        for cell in row {
            if *cell == mark {
                count += 1;
            }
        }
    }
    count
}
fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map = Vec::new();
    for l in lines {
        let line = l.unwrap();
        map.push(line.as_bytes().to_vec());
    }
    let start = find_start(&map);
    let start_pos = if [b'-', b'F', b'L'].contains(&map[start.0][start.1 - 1]) {
        Position::new(start.0, start.1 - 1, Left)
    } else if [b'|', b'F', b'7'].contains(&map[start.0 - 1][start.1]) {
        Position::new(start.0 - 1, start.1, Up)
    } else if [b'-', b'7', b'J'].contains(&map[start.0][start.1 + 1]) {
        Position::new(start.0, start.1 + 1, Right)
    } else {
        Position::new(start.0 + 1, start.1, Down)
    };
    let mut loop_map = find_loop(start_pos.clone(), &map);
    mark_left_right(start_pos, &mut loop_map);
    // check wich marker is inside
    let mut inside_marker = b'.';
    let half = loop_map.len() / 2;
    for i in 0..loop_map[half].len() {
        if loop_map[half][i] != b'.' {
            if loop_map[half][i] == b'O' {
                inside_marker = b'#';
            } else {
                inside_marker = b'O';
            }
            break;
        }
    }
    fill_gaps(&mut loop_map, inside_marker);
    let solution = count_inner(&loop_map, inside_marker);
    format!("{solution}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_10_1() {
        let lines = read_lines("data/2023/10.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "6682");
    }

    #[test]
    fn test_2023_10_2() {
        let lines = read_lines("data/2023/10.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "353");
    }
}
