#[path = "geometry.rs"]
mod geometry;

use geometry::Dimensions;
use geometry::Update;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "input/day_three.txt";

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

// fn calc_manhattan_distance(a: geometry::Point, b: geometry::Point) -> i32 {
//     ((a.x - b.x) + (a.y - b.y)).abs()
// }

fn update_location(location: &mut geometry::Point, instruction: &Instruction) {
    match instruction {
        Instruction::Left(x) => location.x -= x,
        Instruction::Up(y) => location.y += y,
        Instruction::Right(x) => location.x += x,
        Instruction::Down(y) => location.y -= y,
    }
}

fn calc_bounds(wire_instructions: &Vec<Instruction>) -> geometry::Bounds {
    let mut bounds = geometry::Bounds::default();

    let mut current_location = geometry::Point::default();
    for instruction in wire_instructions {
        update_location(&mut current_location, instruction);
        bounds.update_limits(&current_location);
    }

    bounds
}

fn calc_grid_size(instructions: &Instructions) -> geometry::Point {
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

/*
  Some facts:
    1. Vertical lines can only intersect with horizontal lines. Not strictly
       true, but a good limiting assumption to start with.

    2. Thus we can store our line instructions as a pair of points: source
    point and terminal point.

    Two lines intersect if there is some point where x = x', y = y'
    (0, 0) R75 (75, 0)
    (30, 30) D60 (30, -30)

    Horizontal line: y=0, 0 -> 75
    Vertical line: x'=30, y'-30 -> 30

    intersect if x' in x0 -> x1 and y in y'0 ->y'1
    at x = 30, y = 0 they are the same.

    Simple solution:
    for each line in trail one, put in a map by x, y
    for each line in y 2, check and see if it intersects any pair in line 1.

*/
pub fn solve() -> String {
    let instructions = load_all_instructions().expect("invalid instructions");

    format!("size: {:?}", calc_grid_size(&instructions))
}
