use std::convert::TryInto;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;

pub trait Advance {
    fn advance(self: &mut Self, direction: Direction);
    fn advance_copy(self: &Self, direction: Direction) -> Self;
    fn advance_mult(self: &mut Self, direction: Direction, count: i64);
    fn advance_copy_mult(self: &Self, direction: Direction, count: i64) -> Self;
}

pub trait ArrayLike {
    fn size() -> usize;
    fn get(&self, i: usize) -> i64;
    fn set(&mut self, i: usize, v: i64);
}

pub trait Scalable {
    fn scale(&mut self, factor: i64);
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
        self.advance_mult(direction, 1);
    }

    fn advance_copy(self: &Self, direction: Direction) -> Point2D {
        self.advance_copy_mult(direction, 1)
    }

    fn advance_mult(self: &mut Self, direction: Direction, count: i64) {
        match direction {
            Direction::Left => self.x -= count,
            Direction::Up => self.y -= count,
            Direction::Right => self.x += count,
            Direction::Down => self.y += count,
        }
    }

    fn advance_copy_mult(self: &Self, direction: Direction, count: i64) -> Point2D {
        match direction {
            Direction::Left => Point2D {
                x: self.x - count,
                y: self.y,
            },
            Direction::Up => Point2D {
                x: self.x,
                y: self.y - count,
            },
            Direction::Right => Point2D {
                x: self.x + count,
                y: self.y,
            },
            Direction::Down => Point2D {
                x: self.x,
                y: self.y + count,
            },
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ArrayLike for Point2D {
    fn size() -> usize {
        2
    }

    fn get(&self, i: usize) -> i64 {
        match i {
            0 => self.x,
            1 => self.y,
            _ => panic!("out of bounds"),
        }
    }

    fn set(&mut self, i: usize, v: i64) {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            _ => panic!("out of bounds"),
        }
    }
}

impl Add for Point2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Scalable for Point2D {
    fn scale(&mut self, factor: i64) {
        self.x *= factor;
        self.y *= factor;
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

impl AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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
        for i in 0..self.size.y {
            current.y = i;
            write!(f, "[").expect("didn't write!");
            for j in 0..self.size.x {
                current.x = j;
                write!(f, "{} ", self.get(&current)).expect("didn't write!");
            }
            write!(f, "]\n").expect("didn't write!");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn advance_in_place() {
        let mut point = Point2D { x: 0, y: 0 };
        point.advance(Direction::Left);
        assert_eq!(Point2D { x: -1, y: 0 }, point);
        // Down is a matter of perspective, in YAGM down is positive increment.
        point.advance(Direction::Down);
        assert_eq!(Point2D { x: -1, y: 1 }, point);
        point.advance(Direction::Right);
        assert_eq!(Point2D { x: 0, y: 1 }, point);
        point.advance(Direction::Up);
        assert_eq!(Point2D { x: 0, y: 0 }, point);
    }

    #[test]
    pub fn advance_multiple_in_place() {
        let mut point = Point2D { x: 0, y: 0 };
        point.advance_mult(Direction::Left, 3);
        assert_eq!(Point2D { x: -3, y: 0 }, point);
        point.advance_mult(Direction::Down, 4);
        assert_eq!(Point2D { x: -3, y: 4 }, point);
        point.advance_mult(Direction::Right, 13);
        assert_eq!(Point2D { x: 10, y: 4 }, point);
        point.advance_mult(Direction::Up, 8);
        assert_eq!(Point2D { x: 10, y: -4 }, point)
    }

    #[test]
    pub fn advance_a_copy() {
        let point = Point2D { x: 0, y: 0 };

        let copy_left = point.advance_copy(Direction::Left);
        assert_eq!(Point2D { x: -1, y: 0 }, copy_left);

        let copy_down = point.advance_copy(Direction::Down);
        assert_eq!(Point2D { x: 0, y: 1 }, copy_down);

        let copy_right = point.advance_copy(Direction::Right);
        assert_eq!(Point2D { x: 1, y: 0 }, copy_right);

        let copy_up = point.advance_copy(Direction::Up);
        assert_eq!(Point2D { x: 0, y: -1 }, copy_up);
    }
}
