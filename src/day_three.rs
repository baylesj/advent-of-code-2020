use std::cmp;
use std::fs;
use std::ops::Sub;
use std::str::FromStr;

const INPUT_FILENAME: &str = "input/day_three.txt";

// TODO: move Point, Bounds to geometry mod.
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

enum Instruction {
    Left(i32),
    Up(i32),
    Right(i32),
    Down(i32),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let magnitude: i32 = s[1..].parse::<i32>().expect("Invalid magnitude");
        match s.chars().nth(0).expect("Invalid instruction") {
            'L' => Ok(Instruction::Left(magnitude)),
            'U' => Ok(Instruction::Up(magnitude)),
            'R' => Ok(Instruction::Right(magnitude)),
            'D' => Ok(Instruction::Down(magnitude)),
            _ => Err("Invalid direction code"),
        }
    }
}

struct Instructions {
    first_wire: Vec<Instruction>,
    second_wire: Vec<Instruction>,
}

impl FromStr for Instructions {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let first_instructions = lines
            .next()
            .expect("invalid line")
            .split(",")
            .map(|x| Instruction::from_str(x).expect("parse error"))
            .collect();
        let second_instructions = lines
            .next()
            .expect("invalid line")
            .split(",")
            .map(|x| Instruction::from_str(x).expect("parse error"))
            .collect();
        Ok(Instructions {
            first_wire: first_instructions,
            second_wire: second_instructions,
        })
    }
}

// fn calc_manhattan_distance(a: Point, b: Point) -> i32 {
//     ((a.x - b.x) + (a.y - b.y)).abs()
// }

fn update_location(location: &mut Point, instruction: &Instruction) {
    match instruction {
        Instruction::Left(x) => location.x -= x,
        Instruction::Up(y) => location.y += y,
        Instruction::Right(x) => location.x += x,
        Instruction::Down(y) => location.y -= y,
    }
}

fn calc_bounds(wire_instructions: &Vec<Instruction>) -> Bounds {
    let mut bounds = Bounds::default();

    let mut current_location = Point::default();
    for instruction in wire_instructions {
        update_location(&mut current_location, instruction);
        bounds.update_limits(&current_location);
    }

    bounds
}

fn calc_grid_size(instructions: &Instructions) -> Point {
    let mut first_bounds = calc_bounds(&instructions.first_wire);
    let second_bounds = calc_bounds(&instructions.second_wire);

    first_bounds.update_limits(&second_bounds.lower_left);
    first_bounds.update_limits(&second_bounds.upper_right);
    return first_bounds.dimensions();
}

// TODO: refactor file ops into separate mod?
fn load_all_instructions() -> Result<Instructions, &'static str> {
    let lines = fs::read_to_string(INPUT_FILENAME).expect("invalid file");
    Instructions::from_str(&lines)
}

pub fn solve() -> String {
    let instructions = load_all_instructions().expect("invalid instructions");

    format!("size: {:?}", calc_grid_size(&instructions))
}
