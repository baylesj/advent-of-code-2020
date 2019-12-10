use std::cmp;
use std::ops::Sub;

#[derive(Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub trait CalcDistance {
    fn distance(self: Self, other: Self) -> i32;
}

impl CalcDistance for Point {
    fn distance(self: Self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Default)]
pub struct Bounds {
    pub upper_right: Point,
    pub lower_left: Point,
}

pub trait Dimensions {
    fn dimensions(self) -> Point;
}

impl Dimensions for Bounds {
    fn dimensions(self) -> Point {
        Point {
            x: self.upper_right.x - self.lower_left.x,
            y: self.upper_right.y - self.lower_left.y,
        }
    }
}

pub trait Update {
    fn update_limits(&mut self, location: &Point);
}

impl Update for Bounds {
    fn update_limits(&mut self, location: &Point) {
        if location.x < 0 {
            self.lower_left.x = cmp::min(self.lower_left.x, location.x);
        } else {
            self.upper_right.x = cmp::max(self.lower_left.x, location.x);
        }
        if location.y < 0 {
            self.lower_left.y = cmp::min(self.lower_left.y, location.y);
        } else {
            self.upper_right.y = cmp::max(self.lower_left.y, location.y);
        }
    }
}
