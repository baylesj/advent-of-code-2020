use queues::IsQueue;

pub use self::intcode_computer::{LoadableFromFile, Program, Runnable};
#[path = "intcode_computer.rs"]
mod intcode_computer;

const INPUT_FILENAME: &'static str = "input/day_nine.txt";

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    program.io.add(1).ok();
    program.run_until_halted();
    while program.io.peek().unwrap() == 0 {
        program.io.remove().ok();
    }
    program.io.remove().unwrap()
}

pub fn part_two(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    program.io.add(2).ok();
    program.run_until_halted();
    while program.io.peek().unwrap() == 0 {
        program.io.remove().ok();
    }
    program.io.remove().unwrap()
}

pub fn solve() -> String {
    format!(
        "part one: {:#?}, part two {:#?}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
        let mut program = Program::load("input/day_nine_sample_one.txt");
        let expected_buffer = program.buffer.clone();
        program.run_until_halted();
        assert_eq!(expected_buffer[0], program.io.remove().unwrap());
    }

    #[test]
    fn part_one_sample_two() {
        let mut program = Program::load("input/day_nine_sample_two.txt");
        program.run_until_halted();
        let output = program.io.remove().unwrap();
        assert!(i64::pow(10, 15) < output && output < i64::pow(10, 16));
    }

    #[test]
    fn part_one_sample_three() {
        let mut program = Program::load("input/day_nine_sample_three.txt");
        program.run_until_halted();
        assert_eq!(1125899906842624, program.io.peek().unwrap());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(2406950601, part_one(INPUT_FILENAME));
    }
}
