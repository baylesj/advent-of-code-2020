use permutator::Permutation;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
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

pub fn part_one() -> i32 {
    const PHASES: [i32; PROGRAM_COUNT] = [0, 1, 2, 3, 4];
    let program = Program::load(INPUT_FILENAME);
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

pub fn part_two() -> i32 {
  const PHASES: [i32; PROGRAM_COUNT] = [0, 1, 2, 3, 4];
  let program = Program::load(INPUT_FILENAME);
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

pub fn solve() {
    println!("Day seven, part 1: {}, part 2: {}", part_one(), part_two());
}
