#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::Runnable;

const INPUT_FILENAME: &'static str = "input/day_two.txt";

fn set_program_inputs(program: &mut Program, noun: i64, verb: i64) {
    program.buffer[1] = noun;
    program.buffer[2] = verb;
}

fn fixup_program(program: &mut Program) {
    set_program_inputs(program, 12, 2);
}

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    fixup_program(&mut program);
    program.run_until_halted();
    program.buffer[0]
}

pub fn part_two(input_filename: &str) -> i64 {
    const DESIRED_OUTPUT: i64 = 19690720;
    let original_program = Program::load(input_filename);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = original_program.clone();
            set_program_inputs(&mut program, noun, verb);
            program.run();
            if program.buffer[0] == DESIRED_OUTPUT {
                return 100 * program.buffer[1] + program.buffer[2];
            }
        }
    }

    -1
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(9581917, part_one(INPUT_FILENAME));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(2505, part_two(INPUT_FILENAME));
    }
}
