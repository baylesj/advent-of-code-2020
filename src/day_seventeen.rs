#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;

const INPUT_FILENAME: &str = "input/day_seventeen.txt";

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    program.run();

    let mut chars = vec!['0'; program.io.size() + 10];
    for _ in 0..program.io.size() {
        chars.push(program.io.remove().expect("io lies!") as u8 as char);
    }
    let s: String = chars.iter().collect();
    println!("{}", s);
    println!("state: {:?}", program.state);
    1
}

pub fn part_two() -> i64 {
    1
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two()
    )
}
