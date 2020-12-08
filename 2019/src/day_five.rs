#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::Runnable;
use queues::IsQueue;

const INPUT_FILENAME: &'static str = "input/day_five.txt";

pub fn part_one(program: &mut Program) -> i64 {
    program.io.add(1).ok();
    program.run_until_halted();

    // All except the last output should be diagnostic code 0.
    while program.io.size() > 1 {
        assert_eq!(0, program.io.remove().unwrap());
    }
    program.io.peek().unwrap()
}

pub fn part_two(program: &mut Program) -> i64 {
    program.io.add(5).ok();
    program.run_until_halted();
    // All except the last output should be diagnostic code 0.
    while program.io.size() > 1 {
        assert_eq!(0, program.io.remove().unwrap());
    }
    program.io.peek().unwrap()
}

pub fn solve() -> String {
    let mut program = Program::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        part_one(&mut program.clone()),
        part_two(&mut program)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(13978427, part_one(&mut Program::load(INPUT_FILENAME)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(11189491, part_two(&mut Program::load(INPUT_FILENAME)));
    }
}
