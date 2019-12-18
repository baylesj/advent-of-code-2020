#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::ProgramState;
use intcode_computer::Runnable;

const INPUT_FILENAME: &str = "input/day_fifteen.txt";

enum MovementCommand {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

enum RepairDroidStatus {
    HitAWall = 0,
    MovedSuccessfully = 1,
    FoundOxygenSystem = 2,
}

pub fn part_one(program: &mut Program) -> i64 {
    1
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
mod tests {}
