/// The solution here is overly complex.
/// A much simple and less error-prone solution would use a fixed set of block pattern maps
/// instead of hardcoding each block as a separate struct. I was just in the mood to use traits instead.
/// However, it took far to long to implement the structs and code the methods.
/// Still, I was very surprised that my rank wasn't that bad...
/// 

use super::*;
use std::{collections::{HashMap,HashSet}};

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
enum BlockType {
    Minus,
    Plus,
    InvL,
    Bar,
    Square,
}
trait Block {
    fn block_type(&self) -> BlockType;

    fn push(&mut self, blow: i32, floor: &Floor);

    fn fall_down(&mut self, floor: &mut Floor) -> bool;

    fn next_block(&self, floor: &Floor) -> Box<dyn Block>;
}

struct MinusBlock {
    position: (usize,usize)
}

impl MinusBlock {
    fn new(floor: &Floor) -> Self {
        Self {
            position: (2, floor.len()+3)
        }
    }
}

impl Block for MinusBlock {
    fn block_type(&self) -> BlockType {
        BlockType::Minus
    }

    fn push(&mut self, blow: i32, floor: &Floor) {
        let (x,y) = self.position;
        let mut new_x = if blow<0 && x>0 {
            x-1
        } else if blow>0 && x<3 {
            x+1
        } else {
            x
        };
        if y < floor.len() && (floor[y][new_x] != 0 || floor[y][new_x+3] != 0) {
            new_x = x;
        }
        self.position.0 = new_x;
    }

    fn fall_down(&mut self, floor: &mut Floor) -> bool {
        let (x,y) = self.position;
        let is_blocked = y <= floor.len()
            && (y == 0 || floor[y-1][x] + floor[y-1][x+1] + floor[y-1][x+2] + floor[y-1][x+3] != 0);

        if is_blocked {
            for _ in floor.len()..=y {
                floor.push(vec![0; 7]);
            }
            for xx in x..=(x+3) {
                floor[y][xx] = 1;
            }
            false
        } else {
            self.position.1 -= 1;
            true
        }
    }

    fn next_block(&self, floor: &Floor) -> Box<dyn Block> {
        Box::new(PlusBlock::new(floor))
    }
}

struct PlusBlock {
    position: (usize,usize)
}

impl PlusBlock {
    fn new(floor: &Floor) -> Self {
        Self {
            position: (2, floor.len()+3)
        }
    }
}

impl Block for PlusBlock {
    fn block_type(&self) -> BlockType {
        BlockType::Plus
    }

    fn push(&mut self, blow: i32, floor: &Floor) {
        let (x,y) = self.position;
        let mut new_x = if blow<0 && x>0 {
            x-1
        } else if blow>0 && x<4 {
            x+1
        } else {
            x
        };
        if (y < floor.len() && floor[y][new_x+1] != 0)
        || (y+1 < floor.len() && (floor[y+1][new_x] + floor[y+1][new_x+2] != 0) ) 
        || (y+2 < floor.len() && (floor[y+2][new_x+1] != 0) ) {
            new_x = x;
        }
        self.position.0 = new_x;
    }

    fn fall_down(&mut self, floor: &mut Floor) -> bool {
        let (x,y) = self.position;
        let is_blocked = y <= floor.len() 
            && ( y == 0  || floor[y-1][x+1] !=0
            || (y<floor.len() && floor[y][x] + floor[y][x+2] != 0));

        if is_blocked {
            for _ in floor.len()..=y+2 {
                floor.push(vec![0; 7]);
            }
            floor[y][x+1] = 1;
            floor[y+1][x] = 1;
            floor[y+1][x+1] = 1;
            floor[y+1][x+2] = 1;
            floor[y+2][x+1] = 1;
            false
        } else {
            self.position.1 -= 1;
            true
        }
    }

    fn next_block(&self, floor: &Floor) -> Box<dyn Block> {
        Box::new(InvLBlock::new(floor))
    }
}

struct InvLBlock {
    position: (usize,usize)
}

impl InvLBlock {
    fn new(floor: &Floor) -> Self {
        Self {
            position: (2, floor.len()+3)
        }
    }
}

impl Block for InvLBlock {
    fn block_type(&self) -> BlockType {
        BlockType::InvL
    }

    fn push(&mut self, blow: i32, floor: &Floor) {
        let (x,y) = self.position;
        let mut new_x = if blow<0 && x>0 {
            x-1
        } else if blow>0 && x<4 {
            x+1
        } else {
            x
        };
        if (y < floor.len() && (floor[y][new_x] + floor[y][new_x+2] != 0) )
        || (y+1 < floor.len() && floor[y+1][new_x+2] != 0) 
        || (y+2 < floor.len() && floor[y+2][new_x+2] != 0) {
            new_x = x;
        }
        self.position.0 = new_x;
    }

    fn fall_down(&mut self, floor: &mut Floor) -> bool {
        let (x,y) = self.position;
        let is_blocked = y<=floor.len()
            && ( y==0 || floor[y-1][x] + floor[y-1][x+1] + floor[y-1][x+2] != 0);

        if is_blocked {
            for _ in floor.len()..=y+2 {
                floor.push(vec![0; 7]);
            }
            floor[y][x] = 1;
            floor[y][x+1] = 1;
            floor[y][x+2] = 1;
            floor[y+1][x+2] = 1;
            floor[y+2][x+2] = 1;
            false
        } else {
            self.position.1 -= 1;
            true
        }
    }

    fn next_block(&self, floor: &Floor) -> Box<dyn Block> {
        Box::new(BarBlock::new(floor))
    }
}

struct BarBlock {
    position: (usize,usize)
}

impl BarBlock {
    fn new(floor: &Floor) -> Self {
        Self {
            position: (2, floor.len()+3)
        }
    }
}

impl Block for BarBlock {
    fn block_type(&self) -> BlockType {
        BlockType::Bar
    }

    fn push(&mut self, blow: i32, floor: &Floor) {
        let (x,y) = self.position;
        let mut new_x = if blow<0 && x>0 {
            x-1
        } else if blow>0 && x<6 {
            x+1
        } else {
            x
        };
        if (y < floor.len() && floor[y][new_x] != 0)
        || (y+1 < floor.len() && floor[y+1][new_x] != 0) 
        || (y+2 < floor.len() && floor[y+2][new_x] != 0) 
        || (y+3 < floor.len() && floor[y+3][new_x] != 0) {
            new_x = x;
        }
        self.position.0 = new_x;
    }

    fn fall_down(&mut self, floor: &mut Floor) -> bool {
        let (x,y) = self.position;
        let is_blocked = (y <= floor.len())
            && ( self.position.1 ==0 || floor[y-1][x] != 0 );

        if is_blocked {
            for _ in floor.len()..=y+3 {
                floor.push(vec![0; 7]);
            }
            floor[y][x] = 1;
            floor[y+1][x] = 1;
            floor[y+2][x] = 1;
            floor[y+3][x] = 1;
            false
        } else {
            self.position.1 -= 1;
            true
        }
    }

    fn next_block(&self, floor: &Floor) -> Box<dyn Block> {
        Box::new(SquareBlock::new(floor))
    }
}
struct SquareBlock {
    position: (usize,usize)
}

impl SquareBlock {
    fn new(floor: &Floor) -> Self {
        Self {
            position: (2, floor.len()+3)
        }
    }
}

impl Block for SquareBlock {
    fn block_type(&self) -> BlockType {
        BlockType::Square
    }

    fn push(&mut self, blow: i32, floor: &Floor) {
        let (x,y) = self.position;
        let mut new_x = if blow<0 && x>0 {
            x-1
        } else if blow>0 && x<5 {
            x+1
        } else {
            x
        };
        if (y < floor.len() && (floor[y][new_x] + floor[y][new_x+1] != 0) )
        || (y+1 < floor.len() && (floor[y+1][new_x] + floor[y+1][new_x+1] != 0) ) {
            new_x = x;
        }
        self.position.0 = new_x;
    }

    fn fall_down(&mut self, floor: &mut Floor) -> bool {
        let (x,y) = self.position;
        let is_blocked = (y <= floor.len()) 
            && ( y == 0 || (floor[y-1][x] + floor[y-1][x+1] != 0) ); 

        if is_blocked {
            for _ in floor.len()..=y+1 {
                floor.push(vec![0; 7]);
            }
            floor[y][x] = 1;
            floor[y][x+1] = 1;
            floor[y+1][x] = 1;
            floor[y+1][x+1] = 1;
            false
        } else {
            self.position.1 -= 1;
            true
        }
    }

    fn next_block(&self, floor: &Floor) -> Box<dyn Block> {
        Box::new(MinusBlock::new(floor))
    }
}

type Floor = Vec<Vec<usize>>;

fn print_floor(floor: &Floor) {
    for f in floor.into_iter().rev() {
        print!("|");
        for b in f {
            if *b==0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("|");
    }
    println!("+-------+\n");
}

pub fn riddle_17_1(mut lines: io::Lines<io::BufReader<File>>) {
    let wind: Vec<i32> = lines
        .next().unwrap().unwrap()
        .as_bytes()
        .into_iter()
        .map(|x| (*x as i32)-61)
        .collect();
    let mut floor = Vec::new();
    let mut blow_idx = 0;
    let mut current_block: Box<dyn Block> = Box::new(MinusBlock::new(&floor));
    let mut block_counter = 1;
    loop {
        let blow = wind[blow_idx];
        blow_idx = (blow_idx+1) % wind.len();
        current_block.push(blow, &floor);
        if !current_block.fall_down(&mut floor) {
            current_block = current_block.next_block(&floor);
            if block_counter == 2022 {
                break;
            }
            block_counter += 1;
        }
    }
    print_floor(&floor);
    println!("The solution is: {:?}", floor.len());
}

struct State {
    top_floor: Floor,
    block_number: usize,
    height: usize,
}

fn top_of_floor(floor: &Floor) -> Floor {
    let mut top = floor.len();
    let mut found = HashSet::new();
    let mut top_floor = Vec::new();
    loop {
        if top == 0 {
            top_floor.push(vec![1; 7]);
            return top_floor;
        }
        top -= 1;
        for i in 0..7 {
            if floor[top][i]==1  {
                found.insert(i);
            }
        }
        top_floor.push(floor[top].clone());
        if found.len() == 7 {
            return top_floor;
        }
    }
}

fn floor_equal(floor1: &Floor, floor2: &Floor) -> bool {
    if floor1.len() != floor2.len()  {
        return false;
    }
    for i in 0..floor1.len()  {
        for j in 0..7 {
            if floor1[i][j] != floor2[i][j] {
                return false;
            }
        }
    }
    true
}

pub fn riddle_17_2(mut lines: io::Lines<io::BufReader<File>>) {
    let wind: Vec<i32> = lines
        .next().unwrap().unwrap()
        .as_bytes()
        .into_iter()
        .map(|x| (*x as i32)-61)
        .collect();
    let mut floor = Vec::new();
    let mut blow_idx = 0;
    let mut current_block: Box<dyn Block> = Box::new(MinusBlock::new(&floor));
    let mut block_counter: usize = 1;
    let mut combinations: HashMap<(usize, BlockType), State> = HashMap::new();
    let mut block_stopper = 0;
    let mut skipped_height = 0;
    let total_block_num = 1000000000000_usize;
    loop {
        let blow = wind[blow_idx];
        blow_idx = (blow_idx+1) % wind.len();
        current_block.push(blow, &floor);
        if !current_block.fall_down(&mut floor) {
            let sample = (blow_idx, current_block.block_type());
            if combinations.contains_key(&sample) {
                let top_floor = top_of_floor(&floor);
                if floor_equal(&top_floor, &combinations[&sample].top_floor) && blow_idx == 2434 {
                    let state = &combinations[&sample];
                    let delta_block_num = block_counter - state.block_number;
                    let delta_height = floor.len() - state.height;
                    let blocks_to_skip = (total_block_num - block_counter) / delta_block_num;
                    skipped_height = blocks_to_skip * delta_height;
                    block_stopper = block_counter + (total_block_num - block_counter) % delta_block_num;
                }
            } else {
                combinations.insert(sample, State {
                    top_floor: top_of_floor(&floor),
                    block_number: block_counter,
                    height: floor.len()
                });
            }
            if block_counter == block_stopper {
                break;
            }
            current_block = current_block.next_block(&floor);
            block_counter += 1;
        }
    }
    let total_height = floor.len() + skipped_height;
    println!("The solution is: {:?}", total_height);
}
