use std::cmp;
use std::ops::Sub;

#[derive(Debug, Default)]
struct Point {
    x: i32,
    y: i32,
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

#[derive(Debug, Default)]
struct Bounds {
    upper_right: Point,
    lower_left: Point,
}

trait Dimensions {
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

trait Update {
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
struct Instructions {
    first_wire: Vec<String>,
    second_wire: Vec<String>,
}

fn calc_manhattan_distance(a: Point, b: Point) -> i32 {
    ((a.x - b.x) + (a.y - b.y)).abs()
}

fn update_location(location: &mut Point, instruction: &str) {
    let magnitude: i32 = instruction[1..].parse().unwrap();
    match instruction[..0].chars().next().unwrap() {
        'U' => location.y += magnitude,
        'D' => location.y -= magnitude,
        'L' => location.x -= magnitude,
        'R' => location.x += magnitude,
        _ => panic!("Unknown instruction"),
    }
}

fn calc_bounds(wire_instructions: &Vec<String>) -> Bounds {
    let mut bounds = Bounds::default();

    let mut current_location = Point::default();
    for instruction in wire_instructions {
        update_location(&mut current_location, instruction);
        bounds.update_limits(&current_location);
    }

    bounds
}

fn calc_grid_size(instructions: Instructions) -> Point {
    let mut first_bounds = calc_bounds(&instructions.first_wire);
    let second_bounds = calc_bounds(&instructions.first_wire);

    first_bounds.update_limits(&second_bounds.lower_left);
    first_bounds.update_limits(&second_bounds.upper_right);
    return first_bounds.dimensions();
}

pub fn solve() -> String {
    format!("{}", calc_grid_size())
}
