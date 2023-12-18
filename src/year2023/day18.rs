use std::{
    fs::File, 
    io,
    str::FromStr,
    collections::BTreeSet,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point  {
    x: i64,
    y: i64,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HorizontalLine {
    x: i64,
    y_start: i64,
    y_end: i64,
}

impl PartialOrd for HorizontalLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HorizontalLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x == other.x {
            if self.y_start == other.y_start {
                self.y_end.cmp(&other.y_end)
            } else {
                self.y_start.cmp(&other.y_start)
            }
        } else {
            self.x.cmp(&other.x)
        }
    }
}

fn fill_map2(directions: &[(Direction, u32)]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut lines = Vec::new();
    for dir in directions {
        let (x_new, y_new) = dir.0.next_n_pos(x, y, dir.1 as i64);
        if dir.0 == Right || dir.0 == Left  {
            lines.push(HorizontalLine{x, y_start: y.min(y_new), y_end: y.max(y_new)});
        }
        x = x_new;
        y = y_new;
    }
    lines.sort();
    // for l in lines.iter() {
    //     println!("{:?}", l);
    // }
    
    let mut line_area = 0;
    x = lines[0].x;
    let mut current_lines = BTreeSet::new();
    let mut i = 0;
    while lines[i].x == x {
        let line = Point{x: lines[i].y_start, y: lines[i].y_end };
        line_area += line.y - line.x + 1;
        current_lines.insert(line);
        i += 1;
    }

    let mut area = 0;
    for line in lines[i..].iter() {
        if line.x != x {
            area += (line.x-x) * line_area;
            x = line.x;
        }
        let mut line_replacement = None;
        for l in &current_lines {
            if l.x == line.y_end || l.y == line.y_start {
                line_area += line.y_end - line.y_start;
                let l_x = l.x.min(line.y_start);
                let l_y = l.y.max(line.y_end);
                line_replacement = Some((l.clone(), Some(Point{x: l_x, y: l_y}), None));
                break;
            } else if l.x == line.y_start || l.y == line.y_end {
                line_area -= line.y_end - line.y_start;
                area += line.y_end - line.y_start;
                if l.x == line.y_start && l.y == line.y_end {
                    line_replacement = Some((l.clone(), None, None));
                } else if l.x == line.y_start {
                    line_replacement = Some((l.clone(), Some(Point{x: line.y_end, y: l.y}), None));
                } else {
                    line_replacement = Some((l.clone(), Some(Point{ x: l.x, y: line.y_start}), None));
                }
            } else if l.x < line.y_start && l.y > line.y_end {
                line_area -= line.y_end - line.y_start - 1;
                area += line.y_end - line.y_start - 1;
                line_replacement = Some((l.clone(), Some(Point{x: l.x, y: line.y_start}), Some(Point{x: line.y_end, y: l.y})))
            }
        }
        //println!("x: {x:?}, line: {line:?}, replacement: {line_replacement:?}, area: {area}, line_area: {line_area}");
        if let Some(line_replacement) = line_replacement {
            current_lines.remove(&line_replacement.0);
            if let Some(l) = line_replacement.1 {
                current_lines.insert(l);
            }
            if let Some(l) = line_replacement.2 {
                current_lines.insert(l);
            }
        } else {
            line_area += line.y_end - line.y_start +1;
            current_lines.insert( Point{ x: line.y_start, y: line.y_end });
        }
    }
    println!("current_lines {current_lines:?}");
    area += line_area;

    area
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
    let area = fill_map2(&directions);
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

