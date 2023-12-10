use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}
use std::fmt;

fn read_sfnumbers(lines: io::Lines<io::BufReader<File>>) -> Vec<SnailFish> {
    let mut nums = Vec::new();
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .into_iter()
        .for_each(|x| {
            let mut num = Vec::new();
            let (_, sf_idx) = parse_sfnumber(&(x.chars().collect::<Vec<char>>()), 0, &mut num);
            nums.push(SnailFish {
                v: num,
                idx: sf_idx,
            });
        });
    nums
}

#[derive(Debug, Copy, Clone)]
enum SFNumber {
    Empty,
    Number(i32),
    Pair(usize, usize),
}

#[derive(Clone)]
struct SnailFish {
    v: Vec<SFNumber>,
    idx: usize,
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.write(f, self.idx)
    }
}

#[derive(Debug, Clone)]
struct SearchStatus {
    idx: Option<usize>,
    left_num: Option<usize>,
    right_num: Option<usize>,
}

impl SearchStatus {
    fn new() -> Self {
        SearchStatus {
            idx: None,
            left_num: None,
            right_num: None,
        }
    }
}

impl SnailFish {
    fn reduce(&mut self) {
        let mut has_changed = true;
        while has_changed {
            while self.explode() {}
            has_changed = self.split();
        }
    }

    fn search_depth4(&self, idx: usize, depth: i32, status: SearchStatus) -> SearchStatus {
        if status.right_num.is_some() {
            // we have all we need, return
            return status;
        }

        let mut new_status = status.clone();
        match self.v[idx] {
            SFNumber::Number(_) => {
                if status.idx.is_some() {
                    if status.right_num.is_none() {
                        // set index of first right num
                        new_status.right_num = Some(idx);
                    }
                } else {
                    // first depth 4 node node found yet, update left num
                    new_status.left_num = Some(idx);
                }
                return new_status;
            }
            SFNumber::Pair(i, j) => {
                if status.idx.is_none() && depth == 4 {
                    // found first depth 4 node
                    new_status.idx = Some(idx);
                } else {
                    new_status = self.search_depth4(i, depth + 1, new_status);
                    new_status = self.search_depth4(j, depth + 1, new_status);
                }
            }
            SFNumber::Empty => panic!("Tried to access invalid SFNum"),
        }
        new_status
    }

    fn explode(&mut self) -> bool {
        let mut has_changed = false;
        let status = self.search_depth4(self.idx, 0, SearchStatus::new());
        if let Some(idx) = status.idx {
            has_changed = true;
            let idx_nums = match self.v[idx] {
                SFNumber::Pair(i, j) => {
                    let num1 = match self.v[i] {
                        SFNumber::Number(x) => x,
                        _ => panic!("Unreachable code"),
                    };
                    let num2 = match self.v[j] {
                        SFNumber::Number(x) => x,
                        _ => panic!("Unreachable code"),
                    };
                    (num1, num2)
                }
                _ => panic!("Could unreachable"),
            };
            if let Some(left_num) = status.left_num {
                match self.v[left_num] {
                    SFNumber::Number(x) => {
                        self.v[left_num] = SFNumber::Number(x + idx_nums.0);
                    }
                    _ => panic!("Invalid SFNumber to explode, should not happen!"),
                }
            };
            if let Some(right_num) = status.right_num {
                match self.v[right_num] {
                    SFNumber::Number(x) => {
                        self.v[right_num] = SFNumber::Number(x + idx_nums.1);
                    }
                    _ => panic!("Invalid SFNumber to explode, should not happen!"),
                }
            };
            match self.v[idx] {
                SFNumber::Pair(i, j) => {
                    self.v[i] = SFNumber::Empty;
                    self.v[j] = SFNumber::Empty;
                }
                _ => panic!("Invalid SFNumber to explode, should not happen!"),
            }
            self.v[idx] = SFNumber::Number(0);
        }
        has_changed
    }

    fn find_splitable(&self, idx: usize) -> Option<usize> {
        match self.v[idx] {
            SFNumber::Number(x) => {
                if x > 9 {
                    Some(idx)
                } else {
                    None
                }
            }
            SFNumber::Pair(i, j) => {
                let left_idx = self.find_splitable(i);
                if left_idx.is_some() {
                    left_idx
                } else {
                    self.find_splitable(j)
                }
            }
            SFNumber::Empty => panic!("Tried to acces empty node"),
        }
    }

    fn split(&mut self) -> bool {
        let mut has_changed = false;
        let idx = self.find_splitable(self.idx);
        if let Some(idx) = idx {
            match self.v[idx] {
                SFNumber::Number(x) => {
                    self.v.push(SFNumber::Number(x / 2));
                    self.v.push(SFNumber::Number((x + 1) / 2));
                    self.v[idx] = SFNumber::Pair(self.v.len() - 2, self.v.len() - 1);
                    has_changed = true;
                }
                _ => panic!("Unreachable code"),
            }
        }
        has_changed
    }

    fn add(&mut self, other: &Self) {
        let idx1 = self.idx;
        let idx2 = other.idx;
        let offset = self.v.len();
        self.v.extend(other.v.iter());
        for i in offset..self.v.len() {
            match self.v[i] {
                SFNumber::Number(_) => {}
                SFNumber::Pair(k, l) => {
                    self.v[i] = SFNumber::Pair(k + offset, l + offset);
                }
                SFNumber::Empty => panic!("Tried to acccess empty SFNumber"),
            }
        }
        self.v.push(SFNumber::Pair(idx1, idx2 + offset));
        self.idx = self.v.len() - 1;
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, idx: usize) -> Result<(), fmt::Error> {
        match self.v[idx] {
            SFNumber::Number(x) => write!(f, "{}", x),
            SFNumber::Pair(i, j) => {
                write!(f, "[")?;
                self.write(f, i)?;
                write!(f, ",")?;
                self.write(f, j)?;
                write!(f, "]")
            }
            SFNumber::Empty => panic!("Tried to acccess empty SFNumber"),
        }
    }

    fn magnitude(&self, idx: usize) -> usize {
        match self.v[idx] {
            SFNumber::Number(x) => x as usize,
            SFNumber::Pair(i, j) => 3 * self.magnitude(i) + 2 * self.magnitude(j),
            SFNumber::Empty => panic!("Tried to acces empty node"),
        }
    }
}

fn parse_number(s: &[char], idx: usize, num: &mut Vec<SFNumber>) -> (usize, usize) {
    let mut last = idx;
    for i in idx..s.len() {
        last = i;
        if s[i] < '0' || s[i] > '9' {
            break;
        }
    }
    let inum = s[idx..last]
        .iter()
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    num.push(SFNumber::Number(inum));
    (last, num.len() - 1)
}

fn parse_pair(s: &[char], idx: usize, num: &mut Vec<SFNumber>) -> (usize, usize) {
    let (idx, sf_idx1) = parse_sfnumber(s, idx, num);
    let (idx, sf_idx2) = parse_sfnumber(s, idx, num);
    num.push(SFNumber::Pair(sf_idx1, sf_idx2));
    (idx, num.len() - 1)
}
fn parse_sfnumber(s: &[char], idx: usize, num: &mut Vec<SFNumber>) -> (usize, usize) {
    let (idx, sf_idx) = match s[idx] {
        '[' => parse_pair(s, idx + 1, num),
        '0'..='9' => parse_number(s, idx, num),
        _ => {
            panic!("Invalid snail fish number!")
        }
    };
    (idx + 1, sf_idx)
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let nums = read_sfnumbers(lines);
    let mut num = nums[0].clone();
    for n in nums.into_iter().skip(1) {
        num.add(&n);
        num.reduce();
    }
    format!("{}", num.magnitude(num.idx))
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let nums = read_sfnumbers(lines);
    let mut max_magnitude = 0;
    let n = nums.len();
    for i in 0..n {
        for j in i + 1..n {
            let mut num1 = nums[i].clone();
            num1.add(&nums[j]);
            num1.reduce();
            max_magnitude = max_magnitude.max(num1.magnitude(num1.idx));
            let mut num2 = nums[j].clone();
            num2.add(&nums[i]);
            num2.reduce();
            max_magnitude = max_magnitude.max(num2.magnitude(num2.idx));
        }
    }
    format!("{max_magnitude}")
}
