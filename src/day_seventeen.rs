use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, ProgramState, Runnable};
use queues::IsQueue;

const INPUT_FILENAME: &str = "input/day_seventeen.txt";

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    program.run();
}

pub fn part_two() -> i64 {
    1
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(INPUT_FILENAME), part_two())
}
