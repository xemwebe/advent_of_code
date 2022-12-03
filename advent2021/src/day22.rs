use super::*;
use regex::Regex;
use std::collections::HashSet;
use std::cmp::Ordering;

fn read_parameter(lines: io::Lines<io::BufReader<File>>) -> Vec<(bool,(i32,i32),(i32,i32),(i32,i32))> {
    let re = Regex::new(r"(on|off) x=([-0-9]*)..([-0-9]*),y=([-0-9]*)..([-0-9]*),z=([-0-9]*)..([-0-9]*)").unwrap();
    
    lines.into_iter().filter_map(|x| x.ok())
        .map(|x| {
            let r = re.captures_iter(&x).into_iter().next().unwrap();
            let switch = r.get(1).unwrap().as_str() == "on";
            let x1 = r.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = r.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let y1 = r.get(4).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = r.get(5).unwrap().as_str().parse::<i32>().unwrap();
            let z1 = r.get(6).unwrap().as_str().parse::<i32>().unwrap();
            let z2 = r.get(7).unwrap().as_str().parse::<i32>().unwrap();
            (switch, (x1,x2), (y1,y2), (z1, z2))})
            .collect()
}
pub fn riddle_22_1(lines: io::Lines<io::BufReader<File>>) {
    let commands = read_parameter(lines);
    let mut on = HashSet::new();
    for c in commands {
        for x in c.1.0.max(-50)..=c.1.1.min(50) {
            for y in c.2.0.max(-50)..=c.2.1.min(50) {
                for z in c.3.0.max(-50)..=c.3.1.min(50) {
                    let k = (x,y,z);
                    if c.0 {
                        on.insert(k);
                    } else {
                        on.remove(&k);
                    }
                }
            }
        }
    }
    println!("{:?}", on.len());
}

#[derive(Debug,Hash,Clone,Eq)]
struct Cube {
    x1: i32,
    x2: i32,
    y1: i32, 
    y2: i32,
    z1: i32,
    z2: i32,
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x1 == other.x1 && self.y1 == other.y1 && self.z1 == other.z1
        && self.x2 == other.x2 && self.y2 == other.y2 && self.z2 == other.z2
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x1 == other.x1 {
            if self.y1 == other.y1 {
                if self.z1 == other.z1 {
                    if self.x2 == other.x2 {
                        if self.y2 == other.y2 {
                            self.z2.cmp(&other.z2)
                        } else {
                            self.y2.cmp(&other.y2)
                        }
                    } else {
                        self.x2.cmp(&other.x2)
                    }
                } else {
                    self.z1.cmp(&other.z2)
                }
            } else {
                self.y1.cmp(&other.y1)
            }
        } else {
            self.x1.cmp(&other.x1)
        }
    }
}

impl Cube {
    fn new(x1:i32, x2:i32, y1:i32, y2:i32, z1:i32, z2:i32) -> Self {
        Cube{x1,x2,y1,y2,z1,z2}
    }
    fn volume(&self) -> usize {
        ((self.x2-self.x1+1) as usize) * ((self.y2-self.y1+1) as usize) * ((self.z2-self.z1+1) as usize)
    }
}

#[derive(Debug)]
struct Reactor {
    cubids: HashSet<Cube>,
}

impl Reactor {
    fn new() -> Self {
        Reactor{
            cubids: HashSet::new(),
        }
    }

    fn intersections(&self, cube: &Cube) -> HashSet<Cube> {
        let mut intersections = HashSet::new();
        for c in self.cubids.iter() {
            if c.x2>=cube.x1 && cube.x2>=c.x1 && c.y2>=cube.y1 && cube.y2>=c.y1 && c.z2>=cube.z1 && cube.z2>=c.z1 {
                intersections.insert(c.clone());
            }
        }
        intersections
    }

    fn insert(&mut self, cube: &Cube) {
        self.remove(cube);
        self.cubids.insert(cube.clone());
    }

    fn remove(&mut self, cube: &Cube) {
        for c in self.intersections(cube) {
            self.cubids.remove(&c);
            if c.x1<cube.x1 && cube.x1 <= c.x2 {
               self.cubids.insert(Cube::new(c.x1, cube.x1-1, c.y1, c.y2, c.z1, c.z2));
            }
            if c.x1 <= cube.x2 && cube.x2 < c.x2 {
                self.cubids.insert(Cube::new(cube.x2+1,c.x2, c.y1, c.y2, c.z1, c.z2));
            }
            if c.y1< cube.y1 && cube.y1 <= c.y2 {
                self.cubids.insert(Cube::new(c.x1.max(cube.x1),c.x2.min(cube.x2), c.y1, cube.y1-1, c.z1, c.z2));
            }
            if c.y1<= cube.y2 && cube.y2 < c.y2 {
                self.cubids.insert(Cube::new(c.x1.max(cube.x1),c.x2.min(cube.x2), cube.y2+1, c.y2, c.z1, c.z2));
            }
            if c.z1< cube.z1 && cube.z1 <= c.z2 {
                self.cubids.insert(Cube::new(c.x1.max(cube.x1),c.x2.min(cube.x2), 
                    c.y1.max(cube.y1), c.y2.min(cube.y2), c.z1, cube.z1-1));
            }
            if c.z1<= cube.z2 && cube.z2 < c.z2 {
                self.cubids.insert(Cube::new(c.x1.max(cube.x1),c.x2.min(cube.x2), 
                    c.y1.max(cube.y1), c.y2.min(cube.y2), cube.z2+1, c.z2));
            }
        }
    }

    fn volume(&self) -> usize {
        let mut volume = 0;
        for c in self.cubids.iter() {
            volume += c.volume()
        }
        volume
    }
}

pub fn riddle_22_2(lines: io::Lines<io::BufReader<File>>) {
    let commands = read_parameter(lines);
    let mut r = Reactor::new();
    for c in commands {
        let cube = Cube::new(c.1.0, c.1.1, c.2.0, c.2.1, c.3.0, c.3.1);
        if c.0 {
            r.insert(&cube);
        } else {
            r.remove(&cube);
        }
    }
    println!("{:?}", r.volume());
}
