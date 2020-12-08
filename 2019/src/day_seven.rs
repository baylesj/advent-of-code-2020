use permutator::Permutation;
use queues::IsQueue;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::ProgramState;
use intcode_computer::Runnable;

const INPUT_FILENAME: &'static str = "input/day_seven.txt";
const PROGRAM_COUNT: usize = 5;

fn run_permutation(input: i64, permutation: &[i64], programs: &mut Vec<Program>) -> i64 {
    let mut output: i64 = input;
    for i in 0..PROGRAM_COUNT {
        programs[i].io.add(permutation[i]).ok();
        programs[i].io.add(output).ok();
        programs[i].run();
        output = programs[i].io.remove().expect("io should not be empty");
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

// TODO: factor out common permutate function.
pub fn part_one(input_filename: &str) -> i64 {
    const PHASES: [i64; PROGRAM_COUNT] = [0, 1, 2, 3, 4];
    let program = Program::load(input_filename);
    let mut max_signal: i64 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs = create_program_vector(&program);
        max_signal = std::cmp::max(max_signal, run_permutation(0, &p, &mut programs));
    });
    max_signal
}

fn run_program_chain(programs: &mut Vec<Program>) {
    programs[0].run();
    for i in 1..PROGRAM_COUNT {
        pipe_io(programs, i - 1, i);
        programs[i].run();
    }
}

fn pipe_io(programs: &mut Vec<Program>, from: usize, to: usize) {
    while programs[from].io.size() > 0 {
        let input_value = programs[from].io.remove().expect("checked");
        programs[to].io.add(input_value).ok();
    }
}

pub fn part_two(input_filename: &str) -> i64 {
    const PHASES: [i64; PROGRAM_COUNT] = [5, 6, 7, 8, 9];
    let program = Program::load(input_filename);
    let mut max_signal: i64 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs = create_program_vector(&program);
        for i in 0..PROGRAM_COUNT {
            programs[i].io.add(p[i]).ok();
        }
        programs[0].io.add(0).ok();

        while programs.last().expect("last").state != ProgramState::Stopped {
            run_program_chain(&mut programs);
            pipe_io(&mut programs, PROGRAM_COUNT - 1, 0);
        }

        max_signal = std::cmp::max(
            max_signal,
            // Output has been pushed ot program one, even though it's the
            // last output of the last program phase.
            programs[0].io.remove().expect("has top"),
        );
    });
    max_signal
}

pub fn solve() -> String {
    format!(
        "part 1: {}, part 2: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample_one() {
        assert_eq!(43210, part_one("input/day_seven_part_one_sample_one.txt"));
    }

    #[test]
    fn test_part_two_sample_one() {
        assert_eq!(
            139629729,
            part_two("input/day_seven_part_two_sample_one.txt")
        );
    }

    #[test]
    fn test_part_two_sample_two() {
        assert_eq!(18216, part_two("input/day_seven_part_two_sample_two.txt"));
    }
}
