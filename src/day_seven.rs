use permutator::Permutation;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::ProgramState;
use intcode_computer::Runnable;

const INPUT_FILENAME: &str = "input/day_seven.txt";
const PROGRAM_COUNT: usize = 5;

fn run_permutation(permutation: &[i32], programs: &mut Vec<Program>) -> i32 {
    let mut output: i32 = 0;
    for i in 0..PROGRAM_COUNT {
        programs[i].io = vec![output, permutation[i]];
        programs[i].run();
        output = programs[i].io[0];
    }
    programs[PROGRAM_COUNT - 1]
        .io
        .pop()
        .expect("io should not be empty")
}

pub fn part_one(input_filename: &str) -> i32 {
    const PHASES: [i32; PROGRAM_COUNT] = [0, 1, 2, 3, 4];
    let program = Program::load(input_filename);
    let mut max_signal: i32 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs = Vec::new();
        for _ in 0..PROGRAM_COUNT {
            programs.push(program.clone());
        }
        max_signal = std::cmp::max(max_signal, run_permutation(&p, &mut programs));
    });
    // TODO: write basic unit test coverage of intcode computer.
    assert_eq!(422858, max_signal);
    max_signal
}

pub fn part_two(input_filename: &str) -> i32 {
    const PHASES: [i32; PROGRAM_COUNT] = [5, 6, 7, 8, 9];
    let program = Program::load(input_filename);
    let mut max_signal: i32 = 0;

    PHASES.to_vec().permutation().for_each(|p| {
        let mut programs: Vec<Program> = Vec::new();
        for _ in 0..PROGRAM_COUNT {
            programs.push(program.clone());
        }

        let mut signal: i32 = 0;
        while programs.last().expect("last").state != ProgramState::Stopped {
            signal = run_permutation(&p, &mut programs);
            println!(
                "program last state: {:#?}, output signal: {}",
                programs.last().expect("").state,
                signal
            );
        }
        println!(
            "program last state: {:#?}, output signal: {}",
            programs.last().expect("").state,
            signal
        );
        max_signal = std::cmp::max(max_signal, signal);
        println!("max signal = {} for p = {:#?}", max_signal, p);
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
  fn part_two_sample_one() {
      assert_eq!(139629729, part_two("input/day_seven_part_two_sample_one.txt"));
  }

  #[test]
  fn part_two_sample_two() {
      assert_eq!(18216, part_two("input/day_seven_part_two_sample_two.txt"));
  }
}