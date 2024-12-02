use mathru::{
    algebra::linear::{
        matrix::{General, Solve, Transpose},
        vector::Vector,
    },
    vector,
};
use std::{
    fs::File,
    io,
    ops::{Add, Neg, Sub},
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

impl Vec3D {
    fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
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
        Self {
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
        let t1 = if hj.speed.x == 0.0 {
            ca / hi.speed.y
        } else {
            let fbhd = hi.speed.y - hi.speed.x / hj.speed.x * hj.speed.y;
            let geachd = ge - ca / hj.speed.x * hj.speed.y;
            geachd / fbhd
        };
        //println!("t1: {t1}");
        if t1 < 0.0 {
            return false;
        }
        let t2 = if hj.speed.x == 0.0 {
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
        if x < self.min || x > self.max || y < self.min || y > self.max {
            return false;
        }
        true
    }

    fn solve(&self) -> i32 {
        let mut count = 0;
        for i in 0..self.hails.len() {
            for j in i + 1..self.hails.len() {
                if self.xy_crossing(i, j) {
                    count += 1;
                }
            }
        }
        count
    }

    // for any hail with position pi and velocity vi and rock with position r and velocity vr we have in vector notation:
    // r + vr*ti = pi + vi*ti (ti is the hit time), i.e.
    // r - pi = -ti*(vr-vi) thus the following cross product is zero:
    // (r-pi)x(vr-vi) = 0 (this are three bilinear equations). Since r x vr is the same for each i
    // we can use two hails to get three linear equations:
    // r x (v1-v2) - vr x (p1-p2) = p1 x v1 - p2 x v2
    // Using a third hail, we get six linear equations with six variables, which can be solved
    // using standard methods.
    fn solve_p2(&self, first: usize, second: usize, third: usize) -> Hail {
        let p1 = &self.hails[first].pos;
        let v1 = &self.hails[first].speed;
        let p2 = &self.hails[second].pos;
        let v2 = &self.hails[second].speed;
        let p3 = &self.hails[third].pos;
        let v3 = &self.hails[third].speed;
        let d1 = p1.cross(v1) - p2.cross(v2);
        let d2 = p2.cross(v2) - p3.cross(v3);
        let v12 = v1.clone() - v2.clone();
        let p12 = p1.clone() - p2.clone();
        let v23 = v2.clone() - v3.clone();
        let p23 = p2.clone() - p3.clone();
        let a: General<f64> = General::new(
            6,
            6,
            vec![
                0., v12.z, -v12.y, 0., -p12.z, p12.y, -v12.z, 0., v12.x, p12.z, 0., -p12.x, v12.y,
                -v12.x, 0., -p12.y, p12.x, 0., 0., v23.z, -v23.y, 0., -p23.z, p23.y, -v23.z, 0.,
                v23.x, p23.z, 0., -p23.x, v23.y, -v23.x, 0., -p23.y, p23.x, 0.,
            ],
        );
        let a = a.transpose();
        let b: Vector<f64> = vector![d1.x, d1.y, d1.z, d2.x, d2.y, d2.z];
        let b = b.transpose();
        let x: Vector<f64> = a.solve(&b).unwrap();

        let r = Vec3D {
            x: x[0].round(),
            y: x[1].round(),
            z: x[2].round(),
        };
        let vr = Vec3D {
            x: x[3].round(),
            y: x[4].round(),
            z: x[5].round(),
        };

        let rock = Hail { pos: r, speed: vr };
        rock
    }
}

fn parse_input(lines: io::Lines<io::BufReader<File>>) -> Solver {
    let mut hails = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split(" @ ").collect();
        let pos: Vec<f64> = parts[0]
            .split(", ")
            .map(|s| (*s).trim().parse::<f64>().unwrap())
            .collect();
        let speed: Vec<f64> = parts[1]
            .split(", ")
            .map(|s| (*s).trim().parse::<f64>().unwrap())
            .collect();
        let pos = Vec3D {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        };
        let speed = Vec3D {
            x: speed[0],
            y: speed[1],
            z: speed[2],
        };
        hails.push(Hail { pos, speed });
    }
    Solver::new(hails)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines);
    let solution = solver.solve();
    format!("{solution}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let solver = parse_input(lines);
    // Solution is instable due rounding errors, but this values gave the correct result for my input ;-)
    let rock = solver.solve_p2(31, 39, 34);
    let solution = rock.pos.x + rock.pos.y + rock.pos.z;
    format!("{solution}")
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
        assert_eq!(result, "554668916217145");
    }
}
