use std::cmp::Ordering;
use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Solver {
    fat: Vec<u8>,
}

impl Solver {
    fn create_from_input(mut lines: io::Lines<io::BufReader<File>>) -> Self {
        let fat: Vec<u8> = lines
            .next()
            .unwrap()
            .unwrap()
            .as_bytes()
            .iter()
            .map(|x| *x - b'0')
            .collect();
        Self { fat }
    }

    fn solve(&self) -> usize {
        let mut new_fat = vec![(0usize, self.fat[0] as usize)];
        let mut last = self.fat.len() - 1;
        if last % 2 != 0 {
            last -= 1;
        }
        let mut last_left = self.fat[last];
        let mut free = 1;
        let mut free_left = self.fat[free];
        loop {
            //println!("new_fat: {new_fat:?} {free_left} {last_left} {free} {last}");
            while free_left == 0 {
                free += 2;
                free_left = self.fat[free];
                new_fat.push((free / 2, self.fat[free - 1] as usize));
            }
            while last_left == 0 {
                last -= 2;
                last_left = self.fat[last];
            }
            if last <= free {
                break;
            }
            if free_left <= last_left {
                new_fat.push((last / 2, free_left as usize));
                last_left -= free_left;
                free_left = 0;
            } else {
                new_fat.push((last / 2, last_left as usize));
                free_left -= last_left;
                last_left = 0;
            }
            if last == free + 1 {
                break;
            }
        }
        new_fat.push((last / 2, last_left as usize));
        let mut check_sum = 0;
        let mut block: usize = 0;
        for (idx, size) in new_fat {
            for i in block..block + size {
                check_sum += i * idx;
            }
            block += size;
        }
        check_sum
    }
}

#[derive(Debug)]
struct FileEntry {
    len: usize,
    idx: Option<usize>,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug)]
struct Solver2 {
    file_list: Vec<FileEntry>,
}

impl Solver2 {
    fn create_from_input(mut lines: io::Lines<io::BufReader<File>>) -> Self {
        let mut file_list: Vec<FileEntry> = lines
            .next()
            .unwrap()
            .unwrap()
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, x)| FileEntry {
                len: (*x - b'0') as usize,
                idx: if i % 2 == 0 { Some(i / 2) } else { None },
                prev: if i == 0 { None } else { Some(i - 1) },
                next: Some(i + 1),
            })
            .collect();
        file_list.last_mut().unwrap().next = None;
        Self { file_list }
    }

    fn next(&self, i: usize) -> usize {
        self.file_list[i].next.unwrap()
    }

    fn is_none(&self, i: usize) -> bool {
        self.file_list[i].idx.is_none()
    }

    fn cmp(&self, i: usize, j: usize) -> Ordering {
        self.file_list[i].len.cmp(&self.file_list[j].len)
    }

    fn solve(&mut self) -> usize {
        let mut last = self.file_list.len() - 1;
        loop {
            if last < 2 {
                break;
            }
            while self.is_none(last) {
                last -= 2;
            }
            let mut first = 1;
            let mut ordering = self.cmp(first, last);
            let mut not_found = false;
            loop {
                if self.is_none(first) && ordering != Ordering::Less {
                    break;
                }
                if let Some(new_first) = self.file_list[first].next {
                    if new_first == last {
                        not_found = true;
                        break;
                    }
                    first = new_first;
                    ordering = self.cmp(first, last);
                } else {
                    not_found = true;
                    break;
                }
            }
            if not_found {
                last -= 2;
                continue;
            }
            if ordering == Ordering::Equal {
                self.file_list[first].idx = self.file_list[last].idx;
                self.file_list[last].idx = None;
            } else {
                let new_file = FileEntry {
                    idx: self.file_list[last].idx,
                    len: self.file_list[last].len,
                    prev: self.file_list[first].prev,
                    next: Some(self.file_list.len()),
                };
                let new_free = FileEntry {
                    idx: None,
                    len: self.file_list[first].len - self.file_list[last].len,
                    prev: Some(first),
                    next: self.file_list[first].next,
                };
                self.file_list.push(new_free);
                self.file_list[first] = new_file;
                self.file_list[last].idx = None;
                last -= 2;
            }
        }
        let mut i = 0;
        let mut checksum = 0;
        let mut block = 0;
        loop {
            let size = self.file_list[i].len;
            if let Some(file_idx) = self.file_list[i].idx {
                for i in block..block + size {
                    checksum += i * file_idx;
                }
            }
            block += size;
            if self.file_list[i].next.is_none() {
                break;
            }
            i = self.next(i);
        }
        checksum
    }
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = Solver::create_from_input(lines);
    let check_sum = solver.solve();
    format!("{check_sum}")
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = Solver2::create_from_input(lines);
    let check_sum = solver.solve();
    format!("{check_sum}")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2024_9_1() {
        let lines = read_lines("data/2024/9.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "6288707484810");
    }

    #[test]
    fn test_2024_9_2() {
        let lines = read_lines("data/2024/9.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "6311837662089");
    }
}
