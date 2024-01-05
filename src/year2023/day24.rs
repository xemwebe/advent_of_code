use std::{
    fs::File,
    io,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone)]
struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
struct Hail {
    pos: Vec3D,
    speed: Vec3D,
}

#[derive(Debug, Clone)]
struct Solver {
    hails: Vec<Hail>,
    min: f64,
    max: f64,
}

impl Solver {
    fn new(hails: Vec<Hail>) -> Self {
        Self{
            hails,
            min: 200000000000000.0,
            max: 400000000000000.0,
            // min: 7.0,
            // max: 27.0
        }
    }

    fn xy_crossing(&self, i: usize, j: usize) -> bool {
        //println!("check {i}:{j}");
        let hi = &self.hails[i];
        let hj = &self.hails[j];
        let ca = hj.pos.x - hi.pos.x;
        let ge = hj.pos.y - hi.pos.y;
        //println!("ca: {ca}, ge: {ge}");
        if hi.speed.y * hj.speed.x == hi.speed.x * hj.speed.y {
            //println!("parallel");
            // parallel movement
            return false;
        }
        let t1 = if hj.speed.x==0.0 {
            ca/hi.speed.y
        } else {
            let fbhd = hi.speed.y - hi.speed.x/hj.speed.x*hj.speed.y;
            let geachd = ge - ca / hj.speed.x * hj.speed.y;
            geachd / fbhd
        };
        //println!("t1: {t1}");
        if t1 < 0.0 {
            return false;
        }
        let t2 = if hj.speed.x==0.0 {
            (hi.speed.y * t1 - ge) / hj.speed.y 
        } else {
            (hi.speed.x * t1 - ca) / hj.speed.x
        };
        //println!("t2: {t2}");
        if t2 < 0.0 {
            return false;
        }
        let x = hi.pos.x + t1 * hi.speed.x;
        let y = hi.pos.y + t1 * hi.speed.y;
        //println!("x: {x}, y: {y}");
        if x<self.min || x>self.max || y<self.min || y>self.max {
            return false
        }
        true    
    }

    fn solve(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.hails.len() {
            for j in i+1..self.hails.len() {
                if self.xy_crossing(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn parse_input(lines: io::Lines<io::BufReader<File>>, avoid_slippery: bool) -> Solver {
    let mut hails = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(" @ ").collect();
        let pos: Vec<f64> = parts[0].split(", ").map(|s| (*s).trim().parse::<f64>().unwrap() ).collect();
        let speed: Vec<f64> = parts[1].split(", ").map(|s| (*s).trim().parse::<f64>().unwrap() ).collect();
        let pos = Vec3D{ x: pos[0], y: pos[1], z: pos[2] };
        let speed = Vec3D{ x: speed[0], y: speed[1], z: speed[2] };
        hails.push(Hail{pos, speed});
    }
    Solver::new(hails)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines, true);
    let solution = solver.solve();
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    format!("not yet implemented")
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::read_lines;

    #[test]
    fn test_2023_24_1() {
        let lines = read_lines("data/2023/24.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "21785");
    }

    #[test]
    fn test_2023_24_2() {
        let lines = read_lines("data/2023/24.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "6522");
    }
}
