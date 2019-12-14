use queues::IsQueue;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::Runnable;

const INPUT_FILENAME: &str = "input/day_nine.txt";

pub fn part_one(input_filename: &str) -> i128 {
    let mut program = Program::load(input_filename);
    program.io.add(1).ok();
    program.run_until_halted();
    while program.io.peek().unwrap() == 0 {
        program.io.remove().ok();
    }
    program.io.remove().unwrap()
}

pub fn solve() {
    println!("Day nine, part one: {:#?} ", part_one(INPUT_FILENAME));
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
        assert!(i128::pow(10, 15) < output && output < i128::pow(10, 16));
    }

    #[test]
    fn part_one_sample_three() {
        let mut program = Program::load("input/day_nine_sample_three.txt");
        program.run_until_halted();
        assert_eq!(1125899906842624, program.io.peek().unwrap());
    }
}
