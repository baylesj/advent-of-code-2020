use std::convert::TryInto;
use std::fmt;
use std::ops::Add;

pub trait Advance {
    fn advance(self: &mut Self, direction: Direction);
    fn advance_copy(self: &Self, direction: Direction) -> Self;
}

pub trait ArrayLike {
    fn size() -> usize;
    fn get(&self, i: usize) -> i64;
    fn set(&mut self, i: usize, v: i64);
}

pub trait Inverse {
    fn inverse(&self) -> Self;
}

pub trait Matrix2DLike<T> {
    fn create(size: &Point2D) -> Self;
    fn create_with_data(size: &Point2D, data: Vec<T>) -> Self;
    fn size(&self) -> &Point2D;
    fn get(&self, location: &Point2D) -> T;
    fn set(&mut self, location: &Point2D, value: T);
}

pub trait RelativeTurn {
    fn to_left(self: &Self) -> Direction;
    fn to_right(self: &Self) -> Direction;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Matrix2D<T> {
    pub data: Vec<T>,
    pub size: Point2D,
}

impl Default for Direction {
    fn default() -> Self {
        return Direction::Up;
    }
}

impl Inverse for Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
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

impl Advance for Point2D {
    fn advance(self: &mut Self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }
    }

    fn advance_copy(self: &Self, direction: Direction) -> Point2D {
        match direction {
            Direction::Left => Point2D {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Point2D {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Point2D {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Point2D {
                x: self.x,
                y: self.y + 1,
            },
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ArrayLike for Point3D {
    fn size() -> usize {
        3
    }

    fn get(&self, i: usize) -> i64 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("out of bounds"),
        }
    }

    fn set(&mut self, i: usize, v: i64) {
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

impl<T: Copy> Matrix2DLike<T> for Matrix2D<T> {
    fn create(size: &Point2D) -> Self {
        Matrix2D {
            data: Vec::with_capacity((size.x * size.y + 1).try_into().unwrap()),
            size: size.clone(),
        }
    }

    fn create_with_data(size: &Point2D, data: Vec<T>) -> Self {
        Matrix2D {
            data: data,
            size: size.clone(),
        }
    }

    fn size(&self) -> &Point2D {
        &self.size
    }

    fn get(&self, location: &Point2D) -> T {
        assert!(location.x < self.size.x);
        assert!(location.y < self.size.y);
        self.data[(location.y * self.size.x + location.x) as usize]
    }

    fn set(&mut self, location: &Point2D, value: T) {
        assert!(location.x < self.size.x);
        assert!(location.y < self.size.y);

        self.data[(location.y * self.size.x + location.x) as usize] = value;
    }
}

impl<T: fmt::Display + Copy> fmt::Display for Matrix2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = Point2D::default();
        write!(f, "[\n").expect("didn't write!");
        for i in 0..self.size.y {
            current.y = i;
            write!(f, "[").expect("didn't write!");
            for j in 0..self.size.x {
                current.x = j;
                write!(f, "{} ", self.get(&current)).expect("didn't write!");
            }
            write!(f, "]\n").expect("didn't write!");
        }
        write!(f, "]\n").expect("didn't write!");
        Ok(())
    }
}
