use std::{
    fs::File, 
    io,
    str::FromStr
};

#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

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

impl FromStr for Direction {
    type Err=ParseDirectionError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "R" => Ok(Right),
            "L" => Ok(Left),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(ParseDirectionError)
        }
    }
}

impl Direction {
    fn next_pos(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Up => (x-1, y),
            Down => (x+1, y),
            Right => (x, y+1),
            Left => (x, y-1)
        }
    }

    fn next_n_pos(&self, x: i64, y: i64, n: i64) -> (i64, i64) {
        match self {
            Up => (x-n, y),
            Down => (x+n, y),
            Right => (x, y+n),
            Left => (x, y-n)
        }
    }

    fn from_num(b: u8) -> Self {
        match b {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("invalid direction")
        }
    }
 }

fn fill_map(map: &mut Vec<Vec<u8>>, directions: &[(Direction, u8)]) -> u32 {
    let mut x = 250;
    let mut y = 250;
    map[x][y] = b'#';
    let mut min_x = x;
    let mut min_y = y;
    let mut max_x = x;
    let mut max_y = y;

    for dir in directions {
        for _ in 0..dir.1 {
            (x, y) = dir.0.next_pos(x,y);
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            map[x][y] = b'#';
        }
    }

    let mut inner = false;
    let mut maximum = false;
    let mut minimum = false;
    let mut area = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if map[x][y] == b'#' {
                area += 1;
                if !maximum && !minimum {
                    if map[x+1][y] == b'#' && map[x-1][y] != b'#' {
                        maximum = true;
                    }
                    if map[x+1][y] != b'#' && map[x-1][y] == b'#' {
                        minimum = true;
                    }
                    if map[x+1][y] == b'#' && map[x-1][y] == b'#' {
                        inner = !inner;
                    }
                } else if maximum {
                    if map[x+1][y] == b'#' && map[x-1][y] != b'#' {
                        maximum = false;
                    } else if map[x+1][y] != b'#' && map[x-1][y] == b'#' {
                        inner = !inner;
                        maximum = false;
                    }
                } else if minimum {
                    if map[x+1][y] != b'#' && map[x-1][y] == b'#' {
                        minimum = false;
                    } else if map[x+1][y] == b'#' && map[x-1][y] != b'#' {
                        inner = !inner;
                        minimum = false;
                    }
                }
            } else {
                if inner {
                    area += 1;
                    map[x][y] = b'I';
                }
            }
        }
    }
    area
}

fn fill_map2(map: &mut [Vec<u8>], directions: &[(Direction, u32)]) -> i64 {
    let mut min_x = 0i64;
    let mut min_y = 0i64;
    let mut max_x = 0i64;
    let mut max_y = 0i64;
    let mut x = 0i64;
    let mut y = 0i64;
    let mut first = true;
    for dir in directions {
        let n = if first { first = false; dir.1 as i64 + 1} else { dir.1 as i64};
        let (x,y) = dir.0.next_n_pos(x, y, n);
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    println!("x range: {min_x}…{max_x} y range: {min_y}…{max_y}");
    (max_x-min_x)*(max_y-min_y)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut directions = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        directions.push((Direction::from_str(&parts[0]).unwrap(), parts[1].parse::<u8>().unwrap()));
    }
    let mut map = vec![vec![b'.'; 500]; 500];
    let area = fill_map(&mut map, &directions);
    format!("{area}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut directions = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let num = u32::from_str_radix(&parts[2][2..7], 16).unwrap();
        directions.push((Direction::from_num(parts[2].as_bytes()[7]-b'0'), num));
    }
    let mut map = vec![vec![b'.'; 100000]; 100000];
    let area = fill_map2(&mut map, &directions);
    format!("{area}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_18_1() {
        let lines = read_lines("data/2023/18.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "45159");
    }

    #[test]
    fn test_2023_18_2() {
        let lines = read_lines("data/2023/18.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "353");
    }
}

