use std::{collections::BTreeSet, fs::File, io, str::FromStr};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "R" => Ok(Right),
            "L" => Ok(Left),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => Err(ParseDirectionError),
        }
    }
}

impl Direction {
    fn next_n_pos(&self, x: i64, y: i64, n: i64) -> (i64, i64) {
        match self {
            Up => (x - n, y),
            Down => (x + n, y),
            Right => (x, y + n),
            Left => (x, y - n),
        }
    }

    fn from_num(b: u8) -> Self {
        match b {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
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

fn join_lines(lines: &BTreeSet<Point>) -> BTreeSet<Point> {
    let mut new_lines = BTreeSet::new();
    let mut last_point = Point { x: 1, y: 0 };
    for l in lines {
        if last_point.x > last_point.y {
            last_point = l.clone();
        } else {
            if last_point.y + 1 == l.x {
                last_point.y = l.y;
            } else {
                new_lines.insert(last_point.clone());
                last_point = l.clone()
            }
        }
    }
    if last_point.x < last_point.y {
        new_lines.insert(last_point);
    }
    new_lines
}

fn total_line(lines: &BTreeSet<Point>) -> i64 {
    let mut sum = 0;
    for l in lines {
        sum += l.y - l.x + 1;
    }
    sum
}

fn fill_map(directions: &[(Direction, u32)]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut lines = Vec::new();
    for dir in directions {
        let (x_new, y_new) = dir.0.next_n_pos(x, y, dir.1 as i64);
        if dir.0 == Right || dir.0 == Left {
            lines.push(HorizontalLine {
                x,
                y_start: y.min(y_new),
                y_end: y.max(y_new),
            });
        }
        x = x_new;
        y = y_new;
    }
    lines.sort();

    x = lines[0].x;
    let mut current_lines = BTreeSet::new();
    let mut i = 0;
    while lines[i].x == x {
        let line = Point {
            x: lines[i].y_start,
            y: lines[i].y_end,
        };
        current_lines.insert(line);
        i += 1;
    }

    let mut area = total_line(&current_lines);
    for line in lines[i..].iter() {
        if line.x != x {
            area += (line.x - x) * total_line(&current_lines);
            x = line.x;
        }
        let mut ys = line.y_start;
        let mut ye = line.y_end;
        let mut replacements = Vec::new();
        for l in &current_lines {
            if ye < l.x {
                break;
            }
            if ye == l.x {
                ye -= 1;
                break;
            }
            if ys < l.x {
                // only occurs if horizontal line crosse vertical line
                area += l.x - ys;
                replacements.push((Some(l.clone()), Some(Point { x: ys, y: l.x })));
                if ye >= l.y {
                    ys = l.y + 1;
                } else {
                    replacements.push((None, Some(Point { x: ye, y: l.y })));
                    ys = ye + 1;
                    break;
                }
            }
            if ys == l.x {
                if ye >= l.y {
                    replacements.push((Some(l.clone()), None));
                    ys = l.y + 1;
                } else {
                    replacements.push((Some(l.clone()), Some(Point { x: ye, y: l.y })));
                    ys = ye + 1;
                    break;
                }
            } else if ys < l.y {
                if ye <= l.y {
                    replacements.push((Some(l.clone()), Some(Point { x: l.x, y: ys })));
                    if ye < l.y {
                        replacements.push((None, Some(Point { x: ye, y: l.y })));
                    }
                    ys = ye + 1;
                    break;
                } else if ye > l.y {
                    replacements.push((Some(l.clone()), Some(Point { x: l.x, y: ys })));
                    ys = l.y + 1;
                }
            } else if ys == l.y {
                ys += 1;
            }
        }
        if ye > ys {
            area += ye - ys + 1;
            replacements.push((None, Some(Point { x: ys, y: ye })));
        }
        for r in replacements {
            if let Some(to_remove) = r.0 {
                current_lines.remove(&to_remove);
            }
            if let Some(to_insert) = r.1 {
                current_lines.insert(to_insert);
            }
        }
        current_lines = join_lines(&current_lines);
    }

    area
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut directions = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        directions.push((
            Direction::from_str(&parts[0]).unwrap(),
            parts[1].parse::<u32>().unwrap(),
        ));
    }
    let area = fill_map(&directions);
    format!("{area}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut directions = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let num = u32::from_str_radix(&parts[2][2..7], 16).unwrap();
        directions.push((Direction::from_num(parts[2].as_bytes()[7] - b'0'), num));
    }
    let area = fill_map(&directions);
    format!("{area}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

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
        assert_eq!(result, "134549294799713");
    }
}
