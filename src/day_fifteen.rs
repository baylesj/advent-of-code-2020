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
use yet_another_geometry_mod::{Advance, Direction, Inverse, Point2D};

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
    pub steps: i64,
    location: Point2D,
    visited: HashMap<Point2D, i64>,
    dimensions: (Point2D, Point2D),
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

fn attempt_movement(program: &mut Program, direction: &Direction) -> RepairDroidStatus {
    program
        .io
        .add(command_from_direction(&direction).into())
        .expect("should add");
    program.run();
    RepairDroidStatus::try_from(program.io.remove().expect("output")).expect("status")
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

    mark_dead_end(state, state.location.clone());
    // Go back to last valid.
    state.direction = DIRECTIONS_IN_PRIORITY_ORDER[min_index(&visit_values)];
}

fn visualize(tiles: &HashMap<Point2D, i64>, dimensions: &(Point2D, Point2D), location: &Point2D) {
    let mut p = Point2D::default();
    for y in dimensions.0.y..dimensions.1.y {
        p.y = y;
        // TODO: why is the offset weird?
        let mut row: Vec<char> = vec![' '; (dimensions.1.x + dimensions.0.x) as usize + 10];
        for x in dimensions.0.x..dimensions.1.x {
            p.x = x;
            let idx = (x + dimensions.0.x.abs()) as usize;
            if p == *location {
                row[idx] = 'O';
                continue;
            }
            match *tiles.get(&p).unwrap_or(&-1) {
                i64::MAX => row[idx] = '#',
                -1 => row[idx] = '?',
                _ => (),
            }
        }
        let row_as_string: String = row.iter().cloned().collect();
        println!("{}", row_as_string);
    }
    println!("dimensions: {:#?}", dimensions);
    println!("location: {}", location);
}

pub fn part_one(program: &mut Program) -> i64 {
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
                state.dimensions.0.x = i64::min(state.dimensions.0.x, state.location.x);
                state.dimensions.0.y = i64::min(state.dimensions.0.y, state.location.y);
                state.dimensions.1.x = i64::max(state.dimensions.1.x, state.location.x);
                state.dimensions.1.y = i64::max(state.dimensions.1.y, state.location.y);
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
        state.droid_status = attempt_movement(program, &state.direction);
    }
    // Don't forget the final step!
    state.steps += 1;
    visualize(&state.visited, &state.dimensions, &state.location);
    state.steps
}

#[derive(Debug, Default, Clone)]
struct Searcher {
    program: Program,
    location: Point2D,
    origin: Option<Direction>,
}

enum SearchResult {
    Done,
    StillGoing(Vec<Searcher>),
}

fn next_searchers(searcher: &Searcher) -> SearchResult {
    let mut next_layer = Vec::new();
    let mut clone = searcher.clone();
    for direction in DIRECTIONS_IN_PRIORITY_ORDER.iter() {
        if searcher.origin.is_some() && *direction == searcher.origin.unwrap() {
            // Don't backtrack.
            continue;
        }

        let state = attempt_movement(&mut clone.program, direction);
        if state == RepairDroidStatus::HitAWall {
            // The program is unchanged, so don't need to change anything.
            continue;
        }
        clone.location.advance(*direction);
        clone.origin = Some(direction.inverse());
        next_layer.push(clone);
        clone = searcher.clone();
    }

    if next_layer.len() > 0 {
        SearchResult::StillGoing(next_layer)
    } else {
        SearchResult::Done
    }
}

// TODO: come up with algorithm.
pub fn part_two(program: &mut Program) -> i64 {
    let mut current_searchers: Vec<Searcher> = vec![Searcher {
        program: program.clone(),
        location: Point2D::default(),
        origin: None,
    }];

    let mut steps = 0;
    while current_searchers.len() > 0 {
        let mut next_layer: Vec<Searcher> = Vec::new();
        for searcher in &current_searchers {
            let result = next_searchers(&searcher);
            match result {
                SearchResult::StillGoing(x) => next_layer.extend(x.into_iter()),
                SearchResult::Done => (),
            }
        }
        current_searchers = next_layer;
        steps += 1;
    }

    steps
}

pub fn solve() {
    let mut program = Program::load(INPUT_FILENAME);
    println!(
        "Day fifteen, part one: {}, part two: {}",
        part_one(&mut program),
        part_two(&mut program)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut program = Program::load(INPUT_FILENAME);
        assert_eq!(236, part_one(&mut program).steps);
    }
}
