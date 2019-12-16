use matrix::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;
use queues::Queue;

const INPUT_FILENAME: &'static str = "input/day_thirteen.txt";

#[derive(Debug, PartialEq, Eq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum TileId {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
    Score = 5,
}

// TODO: combine with Point3D and put in Yet another geometry package?
#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    location: Point2D,
    id: TileId,
    raw_id: i64,
}

fn pop_tile(queue: &mut Queue<i64>) -> Result<Tile, &str> {
    if queue.peek().is_err() {
        return Err("empty queue");
    }
    let x = queue.remove().unwrap();
    if queue.peek().is_err() {
        return Err("empty queue");
    }
    let y = queue.remove().unwrap();
    if queue.peek().is_err() {
        return Err("empty queue");
    }
    let raw_id = queue.remove().unwrap();
    let tid: TileId;
    if x == -1 && y == 0 {
        tid = TileId::Score;
    } else {
        tid = TileId::try_from(raw_id).unwrap();
    }
    Ok(Tile {
        location: Point2D { x: x, y: y },
        id: tid,
        raw_id: raw_id,
    })
}

#[derive(Debug, Default)]
pub struct GameState {
    tiles: Vec<Tile>,
    score: i64,
    program: Program,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum JoystickPosition {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

fn display_char(tid: &TileId) -> char {
    match tid {
        TileId::Wall => '⬛',
        TileId::Block => '⬜',
        TileId::HorizontalPaddle => '▭',
        TileId::Ball => '⚽',
        _ => ' ',
    }
}

#[derive(Debug, Default)]
struct Buffer {
    buffer: Vec<char>,
    size: Point2D,
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size.y {
            let begin = (row * self.size.x) as usize;
            let end = ((row + 1) * self.size.x - 1) as usize;
            let out: String = self
                .buffer
                .get(begin..end)
                .unwrap()
                .iter()
                .cloned()
                .collect();
            write!(f, "{}\n", out).ok();
        }
        Ok(())
    }
}
trait FrameBuffer {
    fn new(size: &Point2D) -> Self;
    fn set(&mut self, location: &Point2D, value: char);
    fn get(&mut self, location: &Point2D) -> char;
}

impl FrameBuffer for Buffer {
    fn new(size: &Point2D) -> Self {
        // TODO: need plus one?
        Buffer {
            buffer: vec![' '; (size.x * size.y + 1) as usize],
            size: size.clone(),
        }
    }

    fn set(&mut self, location: &Point2D, value: char) {
        self.buffer[(location.y * self.size.x + location.x) as usize] = value;
    }

    fn get(&mut self, location: &Point2D) -> char {
        self.buffer[(location.y * self.size.x + location.x) as usize]
    }
}

fn get_tile(program: &mut Program) -> Result<Tile, &str> {
    if program.state == ProgramState::Stopped {
        return Err("Program is stopped");
    }

    program.run();
    program.run();
    program.run();

    pop_tile(&mut program.io)
}

trait GameActions {
    fn start(&mut self);
    fn update(&mut self, position: JoystickPosition);
    fn draw(&self);
}

impl GameActions for GameState {
    fn start(&mut self) {
        while let Some(tile) = get_tile(&mut self.program).ok() {
            if tile.id == TileId::Score {
                self.score = tile.raw_id;
                break;
            } else {
                self.tiles.push(tile);
            }
        }
    }

    fn update(&mut self, position: JoystickPosition) {
        self.tiles.clear();
        self.program.static_input = Some(position.into());
        self.start();
    }

    fn draw(&self) {
        if self.tiles.is_empty() {
            return;
        }
        let max_x = self.tiles.iter().map(|t| t.location.x).max().unwrap() as i64;
        let max_y = self.tiles.iter().map(|t| t.location.y).max().unwrap() as i64;
        // NOTE: plus one because len = max value + 1
        let mut display = Buffer::new(&Point2D {
            y: max_y + 1,
            x: max_x + 1,
        });

        for tile in &self.tiles {
            display.set(&tile.location, display_char(&tile.id));
        }

        println!("{}\ncurrent score: {}", display, self.score);
    }
}

pub fn part_one(program: &mut Program) -> i64 {
    program.run_until_halted();

    let mut sum = 0;
    while let Some(tile) = pop_tile(&mut program.io).ok() {
        if tile.id == TileId::Block {
            sum += 1;
        }
    }
    sum
}

pub fn part_two(input_filename: &str) -> i64 {
    let mut state = GameState::default();
    state.program = Program::load(input_filename);
    // Set the number of quarters in the arcade cabinet.
    state.program.buffer[0] = 2;
    state.start();
    state.draw();
    for _ in 0..100000 {
        state.update(JoystickPosition::Left);
        state.draw();
    }
    123
}

pub fn solve() {
    let mut program = Program::load(INPUT_FILENAME);
    println!(
        "Day thirteen, part one: {}, part two: {}",
        part_one(&mut program),
        part_two(INPUT_FILENAME)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut program = Program::load(INPUT_FILENAME);
        assert_eq!(320, part_one(&mut program));
    }
}
