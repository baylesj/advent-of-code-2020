use permutator::Permutation;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::ProgramState;
use intcode_computer::Runnable;

const INPUT_FILENAME: &str = "input/day_seven.txt";
const PROGRAM_COUNT: usize = 5;

fn run_permutation(input: i32, permutation: &[i32], programs: &mut Vec<Program>) -> i32 {
    let mut output: i32 = input;
    for i in 0..PROGRAM_COUNT {
        programs[i].io = vec![output, permutation[i]];
        programs[i].run();
        output = programs[i].io.pop().expect("io should not be empty");
    }
    output
}

fn create_program_vector(reference: &Program) -> Vec<Program> {
    let mut programs = Vec::new();
    for _ in 0..PROGRAM_COUNT {
        programs.push(reference.clone());
    }
    programs
}

pub fn part_one(input_filename: &str) -> i32 {
    const PHASES: [i32; PROGRAM_COUNT] = [0, 1, 2, 3, 4];
    let program = Program::load(input_filename);
    let mut max_signal: i32 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs = create_program_vector(&program);
        max_signal = std::cmp::max(max_signal, run_permutation(0, &p, &mut programs));
    });
    max_signal
}

pub fn part_two(input_filename: &str) -> i32 {
    const PHASES: [i32; PROGRAM_COUNT] = [5, 6, 7, 8, 9];
    let program = Program::load(input_filename);
    let mut max_signal: i32 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs = create_program_vector(&program);
        let mut signal: i32 = 0;
        while programs.last().expect("last").state != ProgramState::Stopped {
            signal = run_permutation(signal, &p, &mut programs);
        }
        max_signal = std::cmp::max(max_signal, signal);
    });
    max_signal
}

pub fn solve() {
    println!(
        "Day seven, part 1: {}, part 2: {}",
        /*part_one(INPUT_FILENAME)*/ 0,
        part_two(INPUT_FILENAME)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
        assert_eq!(43210, part_one("input/day_seven_part_one_sample_one.txt"));
    }

    #[test]
    fn part_two_sample_one() {
        assert_eq!(
            139629729,
            part_two("input/day_seven_part_two_sample_one.txt")
        );
    }

    #[test]
    fn part_two_sample_two() {
        assert_eq!(18216, part_two("input/day_seven_part_two_sample_two.txt"));
    }
}
