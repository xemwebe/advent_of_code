use super::*;
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

pub fn riddle_15_1(lines: io::Lines<io::BufReader<File>>) {
    let sensors = read_map(lines);
    let mut cols = HashSet::new();
    let row = 2000000;
    for sensor in &sensors {
        sensor.no_beacon_on_row_distance(row, &mut cols)
    }
    for sensor in &sensors {
        sensor.remove_beacon(row, &mut cols);
    }
    println!("The solution is: {:?}", cols.len());
}

struct ColRange {
    ranges: HashSet<(i64,i64)>,
    min: Option<i64>,
    max: Option<i64>
}

impl ColRange {
    fn insert(&self, range: (i64, i64)) {
        
    }
}
pub fn riddle_15_2(lines: io::Lines<io::BufReader<File>>) {
    let sensors = read_map(lines);
    let mut sum = 0;
    for row in 0..4000000 {
        for sensor in &sensors {
            sum += sensor.beacon_distance_to_row_range(row).unwrap().0;
        }
    }
    println!("The solution is: {:?}", sum);
}
