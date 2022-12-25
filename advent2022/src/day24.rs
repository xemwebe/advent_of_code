use super::*;
use std::{collections::HashSet, ops::{Add, Sub}};
use num::traits::Zero;
use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> 
where T: Zero+Add+Sub+PartialEq+Eq+Copy {
    fn new() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn plus(&self, x: T, y: T) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    } 

    fn set(&mut self, x: T, y:T) {
        self.x = x;
        self.y = y;
    }
}

impl fmt::Display for Point<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Map {
    pub start: Point<i32>,
    pub end: Point<i32>,
    pub up: Vec<Vec<bool>>,
    pub down: Vec<Vec<bool>>,
    pub right: Vec<Vec<bool>>,
    pub left: Vec<Vec<bool>>,
}

impl Map {
    fn new() -> Self {
        Self {
            start: Point::new(),
            end: Point::new(),
            up: Vec::new(),
            down: Vec::new(),
            right: Vec::new(),
            left: Vec::new(),
        }
    }

    fn is_free(&self, p: &Point<i32>, time: i32) -> bool {
        let dimx = self.up.len() as i32;
        let dimy = self.up[0].len() as i32;
        if p.x == 0 || p.y ==0 || p.y > dimy || p.x > dimx {
            // println!("{p} is out of bounds");
            return *p==self.start || *p==self.end;
        }
        if self.up[((p.x-1 + time) % dimx) as usize][(p.y-1) as usize] {
            // println!("{p} up blocked");
            return false;
        }
        if self.down[((p.x-1 + dimx - time%dimx) % dimx) as usize][(p.y-1) as usize] {
            // println!("{p} down blocked");
            return false;
        }
        if self.left[(p.x-1) as usize][((p.y-1 + time) % dimy) as usize] {
            // println!("{p} left blocked");
            return false;
        }
        if self.right[(p.x-1) as usize][((p.y-1 + dimy - time%dimy) % dimy) as usize] {
            // println!("{p} right blocked");
            return false;
        }
        true
    }
}

fn read_map(mut lines: io::Lines<io::BufReader<File>>) -> Map {
    let mut row = 0;
    let mut map = Map::new();
    let mut xdim = 0;
    while let Some(Ok(s)) = lines.next() {
        let bytes = s.as_bytes();
        if row == 0 {
            for col in 1..bytes.len() {
                if bytes[col] == 46 {
                    map.start.set(row, col as i32);
                    xdim = bytes.len()-2;
                    break;
                }
            }
        } else if bytes[2] == 35 || bytes[3] == 35 {
            for col in 1..bytes.len() {
                if bytes[col] == 46 {
                    map.end.set(row, col as i32);
                    break;
                }
            }
        } else {
            map.up.push(vec![false; xdim]);
            map.down.push(vec![false; xdim]);
            map.left.push(vec![false; xdim]);
            map.right.push(vec![false; xdim]);
            let r = row as usize - 1;
            for col in 0..xdim {
                match bytes[col+1] {
                    94 => map.up[r][col] = true,
                    118 => map.down[r][col] = true,
                    60 => map.left[r][col] = true,
                    62 => map.right[r][col] = true,
                    _ => {},
                }
            }
        }
        row += 1;
    }
    map
}

struct State {
    free: HashSet<Point<i32>>,
    time: i32,
}

impl State {
    fn new(start: Point<i32>) -> Self {
        let mut state = Self {
            free: HashSet::new(),
            time: 0,
        };
        state.free.insert(start);
        state
    }

    fn time_to_get_to(&mut self, end: Point<i32>, map: &Map) {
        loop {
            if self.free.contains(&end) {
                break;
            }
            self.time += 1;
            let mut free = HashSet::new();
            for p in &self.free {
                // println!("checking {p}");
                self.add_to_free(&p, &mut free, map); 
                self.add_to_free(&p.plus(1,0),&mut free, map); 
                if p.x!=0 { 
                    self.add_to_free(&p.plus(0, -1), &mut free, map); 
                    self.add_to_free(&p.plus(-1 ,0), &mut free, map); 
                    self.add_to_free(&p.plus(0,1), &mut free, map);
                }
            }
            // println!("time:  {time}, free: {free:?}");
            //println!("free: {}", free.len());
            if free.is_empty() {
                println!("Sorry, no solution found!");
                break;
            }
            self.free = free;
        }
    }

    fn add_to_free(&self, p: &Point<i32>, free: &mut HashSet<Point<i32>>, map: &Map) {
        if free.contains(p) {
            return;
        }
        if map.is_free(p, self.time) {
            free.insert(p.clone());
        }
    }
}

pub fn riddle_24_1(lines: io::Lines<io::BufReader<File>>) {
    let map = read_map(lines);
    // println!("map : {map:?}");
    let mut state = State::new(map.start.clone());
    state.time_to_get_to(map.end.clone(), &map);
    println!("Solution: {:?}", state.time);
}

pub fn riddle_24_2(lines: io::Lines<io::BufReader<File>>) {
    let mut map = read_map(lines);
    // println!("map : {map:?}");
    let mut state = State::new(map.start.clone());
    state.time_to_get_to(map.end.clone(), &map);
    let start = map.start;
    map.start = map.end;
    map.end = start;
    state.free = HashSet::new();
    state.free.insert(map.start.clone());
    state.time_to_get_to(map.end.clone(), &map);
    let start = map.start;
    map.start = map.end;
    map.end = start;
    state.free = HashSet::new();
    state.free.insert(map.start.clone());
    state.time_to_get_to(map.end.clone(), &map);
    println!("Solution: {:?}", state.time);
}
