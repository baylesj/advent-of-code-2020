use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;

const INPUT_FILENAME: &'static str = "input/day_eleven.txt";

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum TileColor {
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

trait RelativeTurn {
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

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

trait Advance {
    fn advance(self: &mut Self, direction: Direction);
}

impl Advance for Point {
    fn advance(self: &mut Self, direction: Direction) {
        match direction {
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }
    }
}

fn print(min_point: &Point, max_point: &Point, tiles: &HashSet<Point>) {
    let mut line: Vec<char> = Vec::with_capacity((max_point.x + 1 - min_point.x) as usize);
    let mut point = Point::default();
    for x in min_point.x..max_point.x + 1 {
        point.x = x;
        for y in min_point.y..max_point.y + 1 {
            point.y = y;
            line.push(if tiles.contains(&point) { '⬜' } else { '⬛' });
        }
        println!("{}", String::from_iter(&line));
        line.truncate(0);
    }
}

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    // All tiles start out black
    let mut white_tiles = HashSet::new();
    let mut current_location = Point::default();
    let mut current_direction = Direction::Up;

    let mut min_point = Point::default();
    let mut max_point = Point::default();
    program.io.add(TileColor::Black.into()).ok();
    program.run();
    program.run();
    while program.state != ProgramState::Stopped {
        let should_turn_right: bool = program.io.remove().expect("missing output") == 1;
        let tile_color = TileColor::try_from(program.io.remove().expect("missing output"));
        match tile_color.expect("invalid tile color") {
            TileColor::Black => {
                white_tiles.remove(&current_location);
            }
            TileColor::White => {
                white_tiles.insert(current_location);
            }
        }

        match should_turn_right {
            true => current_direction = current_direction.to_right(),
            false => current_direction = current_direction.to_left(),
        }

        current_location.advance(current_direction);
        if white_tiles.contains(&current_location) {
            program.io.add(TileColor::White.into()).ok();
        } else {
            program.io.add(TileColor::Black.into()).ok();
        }

        min_point.x = i32::min(min_point.x, current_location.x);
        min_point.y = i32::min(min_point.x, current_location.y);
        max_point.x = i32::max(max_point.x, current_location.x);
        max_point.y = i32::max(max_point.x, current_location.y);
        program.run();
        program.run();
    }

    print(&min_point, &max_point, &white_tiles);
    white_tiles.len() as i64
}

pub fn solve() {
    println!("Day eleven, part one: {}", part_one(INPUT_FILENAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let mut actual = Point::default();
        actual.advance(Direction::Right);
        assert_eq!(Point { x: 1, y: 0 }, actual);
        actual.advance(Direction::Up);
        assert_eq!(Point { x: 1, y: -1 }, actual);
        actual.advance(Direction::Left);
        assert_eq!(Point { x: 0, y: -1 }, actual);
        actual.advance(Direction::Down);
        assert_eq!(Point { x: 0, y: 0 }, actual);
    }

    #[test]
    fn test_relative_turn() {
        assert_eq!(Direction::Down, Direction::Left.to_left());
        assert_eq!(Direction::Right, Direction::Down.to_left());
        assert_eq!(Direction::Up, Direction::Right.to_left());
        assert_eq!(Direction::Left, Direction::Up.to_left());
        assert_eq!(Direction::Up, Direction::Left.to_right());
        assert_eq!(Direction::Left, Direction::Down.to_right());
        assert_eq!(Direction::Down, Direction::Right.to_right());
        assert_eq!(Direction::Right, Direction::Up.to_right());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(732, part_one(INPUT_FILENAME));
    }
}
