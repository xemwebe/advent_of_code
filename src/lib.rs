use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionType {
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
    Right,
    UpRight,
}

#[derive(Debug, Clone)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn from_type(dir_type: DirectionType) -> Self {
        let x = match dir_type {
            DirectionType::UpLeft | DirectionType::Left | DirectionType::DownLeft => -1,
            DirectionType::DownRight | DirectionType::Right | DirectionType::UpRight => 1,
            _ => 0,
        };
        let y = match dir_type {
            DirectionType::UpLeft | DirectionType::Up | DirectionType::UpRight => -1,
            DirectionType::DownRight | DirectionType::Down | DirectionType::DownLeft => 1,
            _ => 0,
        };
        Self { x, y }
    }

    pub fn to_type(&self) -> Option<DirectionType> {
        match (self.x, self.y) {
            (-1, -1) => Some(DirectionType::UpLeft),
            (-1, 0) => Some(DirectionType::Left),
            (-1, 1) => Some(DirectionType::DownLeft),
            (0, -1) => Some(DirectionType::Up),
            (0, 1) => Some(DirectionType::Down),
            (1, -1) => Some(DirectionType::UpRight),
            (1, 0) => Some(DirectionType::Right),
            (1, 1) => Some(DirectionType::DownRight),
            _ => None,
        }
    }

    pub fn turn_right(&mut self) {
        let x = self.x;
        self.x = -self.y;
        self.y = x;
    }
}

impl Mul<i32> for &Direction {
    type Output = Direction;

    fn mul(self, rhs: i32) -> Self::Output {
        Direction {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Direction> for i32 {
    type Output = Direction;

    fn mul(self, rhs: Direction) -> Self::Output {
        Direction {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn move_by(&mut self, dir: &Direction) {
        self.x += dir.x;
        self.y += dir.y;
    }

    pub fn try_move_by(&self, dir: &Direction, n: usize, m: usize) -> Option<Position> {
        let new_pos = self + dir;
        if new_pos.check_on_grid(n, m) {
            Some(new_pos)
        } else {
            None
        }
    }

    pub fn check_move_by(&self, dir: &Direction, n: usize, m: usize) -> bool {
        (self + dir).check_on_grid(n, m)
    }

    pub fn check_on_grid(&self, n: usize, m: usize) -> bool {
        !(self.x < 0 || self.y < 0 || self.x >= n as i32 || self.y >= m as i32)
    }
}

impl Add<&Direction> for &Position {
    type Output = Position;

    fn add(self, dir: &Direction) -> Self::Output {
        Position {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl Add<&Position> for &Direction {
    type Output = Position;

    fn add(self, pos: &Position) -> Self::Output {
        Position {
            x: self.x + pos.x,
            y: self.y + pos.y,
        }
    }
}

impl Sub<&Position> for &Position {
    type Output = Direction;

    fn sub(self, other: &Position) -> Self::Output {
        Direction {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<&Direction> for &Position {
    type Output = Position;

    fn sub(self, dir: &Direction) -> Self::Output {
        Position {
            x: self.x - dir.x,
            y: self.y - dir.y,
        }
    }
}
