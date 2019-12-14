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
        // let expected_program = Program::load("input/day_nine_sample_one.txt");
        // println!("expected: {:#?}", expected_program.buffer);
        // let mut actual_program = part_one("input/day_nine_sample_one.txt");
        // for i in 0..expected_program.buffer.len() {
        //     assert_eq!(expected_program.buffer[i], actual_program.remove().unwrap());
        // }
    }

    #[test]
    fn part_one_sample_two() {
        // a sixteen digit number
        // assert_eq!(
        //     139629729,
        //     part_one("input/day_nine_sample_two.txt")
        // );
    }

    #[test]
    fn part_one_sample_three() {
        assert_eq!(
            1125899906842624,
            part_one("input/day_nine_sample_three.txt")
        );
    }
}
