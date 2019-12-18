use num_enum::{IntoPrimitive, TryFromPrimitive};
use queues::IsQueue;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::i64;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::Runnable;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::{Advance, Direction, Point2D, RelativeTurn};

const INPUT_FILENAME: &str = "input/day_fifteen.txt";

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i64)]
enum MovementCommand {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Debug, IntoPrimitive, TryFromPrimitive, PartialEq, Eq)]
#[repr(i64)]
enum RepairDroidStatus {
    Ready = -1,
    HitAWall = 0,
    MovedSuccessfully = 1,
    FoundOxygenSystem = 2,
}

impl Default for RepairDroidStatus {
    fn default() -> Self {
        RepairDroidStatus::Ready
    }
}

#[derive(Debug, Default)]
struct MazeState {
    steps: i64,
    location: Point2D,
    visited: HashMap<Point2D, i64>,
    direction: Direction,
    droid_status: RepairDroidStatus,
}

fn command_from_direction(direction: &Direction) -> MovementCommand {
    match direction {
        Direction::Up => MovementCommand::North,
        Direction::Right => MovementCommand::East,
        Direction::Down => MovementCommand::South,
        Direction::Left => MovementCommand::West,
    }
}

fn attempt_movement(program: &mut Program, state: &mut MazeState) {
    program
        .io
        .add(command_from_direction(&state.direction).into())
        .expect("should add");
    program.run();
    state.droid_status =
        RepairDroidStatus::try_from(program.io.remove().expect("output")).expect("status");
}

// TODO: cleanup
const DIRECTIONS_IN_PRIORITY_ORDER: [Direction; 4] = [
    Direction::Right,
    Direction::Down,
    Direction::Left,
    Direction::Up,
];

fn mark_dead_end(state: &mut MazeState, location: Point2D) {
    state.visited.insert(location, i64::MAX);
}

fn min_index(array: &[i64]) -> usize {
    let mut i = 0;

    for (j, &value) in array.iter().enumerate() {
        if value < array[i] {
            i = j;
        }
    }

    i
}

fn pick_new_direction(state: &mut MazeState) {
    let mut visit_values = Vec::new();
    for direction in &DIRECTIONS_IN_PRIORITY_ORDER {
        let mut speculative_location = state.location.clone();
        speculative_location.advance(*direction);
        if !state.visited.contains_key(&speculative_location) {
            state.direction = *direction;
            return;
        }
        visit_values.push(state.visited[&speculative_location]);
    }

    // if we have already visited all of our neighbors, this is a dead-end.
    mark_dead_end(state, state.location.clone());
    // Go back to last valid.
    state.direction = DIRECTIONS_IN_PRIORITY_ORDER[min_index(&visit_values)];
}

pub fn part_one(program: &mut Program) -> i64 {
    // Initial assumption: only one route (dumb probably).

    let mut state = MazeState::default();
    while state.droid_status != RepairDroidStatus::FoundOxygenSystem {
        match state.droid_status {
            RepairDroidStatus::HitAWall => {
                let mut c = state.location.clone();
                c.advance(state.direction);
                mark_dead_end(&mut state, c);
                pick_new_direction(&mut state);
            }
            RepairDroidStatus::MovedSuccessfully => {
                state.location.advance(state.direction);
                state.steps += 1;
                // We always prefer to go right when possible.
                if state.visited.contains_key(&state.location) {
                    state.steps = i64::min(state.steps, state.visited[&state.location]);
                }
                state.visited.insert(state.location, state.steps);
                pick_new_direction(&mut state);
            }
            _ => (),
        }
        attempt_movement(program, &mut state);
    }
    // Don't forget the final step!
    state.steps + 1
}

pub fn part_two(program: &mut Program) -> i64 {
    1
}

pub fn solve() {
    let mut program = Program::load(INPUT_FILENAME);
    println!(
        "Day fifteen, part one: {}, part two: {}",
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
        assert_eq!(236, part_one(&mut program));
    }
}
