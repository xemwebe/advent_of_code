use std::{
    fs::File, 
    io,
    collections::HashSet,
    cmp::Ordering,
};

pub fn execute(part: u32, lines: io::Lines<io::BufReader<File>>) -> String {
    match part {
        1 => riddle_1(lines),
        2 => riddle_2(lines),
        _ => format!("Error: part {part} not found!"),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
    fn new(v: &[usize]) -> Self {
        Self{ 
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}
impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.z == other.z {
            if self.y == other.y {
                self.x.cmp(&other.x)
            } else {
                self.y.cmp(&other.y)
            }
        } else {
            self.z.cmp(&other.z)
        }
    }
}

impl PartialOrd for Point3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: Point3D,
    end: Point3D
}

impl Brick {
    fn xy_overlaps_with(&self, other: &Brick) -> bool {
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                if x>=other.start.x && x<=other.end.x
                && y>=other.start.y && y<=other.end.y {
                    return true;
                }
            }
        }
        false
    }
}
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Solver {
    bricks: Vec<Brick>,
    top_layer: Vec<HashSet<usize>>,
    bottom_layer: Vec<HashSet<usize>>
}

impl Solver {
    fn new(bricks: Vec<Brick>, top_layer: Vec<HashSet<usize>>, bottom_layer: Vec<HashSet<usize>>) -> Self {
        Self{
            bricks,
            top_layer,
            bottom_layer
        }
    }

    fn verify_brick_stack(&self, check_for_no_supporter: bool) {
        for i in 0..self.bricks.len()-1 {
            let brick_a = &self.bricks[i];
            for z in brick_a.start.z..=brick_a.end.z {
                for j in i+1..self.bricks.len() {
                    let brick_b = &self.bricks[j];
                    if z>=brick_b.start.z && z<=brick_b.end.z {
                        if brick_a.xy_overlaps_with(&brick_b) {
                            panic!("overlapping bricks!")
                        }
                    }
                }
            }
        }

        if check_for_no_supporter {
            for i in 0..self.bricks.len() {
                let brick_a = &self.bricks[i];
                if brick_a.start.z == 1 {
                    continue;
                }

                let mut has_supporter = false;
                for j in 0..self.bricks.len() {
                    if i==j { continue; }
                    let brick_b = &self.bricks[j];
                    if brick_b.end.z+1 == brick_a.start.z {
                        if brick_a.xy_overlaps_with(&brick_b) {
                            has_supporter = true;
                            break;
                        }
                    }
                }
                if !has_supporter {
                    panic!("free floating brick found");
                }
            }    
        }
    }

    fn let_bricks_fall(&mut self) -> u64 {
        let mut fallen = 0;
        self.verify_brick_stack(false);
        let bottom_layer_len = self.bottom_layer.len();
        for i in 2..bottom_layer_len {
            let mut falling_bricks = Vec::new();
            for brick in &self.bottom_layer[i] {
                let mut z_new = i;
                let mut found_support = false;
                for j in (1..i).rev() {
                    z_new = j+1;
                    for lower_brick in &self.top_layer[j] {
                        if self.bricks[*lower_brick].xy_overlaps_with(&self.bricks[*brick]) {
                            found_support = true;
                            break;
                        }
                    }
                    if found_support {
                        break;
                    }
                }
                if !found_support {
                    z_new = 1;
                }
                if z_new < i {
                    fallen += 1;
                    falling_bricks.push((*brick, i-z_new));
                }
            }
            for falling in falling_bricks {
                self.bottom_layer[i].remove(&falling.0);
                self.top_layer[self.bricks[falling.0].end.z].remove(&falling.0);
                self.bricks[falling.0].start.z -= falling.1;
                self.bricks[falling.0].end.z -= falling.1;
                self.bottom_layer[self.bricks[falling.0].start.z].insert(falling.0);
                self.top_layer[self.bricks[falling.0].end.z].insert(falling.0);
            }
        }
        self.verify_brick_stack(true);
        fallen
    }

    fn count_desintegratable(&self) -> u64 {
        let mut critical_bricks = HashSet::new();
        for brick in &self.bricks {
            if brick.start.z == 1 { continue; }
            let mut supporting = None;
            for lower_brick in &self.top_layer[brick.start.z-1] 
            {
                if brick.xy_overlaps_with(&self.bricks[*lower_brick]) {
                    if supporting.is_none() {
                        supporting = Some(*lower_brick);
                    } else {
                        supporting = None;
                        break;
                    }
                }
            }
            if let Some(supporting) = supporting {
                critical_bricks.insert(supporting);
            }
        }
        (self.bricks.len() - critical_bricks.len()) as u64
    }

    fn count_chain_reaction(&self) -> u64 {
        let mut critical_bricks = HashSet::new();
        for brick in &self.bricks {
            if brick.start.z == 1 { continue; }
            let mut supporting = None;
            for lower_brick in &self.top_layer[brick.start.z-1] 
            {
                if brick.xy_overlaps_with(&self.bricks[*lower_brick]) {
                    if supporting.is_none() {
                        supporting = Some(*lower_brick);
                    } else {
                        supporting = None;
                        break;
                    }
                }
            }
            if let Some(supporting) = supporting {
                critical_bricks.insert(supporting);
            }
        }
        let mut fallen = 0;
        for i in critical_bricks {
            let mut bricks = Vec::new();
            for j in 0..self.bricks.len() {
                if i==j { continue; }
                bricks.push(self.bricks[j].clone());
            }
            let top_layer = copy_hash_set(&self.top_layer, i);
            let bottom_layer = copy_hash_set(&self.bottom_layer, i);
            let mut fallen_solver = Solver::new(bricks, top_layer, bottom_layer);
            fallen += fallen_solver.let_bricks_fall();
        }
        fallen
    }
}

fn copy_hash_set(sets: &[HashSet<usize>], exclude: usize) -> Vec<HashSet<usize>> {
    let mut new_sets = Vec::new();
    for s in sets {
        let mut new_set = HashSet::new();
        for idx in s {
            if *idx == exclude { continue; }
            if *idx > exclude {
                new_set.insert(idx-1);
            } else {
                new_set.insert(*idx);
            }
        }
        new_sets.push(new_set);
    }
    new_sets
}

fn parse_input(lines: io::Lines<io::BufReader<File>>) -> Solver {
    let mut bricks = Vec::new();
    let mut bottom = Vec::new();
    let mut top = Vec::new();
    for l in lines {
        let line = l.unwrap();
        let parts: Vec<&str> = line.split('~').collect();
        let start: Vec<usize> = parts[0].split(',')
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let end: Vec<usize> = parts[1].split(',')
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        let start = Point3D::new(&start);
        let end = Point3D::new(&end);
        let idx = bricks.len();
        for _ in top.len()..=end.z {
            top.push(HashSet::new());
        }
        for _ in bottom.len()..=start.z {
            bottom.push(HashSet::new());
        }
        top[end.z].insert(idx);
        bottom[start.z].insert(idx);
        bricks.push(Brick{ start, end });
    }
    Solver::new(bricks, top, bottom)
}

fn riddle_1(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines);
    solver.let_bricks_fall();
    let sum = solver.count_desintegratable();
    format!("{sum}")
}

fn riddle_2(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut solver = parse_input(lines);
    solver.let_bricks_fall();
    let sum = solver.count_chain_reaction();
    format!("{sum}")
}

#[cfg(test)]
mod test {
    use crate::read_lines;
    use super::execute;

    #[test]
    fn test_2023_22_1() {
        let lines = read_lines("data/2023/22.txt").unwrap();
        let result = execute(1, lines);
        assert_eq!(result, "439");
    }

    #[test]
    fn test_2023_22_2() {
        let lines = read_lines("data/2023/22.txt").unwrap();
        let result = execute(2, lines);
        assert_eq!(result, "43056");
    }
}
