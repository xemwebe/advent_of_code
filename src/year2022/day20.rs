use std::{fs::File, io};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

fn read_values(lines: io::Lines<io::BufReader<File>>) -> Vec<(i64, i64)> {
    let mut idx = 0;
    lines
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|s| {
            let value = (s.parse::<i64>().unwrap(), idx);
            idx += 1;
            value
        })
        .collect()
}

fn scramble(vals: &mut Vec<(i64, i64)>) {
    let len = vals.len() as i64;
    for i in 0..len {
        let mut old_idx = 0;
        for j in 0..vals.len() {
            if vals[j].1 == i {
                old_idx = j;
                break;
            }
        }
        let v = vals[old_idx];
        let mut new_idx = (old_idx as i64) + v.0;
        if new_idx < 0 {
            let factor = new_idx / (len - 1);
            new_idx -= factor * (len - 1);
            if new_idx < 0 {
                new_idx += len - 1;
            }
        }
        if new_idx >= len {
            let factor = new_idx / (len - 1);
            new_idx -= factor * (len - 1);
            if new_idx >= len {
                new_idx -= len - 1;
            }
        }
        let new_idx = new_idx as usize;
        if new_idx < old_idx {
            for k in (new_idx..old_idx).rev() {
                vals[k + 1] = vals[k]
            }
        } else if new_idx > old_idx {
            for k in old_idx..new_idx {
                vals[k] = vals[k + 1]
            }
        }
        vals[new_idx] = v;
    }
}

fn get_coordinate(vals: &Vec<(i64, i64)>) -> i64 {
    let mut zero_idx = 0;
    for i in 0..vals.len() {
        if vals[i].0 == 0 {
            zero_idx = i;
        }
    }
    let first = (zero_idx + 1000) % vals.len();
    let second = (first + 1000) % vals.len();
    let third = (second + 1000) % vals.len();
    vals[first].0 + vals[second].0 + vals[third].0
}

pub fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut values = read_values(lines);
    scramble(&mut values);
    format!("{:?}", get_coordinate(&values))
}

pub fn multiply_with_key(vals: &mut Vec<(i64, i64)>, key: i64) {
    for v in vals {
        v.0 *= key;
    }
}

pub fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut values = read_values(lines);
    let key = 811589153_i64;
    multiply_with_key(&mut values, key);
    for _ in 0..10 {
        scramble(&mut values);
    }
    format!("{:?}", get_coordinate(&values))
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2022_20_1() {
        let lines = read_lines("data/2022/20.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "17490");
    }

    #[test]
    fn test_2022_20_2() {
        let lines = read_lines("data/2022/20.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "1632917375836");
    }
}
