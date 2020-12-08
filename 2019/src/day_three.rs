use geo;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &'static str = "input/day_three.txt";
// TODO: doesn't work on day_three_sample.txt. Looking at the implementation
// implies there's an offset issue from the original advent source?
//const INPUT_FILENAME: &'static str = "input/day_three_sample.txt";
//const INPUT_FILENAME: &'static str = "input/day_three_sample_two.txt";

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

#[derive(Debug)]
struct WireSegments {
    first_wire: geo::LineString<f64>,
    second_wire: geo::LineString<f64>,
}

fn process_instruction(
    begin: geo::Coordinate<f64>,
    instruction: &Instruction,
) -> geo::Coordinate<f64> {
    let mut end: geo::Coordinate<f64> = begin;
    match instruction {
        Instruction::Left(x) => end.x -= *x as f64,
        Instruction::Up(y) => end.y += *y as f64,
        Instruction::Right(x) => end.x += *x as f64,
        Instruction::Down(y) => end.y -= *y as f64,
    }
    end
}

fn parse_wire(lines: &mut std::str::Lines) -> geo::LineString<f64> {
    let instructions: Vec<Instruction> = lines
        .next()
        .expect("invalid line")
        .split(",")
        .map(|x| Instruction::from_str(x).expect("parse error"))
        .collect();

    // TODO: set size in advance
    // TODO: aggregate mapper?
    let mut points: Vec<geo::Coordinate<f64>> = vec![];
    let mut current_location = geo::Coordinate::<f64> { x: 0.0, y: 0.0 };
    for inst in instructions {
        points.push(current_location);
        current_location = process_instruction(current_location, &inst);
    }
    geo::LineString(points)
}

impl FromStr for WireSegments {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        Ok(WireSegments {
            first_wire: parse_wire(&mut lines),
            second_wire: parse_wire(&mut lines),
        })
    }
}

#[derive(Debug)]
pub struct IntersectionInfo {
    location: geo::Coordinate<i64>,
    steps: usize,
}

pub trait IntersectionList {
    fn intersections(self: &Self, other: &Self) -> Vec<IntersectionInfo>;
}

// TODO: clean this up.
impl IntersectionList for geo::LineString<f64> {
    fn intersections(self: &Self, other: &Self) -> Vec<IntersectionInfo> {
        let mut intersections = Vec::new();
        if self.0.is_empty() || other.0.is_empty() {
            return intersections;
        }

        let mut a_len: usize = 0;
        for a in self.lines() {
            let mut b_len: usize = 0;
            for b in other.lines() {
                let u_b = b.dy() * a.dx() - b.dx() * a.dy();
                if u_b == 0.0 {
                    // -1 to not double count the ends of segments
                    b_len += b.length();
                    continue;
                }

                let ua_t = b.dx() * (a.start.y - b.start.y) - b.dy() * (a.start.x - b.start.x);
                let ub_t = a.dx() * (a.start.y - b.start.y) - a.dy() * (a.start.x - b.start.x);
                let u_a = ua_t / u_b;
                let u_b = ub_t / u_b;
                if (0.0 <= u_a) && (u_a <= 1.0) && (0.0 <= u_b) && (u_b <= 1.0) {
                    let coord = geo::Coordinate {
                        x: (a.start.x + u_a * (a.end.x - a.start.x)) as i64,
                        y: (a.start.y + u_a * (a.end.y - a.start.y)) as i64,
                    };

                    // TODO: refactor into trait. Note the -2 offset, which
                    // ensures that we don't include the beginning of the line (similar to others)
                    // as well as not counting the intersection point itself.
                    let a_line_frag_len = std::cmp::max(
                        0,
                        (coord.x - a.start.x as i64).abs() + (coord.y - a.start.y as i64).abs(), /*- 2*/
                    ) as usize;
                    let b_line_frag_len = std::cmp::max(
                        0,
                        (coord.x - b.start.x as i64).abs() + (coord.y - b.start.y as i64).abs(), /*- 2*/
                    ) as usize;

                    // println!(
                    //     "x: {}, y: {}, stepsA: {}, stepsB: {}",
                    //     coord.x,
                    //     coord.y,
                    //     a_len + a_line_frag_len,
                    //     b_len + b_line_frag_len
                    // );
                    if coord.x_y() != (0, 0) {
                        intersections.push(IntersectionInfo {
                            location: coord,
                            steps: a_len + b_len + a_line_frag_len + b_line_frag_len,
                        });
                    }
                }
                // -1 to not double count the ends of segments
                b_len += b.length();
            }
            // -1 to not double count the ends of segments
            a_len += a.length();
        }
        intersections
    }
}

pub trait Length {
    fn length(self: &Self) -> usize;
}

impl Length for geo::Line<f64> {
    fn length(self: &Self) -> usize {
        ((self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs()) as usize
    }
}

// TODO: refactor file ops into separate mod?
fn load_all_instructions() -> Result<WireSegments, &'static str> {
    let lines = fs::read_to_string(INPUT_FILENAME).expect("invalid file");
    WireSegments::from_str(&lines)
}

use geo::algorithm::intersects::Intersects;
pub fn solve() -> String {
    let instructions = load_all_instructions().expect("invalid instructions");

    let intersects: bool = instructions
        .first_wire
        .intersects(&instructions.second_wire);
    let intersections = instructions
        .first_wire
        .intersections(&instructions.second_wire);

    format!(
        "intersects: {}, min distance: {}, min steps: {}",
        intersects,
        intersections
            .iter()
            .map(|i| (i.location.x.abs() + i.location.y.abs()) as i64)
            .min()
            .expect("failed to find intersection"),
        intersections
            .iter()
            .map(|i| i.steps)
            .min()
            .expect("failed to find intersection")
    )
}
