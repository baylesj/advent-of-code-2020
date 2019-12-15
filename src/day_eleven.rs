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

impl Default for Direction {
    fn default() -> Self {
        return Direction::Up;
    }
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

#[derive(Default, Debug)]
struct PainterState {
    white_tiles: HashSet<Point>,
    current_location: Point,
    current_direction: Direction,

    min_point: Point,
    max_point: Point,
}

trait Update {
    fn update(self: &mut Self);
}

impl Update for PainterState {
    fn update(self: &mut Self) {
        self.current_location.advance(self.current_direction);
        self.min_point.x = i32::min(self.min_point.x, self.current_location.x);
        self.min_point.y = i32::min(self.min_point.x, self.current_location.y);
        self.max_point.x = i32::max(self.max_point.x, self.current_location.x);
        self.max_point.y = i32::max(self.max_point.x, self.current_location.y);
    }
}

trait CurrentColor {
    fn current_color(self: &Self) -> TileColor;
}

impl CurrentColor for PainterState {
    fn current_color(self: &Self) -> TileColor {
        if self.white_tiles.contains(&self.current_location) {
            TileColor::White
        } else {
            TileColor::Black
        }
    }
}

fn print(state: &PainterState) {
    let mut line: Vec<char> =
        Vec::with_capacity((state.max_point.x + 1 - state.min_point.x) as usize);
    let mut point = Point::default();
    for x in state.min_point.x..state.max_point.x + 1 {
        point.x = x;
        for y in state.min_point.y..state.max_point.y + 1 {
            point.y = y;
            line.push(if state.white_tiles.contains(&point) {
                '⬜'
            } else {
                '⬛'
            });
        }
        println!("{}", String::from_iter(&line));
        line.truncate(0);
    }
}

fn paint(program: &mut Program, initial_color: TileColor) -> i64 {
    let mut state = PainterState::default();
    // All tiles start out black
    program.io.add(initial_color.into()).ok();
    program.run();
    program.run();
    while program.state != ProgramState::Stopped {
        let should_turn_right: bool = program.io.remove().expect("missing output") == 1;
        let tile_color = TileColor::try_from(program.io.remove().expect("missing output"));
        match tile_color.expect("invalid tile color") {
            TileColor::Black => {
                state.white_tiles.remove(&state.current_location);
            }
            TileColor::White => {
                state.white_tiles.insert(state.current_location);
            }
        }

        match should_turn_right {
            true => state.current_direction = state.current_direction.to_right(),
            false => state.current_direction = state.current_direction.to_left(),
        }

        state.update();
        program.io.add(state.current_color().into()).ok();
        program.run();
        program.run();
    }

    print(&state);
    state.white_tiles.len() as i64
}

pub fn part_one(program: &mut Program) -> i64 {
    paint(program, TileColor::Black)
}

pub fn part_two(program: &mut Program) -> i64 {
    paint(program, TileColor::White)
}

pub fn solve() {
    // TODO: fix cloning
    let mut program = Program::load(INPUT_FILENAME);
    println!("Day eleven, part one:");
    println!("total of {} white tiles", part_one(&mut program.clone()));
    println!("part two:");
    println!("total of {} white tiles", part_two(&mut program));
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
        let mut program = Program::load(INPUT_FILENAME);
        assert_eq!(732, part_one(&mut program));
    }
}
