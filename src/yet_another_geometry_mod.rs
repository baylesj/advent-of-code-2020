use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point3D {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub trait ArrayLike {
    fn size() -> usize;
    fn get(&self, i: usize) -> i128;
    fn set(&mut self, i: usize, v: i128);
}

impl ArrayLike for Point3D {
    fn size() -> usize {
        3
    }

    fn get(&self, i: usize) -> i128 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("out of bounds"),
        }
    }

    fn set(&mut self, i: usize, v: i128) {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            2 => self.z = v,
            _ => panic!("out of bounds"),
        }
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        return Direction::Up;
    }
}

pub trait RelativeTurn {
    fn to_left(self: &Self) -> Direction;
    fn to_right(self: &Self) -> Direction;
}

impl RelativeTurn for Direction {
    fn to_left(self: &Self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn to_right(self: &Self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub trait Advance {
    fn advance(self: &mut Self, direction: Direction);
}

impl Advance for Point2D {
    fn advance(self: &mut Self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }
    }
}
