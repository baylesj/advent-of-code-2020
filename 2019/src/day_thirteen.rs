use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;
use queues::Queue;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::Point2D;

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

#[derive(Debug, Copy, Clone)]
struct Tile {
    location: Point2D,
    id: TileId,
    raw_id: i64,
}

fn pop_tile(queue: &mut Queue<i64>) -> Result<Tile, &str> {
    if queue.size() < 3 {
        return Err("non-full queue");
    }
    let x = queue.remove().unwrap();
    let y = queue.remove().unwrap();
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
    ball_x: i64,
    paddle_x: i64,
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

    if program.io.size() > 2 {
        pop_tile(&mut program.io)
    } else {
        Err("probably waiting for input")
    }
}

trait GameActions {
    fn run(&mut self);
    fn draw(&self);
}

impl GameActions for GameState {
    fn run(&mut self) {
        while self.program.state != ProgramState::Stopped {
            // Handle output
            while self.program.io.size() > 2 {
                let tile = get_tile(&mut self.program).expect("tile");
                if tile.id == TileId::Ball {
                    self.ball_x = tile.location.x;
                } else if tile.id == TileId::HorizontalPaddle {
                    self.paddle_x = tile.location.x;
                } else if tile.id == TileId::Score {
                    self.score = tile.raw_id;
                    break;
                }
                self.tiles.push(tile);
            }

            // Handle input
            if self.program.state == ProgramState::PausedWaitingForInput {
                assert!(self.program.io.size() == 0);
                let input: i64;
                if self.paddle_x < self.ball_x {
                    input = JoystickPosition::Right.into();
                } else if self.paddle_x == self.ball_x {
                    input = JoystickPosition::Neutral.into();
                } else {
                    input = JoystickPosition::Left.into();
                }

                // TODO: split input and output queues;
                self.program.io.add(input).expect("should be able to add");
                self.program.run();
            }

            self.program.run();
        }
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
    state.run();
    state.score
}

pub fn solve() -> String {
    let mut program = Program::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        part_one(&mut program),
        part_two(INPUT_FILENAME)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut program = Program::load(INPUT_FILENAME);
        assert_eq!(320, part_one(&mut program));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(15156, part_two(INPUT_FILENAME));
    }
}
