use super::*;

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    value: i32,
}

impl Move {
    fn parse(s: &str) -> Result<Move> {
        let mut parts = s.split(" ");
        let direction = match parts.next() {
            Some("forward") => Ok(Direction::Forward),
            Some("up") => Ok(Direction::Up),
            Some("down") => Ok(Direction::Down),
            _ => Err(anyhow!("Unknown direction"))
        }?;
        if let Some(v) = parts.next() {
            Ok(Move{
                direction,
                value: v.parse::<i32>()? 
            })
        } else {
            Err(anyhow!("Invalid direction value"))
        }
    }
}

pub fn riddle_2_1(lines: io::Lines<io::BufReader<File>>) {
    let moves: Vec<Move> = lines.into_iter()
    .filter_map(|s| s.ok())
    .filter_map(|s| Move::parse(&s).ok() )
    .collect();

    let mut pos = Position{horizontal: 0, depth: 0, aim: 0};
    for m in moves {
        match m.direction {
            Direction::Forward => pos.horizontal += m.value,
            Direction::Up => pos.depth -= m.value,
            Direction::Down => pos.depth += m.value,
        }
    }
    println!("{:?}", pos);    
}

pub fn riddle_2_2(lines: io::Lines<io::BufReader<File>>) {
    let moves: Vec<Move> = lines.into_iter()
    .filter_map(|s| s.ok())
    .filter_map(|s| Move::parse(&s).ok() )
    .collect();

    let mut pos = Position{horizontal: 0, depth: 0, aim: 0};
    for m in moves {
        match m.direction {
            Direction::Forward => { 
                pos.horizontal += m.value;
                pos.depth += pos.aim*m.value;
            }
            Direction::Up => pos.aim -= m.value,
            Direction::Down => pos.aim += m.value,
        }
    }
    println!("{:?}", pos);    
}
