use std::{
    fs::File, 
    io,
    collections::{VecDeque, HashMap},
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

struct Solver {
    map: Vec<Vec<u8>>,
}

impl Solver {
    fn new(map: Vec<Vec<u8>>) -> Self {
        Self {
            map
        }
    }

    fn sum(&self, steps: i32) -> i32 {
        let mut reached = vec![vec![false; self.map[0].len()]; self.map.len()];
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if self.map[x][y] == b'S' {
                    reached[x][y] = true;
                }
            }
        }

        for _ in 0..steps {
            let mut new_reached = vec![vec![false; self.map[0].len()]; self.map.len()];
            for x in 0..self.map.len() {
                for y in 0..self.map[x].len() {
                    if reached[x][y] {
                        if x>0 && self.map[x-1][y] != b'#' {
                            new_reached[x-1][y] = true;
                        }
                        if x+1<self.map.len() && self.map[x+1][y] != b'#' {
                            new_reached[x+1][y] = true;
                        }
                        if y>0 && self.map[x][y-1] != b'#' {
                            new_reached[x][y-1] = true;
                        }
                        if y+1<self.map[x].len() && self.map[x][y+1] != b'#' {
                            new_reached[x][y+1] = true;
                        }
                    }
                }
            }
            reached = new_reached;
        }
        reached.iter().map(|row|  row.iter().map(|cell| if *cell { 1 } else { 0 }).sum::<i32>() ).sum()
    }

    fn infinite_sum(&self, steps: i64) -> i64 {
        let max_sum = if steps%2 == 1 {
            // 4*3 + 4*5 + 4*7+ ... + 4*step
            // steps = 2n+1  => n = (steps - 1) / 2 
            //4 + 4 * ((steps-1)/2+1)*(steps-1)/2
            //4 + (steps-1 + 2)(steps-1)
            //4 + (steps +1) *(steps -1)
            //4+ steps*steps -1
            3 + steps*steps
        } else {
            1 + 4*2 + 4*4
        };
        max_sum
    }
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().to_vec());
    }
    let solver = Solver::new(map);
    let sum = solver.sum(64);
    format!("{sum}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut map: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        map.push(l.unwrap().as_bytes().to_vec());
    }
    println!("x: {}, y: {}, x*y: {}", map.len(), map[0].len(), map.len()*map[0].len());
    let solver = Solver::new(map);
    let sum = solver.infinite_sum(26501365);
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_21_1() {
        let lines = read_lines("data/2023/21.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "3724");
    }

    #[test]
    fn test_2023_21_2() {
        let lines = read_lines("data/2023/21.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "134549294799713");
    }
}
