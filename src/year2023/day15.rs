use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn hash(s: &str) -> u8 {
    let mut h = 0;
    for c in s.as_bytes() {
        h = ((h + *c as u32) * 17) % 256;
    }
    h as u8
}

fn riddle_1(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let l = lines.next().unwrap().unwrap();
    let steps: Vec<&str> = l.split(',').collect();
    let mut solution = 0;
    for step in steps {
        solution += hash(step) as u64;
    }
    format!("{solution}")
}

#[derive(Clone, Debug)]
struct Item {
    next: Option<usize>,
    prev: Option<usize>,
    name: String,
    focal: u8,
}

#[derive(Clone, Debug)]
struct List {
    array: Vec<Item>,
    head: Option<usize>,
}

impl List {
    fn new() -> Self {
        Self {
            array: Vec::new(),
            head: None,
        }
    }

    fn insert_or_update(&mut self, mut i: Item) {
        if self.head.is_none() {
            if self.array.is_empty() {
                self.array.push(i);
            } else {
                self.array[0] = i;
            }
            self.head = Some(0);
            return;
        }
        let mut item = self.head.unwrap();
        while self.array[item].next.is_some() && self.array[item].name != i.name {
            item = self.array[item].next.unwrap();
        }
        if self.array[item].name == i.name {
            self.array[item].focal = i.focal;
        } else {
            let mut k = 0;
            while k < self.array.len() {
                if self.array[k].focal == 255 {
                    break;
                }
                k += 1;
            }
            i.prev = Some(item);
            i.next = None;
            if k == self.array.len() {
                self.array.push(i);
            } else {
                self.array[k] = i;
            }
            self.array[item].next = Some(k);
        }
    }

    fn remove(&mut self, name: String) {
        if self.head.is_none() {
            return;
        }
        let mut item = self.head.unwrap();
        while self.array[item].next.is_some() && self.array[item].name != name {
            item = self.array[item].next.unwrap();
        }
        if self.array[item].name == name {
            self.array[item].focal = 255;
            if let Some(next) = self.array[item].next {
                if let Some(prev) = self.array[item].prev {
                    self.array[next].prev = Some(prev);
                    self.array[prev].next = Some(next);
                } else {
                    self.array[next].prev = None;
                    self.head = Some(next);
                }
            } else if let Some(prev) = self.array[item].prev {
                self.array[prev].next = None;
            } else {
                self.head = None;
            }
        }
    }

    fn score(&self) -> usize {
        if let Some(mut head) = self.head {
            let mut score = self.array[head].focal as usize;
            let mut count = 1;
            while self.array[head].next.is_some() {
                head = self.array[head].next.unwrap();
                count += 1;
                score += count * self.array[head].focal as usize;
            }
            score
        } else {
            0
        }
    }

    fn check(&self) {
        let mut head_opt = self.head;
        while let Some(head) = head_opt {
            if self.array[head].focal == 255 {
                panic!("invalid state:  {self:?}");
            }
            head_opt = self.array[head].next;
        }
    }
}

fn find_op(b: &[u8]) -> usize {
    for i in 0..b.len() {
        if b[i] == b'-' || b[i] == b'=' {
            return i;
        }
    }
    panic!("invalid instruction: {}", String::from_utf8_lossy(b));
}

fn riddle_2(mut lines: io::Lines<io::BufReader<File>>) -> String {
    let l = lines.next().unwrap().unwrap();
    let steps: Vec<&str> = l.split(',').collect();
    let mut boxes = vec![List::new(); 256];
    for s in steps {
        let bytes = s.as_bytes();
        let op_idx = find_op(bytes);
        let name = s[0..op_idx].to_string();
        let box_id = hash(&name) as usize;
        match bytes[op_idx] {
            b'=' => {
                let focal: u8 = bytes[op_idx + 1] - b'0';
                boxes[box_id].insert_or_update(Item {
                    next: None,
                    prev: None,
                    focal,
                    name,
                });
                boxes[box_id].check();
            }
            b'-' => {
                boxes[box_id].remove(name);
                boxes[box_id].check();
            }
            _ => {
                panic!("invalid operation: {}", s);
            }
        }
    }
    let mut solution = 0;
    for i in 0..256 {
        solution += (i + 1) * boxes[i].score();
    }
    format!("{solution}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_15_1() {
        let lines = read_lines("data/2023/15.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "497373");
    }

    #[test]
    fn test_2023_15_2() {
        let lines = read_lines("data/2023/15.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "259356");
    }
}
