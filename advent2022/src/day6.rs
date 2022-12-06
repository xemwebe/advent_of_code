use super::*;

fn is_valid(i: usize, v: &[u8], count: usize) -> bool {
    for j in 0..count {
        for k in j+1..count {
            if v[i-j] == v[i-k] {
                return false;
            }
        }
    }
    true
}

fn find_start(s: &str, count: usize) -> usize {
    let v = s.as_bytes();
    for i in count..v.len() {
        if is_valid(i, v, count) {
            return i+1;
        }
    }
    return 0;
}

pub fn riddle_6_1(lines: io::Lines<io::BufReader<File>>) {
    let input = lines.into_iter().next().unwrap().unwrap();
    let start = find_start(&input, 4);
    println!("The solution is: {}", start);
}

pub fn riddle_6_2(lines: io::Lines<io::BufReader<File>>) {
    let input = lines.into_iter().next().unwrap().unwrap();
    let start = find_start(&input, 14);
    println!("The solution is: {}", start);
}
