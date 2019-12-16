use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::collections::HashSet;
use std::convert::TryFrom;
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
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    x: i64,
    y: i64,
    id: TileId,
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
    let tid = TileId::try_from(queue.remove().unwrap()).unwrap();
    Ok(Tile {
        x: x,
        y: y,
        id: tid,
    })
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

pub fn part_two(program: &mut Program) -> i64 {
    // Set the number of quartes in the arcade cabinet.
    program.buffer[0] = 2;
    123
}

pub fn solve() {
    let mut program = Program::load(INPUT_FILENAME);
    println!(
        "Day thirteen, part one: {}, part two: {}",
        part_one(&mut program.clone()),
        part_two(&mut program)
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
