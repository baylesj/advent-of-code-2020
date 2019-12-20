#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, Runnable};
use queues::IsQueue;

const INPUT_FILENAME: &str = "input/day_seventeen.txt";

fn get_grid(input_filename: &str) -> Vec<char> {
    let mut program = Program::load(input_filename);
    program.run_until_halted();

    let mut chars = Vec::new();
    for _ in 0..program.io.size() {
        let i: u8 = program.io.remove().expect("io size wrong") as u8;
        if i > 0 && i as char != '0' {
            chars.push(i as char);
        }
    }
    chars
}

fn print_grid(grid: &Vec<char>) {
    let s: String = grid.iter().cloned().collect();
    println!("{}", s);
}

pub fn part_one(input_filename: &str) -> i64 {
    let grid = get_grid(input_filename);
    print_grid(&grid);
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
