use super::*;

fn is_digit(b: u8) -> bool {
    (b >= b'0') && (b <= b'9')
}

fn get_num(bytes: &[u8]) -> i32 {
    let mut num = 0;
    for b in bytes {
        if is_digit(*b) {
            num = 10 * (b - b'0');
            break;
        }
    }
    for i in (0..bytes.len()).rev() {
        if is_digit(bytes[i]) {
            return (num + bytes[i] - b'0').into();
        }
    }
    0
}

pub fn riddle_1_1(lines: io::Lines<io::BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let num = get_num(line.unwrap().as_bytes());
        sum += num;
    }
    println!("The solution is: {}", sum);
}

fn is_digit2(i: usize, b: &[u8]) -> Option<u8> {
    let digits: Vec<Vec<u8>> = vec![
        b"zero".to_vec(),
        b"one".to_vec(),
        b"two".to_vec(),
        b"three".to_vec(),
        b"four".to_vec(),
        b"five".to_vec(),
        b"six".to_vec(),
        b"seven".to_vec(),
        b"eight".to_vec(),
        b"nine".to_vec(),
    ];

    if b[i] >= b'0' && b[i] <= b'9' {
        return Some(b[i] - b'0');
    }
    for j in 0..10 {
        let di = digits[j].len();
        if i+di>b.len() {
            continue;
        }
        if b[i..i + di] == digits[j] {
            return Some(j as u8);
        }
    }
    None
}

fn get_num2(bytes: &[u8]) -> i32 {
    let mut num = 0;
    for i in 0..bytes.len() {
        if let Some(d) = is_digit2(i, bytes) {
            num = 10 * d;
            break;
        }
    }
    for i in (0..bytes.len()).rev() {
        if let Some(d) = is_digit2(i, bytes) {
            return (num + d).into();
        }
    }
    0
}

pub fn riddle_1_2(lines: io::Lines<io::BufReader<File>>) {
    let mut sum = 0;
    for line in lines {
        let num = get_num2(line.unwrap().as_bytes());
        sum += num;
    }
    println!("The solution is: {}", sum);
}
