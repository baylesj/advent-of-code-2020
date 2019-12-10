use std::fs;

const INPUT_FILENAME: &str = "input/day_two.txt";

fn operation_code_one(index: usize, program: &mut Vec<i32>) {
    let a_i: usize = program[index + 1] as usize;
    let b_i: usize = program[index + 2] as usize;
    let r_i: usize = program[index + 3] as usize;

    program[r_i] = program[a_i] + program[b_i];
}

// TODO: create generic and pass operator?
fn operation_code_two(index: usize, program: &mut Vec<i32>) {
    let a_i: usize = program[index + 1] as usize;
    let b_i: usize = program[index + 2] as usize;
    let r_i: usize = program[index + 3] as usize;

    program[r_i] = program[a_i] * program[b_i];
}

fn perform_operation(index: usize, program: &mut Vec<i32>) -> bool {
    match program[index] {
        1 => {
            operation_code_one(index, program);
            true
        }
        2 => {
            operation_code_two(index, program);
            true
        }
        // 99 is HALT.
        99 => false,
        _ => panic!("unknown op code received!"),
    }
}

fn fixup_program(program: &mut Vec<i32>) {
    program[1] = 12;
    program[2] = 2;
}

fn set_program_inputs(program: &mut Vec<i32>, noun: i32, verb: i32) {
    program[1] = noun;
    program[2] = verb;
}

fn load_program() -> Vec<i32> {
    let fc: String = fs::read_to_string(INPUT_FILENAME).expect("invalid filename");
    fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn run_program(program: &mut Vec<i32>) {
    let mut index: usize = 0;
    while perform_operation(index, program) {
        index += 4;
    }
}

fn part_one() -> i32 {
    let mut program = load_program();
    fixup_program(&mut program);
    run_program(&mut program);

    program[0]
}

fn part_two() -> i32 {
    const DESIRED_OUTPUT: i32 = 19690720;
    let original_program = load_program();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program: Vec<i32> = original_program.to_vec();
            set_program_inputs(&mut program, noun, verb);
            run_program(&mut program);
            if program[0] == DESIRED_OUTPUT {
                return 100 * program[1] + program[2];
            }
        }
    }

    -1
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(), part_two())
}
