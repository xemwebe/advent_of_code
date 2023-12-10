use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

struct Triangle {
    len: usize,
    data: Vec<Vec<i64>>,
}

impl Triangle {
    fn new() -> Self {
        Self {
            len: 0,
            data: Vec::new(),
        }
    }
    fn clear(&mut self) {
        for t in &mut self.data {
            t.clear();
        }
        self.len = 0;
    }

    fn complete(&self) -> bool {
        self.len > 0 && *self.data[self.len - 1].last().unwrap() == 0
    }

    fn hist_complete(&self) -> bool {
        if self.len > 1 {
            self.data[self.len - 2][0] == 0 && self.data[self.len - 2][1] == 0
        } else {
            false
        }
    }

    fn add_row(&mut self, sequence: &[i64]) {
        let last = sequence.len() - 1 - self.len;
        let mut diff = sequence[last] - sequence[last - 1];
        for i in 0..self.len + 1 {
            if self.data.len() <= i {
                self.data.push(vec![diff]);
            } else {
                self.data[i].push(diff);
            }
            let di = self.data[i].len();
            if di >= 2 {
                diff = self.data[i][di - 2] - self.data[i][di - 1];
            }
        }
        self.len += 1;
    }

    fn prediction(&self) -> i64 {
        let mut prediction = 0;
        for i in (0..(self.len - 1)).rev() {
            prediction += self.data[i][0];
        }
        prediction
    }

    fn add_hist_row(&mut self, sequence: &[i64]) {
        let first = self.len;
        let mut diff = sequence[first + 1] - sequence[first];
        for i in 0..self.len + 1 {
            if self.data.len() <= i {
                self.data.push(vec![diff]);
            } else {
                self.data[i].push(diff);
            }
            let di = self.data[i].len();
            if di >= 2 {
                diff = self.data[i][di - 1] - self.data[i][di - 2];
            }
        }
        self.len += 1;
    }

    fn hist_prediction(&self) -> i64 {
        let mut prediction = 0;
        for i in (0..(self.len - 1)).rev() {
            prediction = self.data[i][0] - prediction;
        }
        prediction
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solution = 0;
    let mut triangle = Triangle::new();
    for line in lines {
        let sequence: Vec<i64> = line
            .unwrap()
            .split(' ')
            .map(|s| s.to_string().parse().unwrap())
            .collect();
        triangle.clear();
        while !triangle.complete() {
            triangle.add_row(&sequence);
        }
        solution += triangle.prediction() + sequence.last().unwrap();
    }
    format!("{solution}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solution = 0;
    let mut triangle = Triangle::new();
    for line in lines {
        let sequence: Vec<i64> = line
            .unwrap()
            .split(' ')
            .map(|s| s.to_string().parse().unwrap())
            .collect();
        triangle.clear();
        while !triangle.hist_complete() {
            triangle.add_hist_row(&sequence);
        }
        solution += sequence[0] - triangle.hist_prediction();
    }
    format!("{solution}")
}
