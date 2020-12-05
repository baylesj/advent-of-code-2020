use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::{Advance, Direction, Point2D, RelativeTurn};

const INPUT_FILENAME: &'static str = "input/day_eleven.txt";

#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum TileColor {
    Black = 0,
    White = 1,
}

#[derive(Default, Debug)]
struct PainterState {
    white_tiles: HashSet<Point2D>,
    // Important note from prompt: painting a tile black
    // does count as painting, not "erasing."
    black_tiles: HashSet<Point2D>,
    current_location: Point2D,
    current_direction: Direction,

    min_point: Point2D,
    max_point: Point2D,
}

trait Update {
    fn update(self: &mut Self);
}

impl Update for PainterState {
    fn update(self: &mut Self) {
        self.current_location.advance(self.current_direction);
        self.min_point.x = i64::min(self.min_point.x, self.current_location.x);
        self.min_point.y = i64::min(self.min_point.y, self.current_location.y);
        self.max_point.x = i64::max(self.max_point.x, self.current_location.x);
        self.max_point.y = i64::max(self.max_point.y, self.current_location.y);
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

fn print(state: &PainterState) -> String {
    let mut output: String = String::default();
    let mut line: Vec<char> =
        Vec::with_capacity((state.max_point.x + 1 - state.min_point.x) as usize);
    let mut point = Point2D::default();
    for y in state.min_point.y..state.max_point.y + 1 {
        point.y = y;
        for x in state.min_point.x..state.max_point.x + 1 {
            point.x = x;
            line.push(if state.white_tiles.contains(&point) {
                '⬜'
            } else {
                '⬛'
            });
        }
        output += &format!("    {}\n", String::from_iter(&line));
        line.truncate(0);
    }
    output
}

fn paint(program: &mut Program, initial_color: TileColor) -> PainterState {
    let mut state = PainterState::default();
    // All tiles start out black
    program.io.add(initial_color.into()).ok();
    program.run();
    program.run();
    while program.state != ProgramState::Stopped {
        let tile_color = TileColor::try_from(program.io.remove().expect("missing output"));
        let should_turn_right: bool = program.io.remove().expect("missing output") == 1;
        match tile_color.expect("invalid tile color") {
            TileColor::Black => {
                state.white_tiles.remove(&state.current_location);
                state.black_tiles.insert(state.current_location);
            }
            TileColor::White => {
                state.white_tiles.insert(state.current_location);
                state.black_tiles.remove(&state.current_location);
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
    state
}

pub fn part_one(program: &mut Program) -> i64 {
    let state = paint(program, TileColor::Black);
    (state.white_tiles.len() + state.black_tiles.len()) as i64
}

pub fn part_two(program: &mut Program) -> String {
    let state = paint(program, TileColor::White);
    print(&state)
}

pub fn solve() -> String {
    let mut program = Program::load(INPUT_FILENAME);
    format!(
        "part one: {} white tiles, part two:\n{}",
        part_one(&mut program.clone()),
        part_two(&mut program)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance() {
        let mut actual = Point2D::default();
        actual.advance(Direction::Right);
        assert_eq!(Point2D { x: 1, y: 0 }, actual);
        actual.advance(Direction::Up);
        assert_eq!(Point2D { x: 1, y: -1 }, actual);
        actual.advance(Direction::Left);
        assert_eq!(Point2D { x: 0, y: -1 }, actual);
        actual.advance(Direction::Down);
        assert_eq!(Point2D { x: 0, y: 0 }, actual);
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
        assert_eq!(2322, part_one(&mut program));
    }
}
