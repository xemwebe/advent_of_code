use super::*;

pub fn riddle_6_1(lines: io::Lines<io::BufReader<File>>) {
    let mut lines = lines.into_iter();
    let times: Vec<u64> = lines.next().unwrap().unwrap().split(' ').filter_map(|s| s.parse().ok() ).collect();
    let distances: Vec<u64> = lines.next().unwrap().unwrap().split(' ').filter_map(|s| s.parse().ok() ).collect();
    let mut solution = 1;
    for i in 0..times.len() {
        let t_half = (times[i] as f64)/2.0;
        let lower = t_half - (t_half*t_half - distances[i] as f64).sqrt();
        let upper = t_half + (t_half*t_half - distances[i] as f64).sqrt();
        let count = upper.floor() - lower.ceil() + 1.0;
        solution *= count as u64;
    }
    println!("The solution is: {}", solution);
}

pub fn riddle_6_2(lines: io::Lines<io::BufReader<File>>) {
    let mut lines = lines.into_iter();
    let first_line: Vec<String> = lines.next().unwrap().unwrap().split(": ").map(|s| s.to_string()).collect();
    let second_line: Vec<String> = lines.next().unwrap().unwrap().split(": ").map(|s| s.to_string()).collect();
    let time: u64 = first_line[1].replace(' ', "").parse().unwrap();
    let distance: u64 = second_line[1].replace(' ', "").parse().unwrap();
    println!("time:{time:?}, distance: {distance:?}");
    let t_half = (time as f64)/2.0;
    let lower = t_half - (t_half*t_half - distance as f64).sqrt();
    let upper = t_half + (t_half*t_half - distance as f64).sqrt();
    let solution = upper.floor() - lower.ceil() + 1.0;
    println!("The solution is: {}", solution);
}
