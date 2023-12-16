use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Sensor {
    pub pos: (i64, i64),
    pub beacon: (i64, i64),
}

impl Sensor {
    fn beacon_distance(&self) -> i64 {
        (self.pos.0 - self.beacon.0).abs() + (self.pos.1 - self.beacon.1).abs()
    }

    fn beacon_distance_to_row_range(&self, row: i64) -> Option<(i64, i64)> {
        let extra_columns = self.beacon_distance() - (row - self.pos.1).abs();
        if extra_columns >= 0 {
            Some((self.pos.0 - extra_columns, self.pos.0 + extra_columns))
        } else {
            None
        }
    }

    fn no_beacon_on_row_distance(&self, row: i64, cols: &mut HashSet<i64>) {
        if let Some(col_range) = self.beacon_distance_to_row_range(row) {
            for c in col_range.0..=col_range.1 {
                cols.insert(c);
            }
        }
    }

    fn remove_beacon(&self, row: i64, cols: &mut HashSet<i64>) {
        if self.beacon.1 == row {
            cols.remove(&self.beacon.0);
        }
    }
}

pub fn read_map(lines: io::Lines<io::BufReader<File>>) -> Vec<Sensor> {
    let re = Regex::new(r".*x=([-0-9]*), y=([-0-9]*).*x=([-0-9]*), y=([-0-9]*)").unwrap();
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| {
            let r = re.captures_iter(&x).into_iter().next().unwrap();
            let x = r.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = r.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let bx = r.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let by = r.get(4).unwrap().as_str().parse::<i64>().unwrap();
            Sensor {
                pos: (x, y),
                beacon: (bx, by),
            }
        })
        .collect()
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let sensors = read_map(lines);
    let mut cols = HashSet::new();
    let row = 2000000;
    for sensor in &sensors {
        sensor.no_beacon_on_row_distance(row, &mut cols)
    }
    for sensor in &sensors {
        sensor.remove_beacon(row, &mut cols);
    }
    format!("{}", cols.len())
}

struct ColRange {
    ranges: HashSet<(i64, i64)>,
    min: Option<i64>,
    max: Option<i64>,
}

impl ColRange {
    fn insert(&mut self, range: (i64, i64)) {
        let mut range = range;
        if let Some(min) = self.min {
            range = (range.0.max(min), range.1.max(min));
        }
        if let Some(max) = self.max {
            range = (range.0.min(max), range.1.min(max));
        }
        self.merge_with_ranges(range);
    }

    fn merge_with_ranges(&mut self, range: (i64, i64)) {
        let mut drop = None;
        for r in &self.ranges {
            if range.0 >= r.0 && range.1 <= r.1 {
                return;
            }
            if (range.0 >= r.0 && range.0 <= r.1)
                || (range.1 >= r.0 && range.1 <= r.1)
                || (range.0 <= r.0 && range.1 >= r.1)
            {
                drop = Some((r.0, r.1));
                break;
            }
        }
        if let Some(drop) = drop {
            self.ranges.remove(&drop);
            self.merge_with_ranges((drop.0.min(range.0), drop.1.max(range.1)))
        } else {
            self.ranges.insert(range);
        }
    }

    fn total_size(&self) -> i64 {
        let mut sum = 0;
        for r in &self.ranges {
            sum += r.1 - r.0 + 1;
        }
        sum
    }

    fn get_beacon(&self) -> i64 {
        let r = self.ranges.iter().next().unwrap();
        if r.0 > 0 {
            r.0 - 1
        } else {
            r.1 + 1
        }
    }
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let sensors = read_map(lines);
    let max = 4000000;
    for row in 0..=max {
        let mut cols = ColRange {
            ranges: HashSet::new(),
            min: Some(0),
            max: Some(max),
        };
        for sensor in &sensors {
            if let Some(range) = sensor.beacon_distance_to_row_range(row) {
                cols.insert(range);
            }
        }
        if cols.total_size() < max + 1 {
            let y = row;
            let x = cols.get_beacon();
            return format!("{}", x * max + y);
        }
    }
    "no solution found".to_string()
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2022_15_1() {
        let lines = read_lines("data/2022/15.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "7195");
    }

    #[test]
    fn test_2022_15_2() {
        let lines = read_lines("data/2022/15.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "33992866292225");
    }
}

