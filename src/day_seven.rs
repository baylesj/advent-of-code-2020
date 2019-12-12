use permutator::Permutation;
use std::fs;

const INPUT_FILENAME: &str = "input/day_seven.txt";

// TODO: redirect to fakes?
static mut STDIN_CONTENTS: Vec<i32> = Vec::new();
static mut STDOUT_CONTENTS: i32 = 0;

#[derive(PartialEq, Clone, Copy)]
enum ParameterMode {
    Immediate,
    Position,
}

type Program = Vec<i32>;

// NOTE: ONLY for read only params.
fn access_parameter(index: usize, program: &Program, mode: ParameterMode) -> i32 {
    if mode == ParameterMode::Immediate {
        program[index]
    } else {
        program[program[index] as usize]
    }
}

fn operation_add(index: usize, program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a + b;
}

// TODO: create generic and pass operator?
fn operation_multiply(index: usize, program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a * b;
}

fn read() -> i32 {
    // TODO: refactor to remove unsafe.
    unsafe { STDIN_CONTENTS.pop().expect("stack empty") }
}

fn operation_input(index: usize, program: &mut Program) {
    let r_i: usize = program[index + 1] as usize;

    let i: i32 = read();
    println!("\tSYSTEM INPUT REQUESTED: {}", i);
    program[r_i] = i;
}

fn operation_output(index: usize, program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    if a > 0 {
        println!("\tSYSTEM OUTPUT PROVIDED: {}", a);
        unsafe {
            STDOUT_CONTENTS = a;
        }
    }
}

fn operation_jump_if_true(
    index: usize,
    program: &mut Program,
    modes: &Vec<ParameterMode>,
) -> usize {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    if a != 0 {
        return b as usize;
    }
    index + 3
}

fn operation_jump_if_false(
    index: usize,
    program: &mut Program,
    modes: &Vec<ParameterMode>,
) -> usize {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    if a == 0 {
        return b as usize;
    }
    index + 3
}

fn operation_less_than(index: usize, program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    if a < b {
        program[r_i] = 1;
    } else {
        program[r_i] = 0;
    }
}

fn operation_equals(index: usize, program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    if a == b {
        program[r_i] = 1;
    } else {
        program[r_i] = 0;
    }
}

fn dig(op: i32, place: u32) -> i32 {
    let t = i32::pow(10, place);
    ((op / t) % 10) * t
}

fn dig_mode(op: i32, place: u32) -> ParameterMode {
    if dig(op, place) > 0 {
        return ParameterMode::Immediate;
    }
    ParameterMode::Position
}

fn perform_operation(index: usize, program: &mut Program) -> Option<usize> {
    let op = program[index];
    let op_code: i32 = dig(op, 1) + dig(op, 0);
    let modes: Vec<ParameterMode> = vec![dig_mode(op, 2), dig_mode(op, 3), dig_mode(op, 4)];

    // TODO: separate algo from data here.
    match op_code {
        1 => {
            operation_add(index, program, &modes);
            Some(index + 4)
        }
        2 => {
            operation_multiply(index, program, &modes);
            Some(index + 4)
        }
        3 => {
            operation_input(index, program);
            Some(index + 2)
        }
        4 => {
            operation_output(index, program, &modes);
            Some(index + 2)
        }
        5 => Some(operation_jump_if_true(index, program, &modes)),
        6 => Some(operation_jump_if_false(index, program, &modes)),
        7 => {
            operation_less_than(index, program, &modes);
            Some(index + 4)
        }
        8 => {
            operation_equals(index, program, &modes);
            Some(index + 4)
        }
        // 99 is HALT.
        99 => None,
        _ => panic!("unknown OP code: {}", op_code),
    }
}

fn load_program() -> Program {
    let fc: String = fs::read_to_string(INPUT_FILENAME).expect("invalid filename");
    fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn run_program(program: &mut Program) {
    let mut index: usize = 0;

    while let Some(new_index) = perform_operation(index, program) {
        index = new_index;
    }
}

fn run_permutation(permutation: &[i32], program: &Program) -> i32 {
    unsafe {
        STDOUT_CONTENTS = 0;
        for i in 0..5 {
            STDIN_CONTENTS = vec![STDOUT_CONTENTS, permutation[i]];
            run_program(&mut program.to_vec());
        }
        STDOUT_CONTENTS
    }
}

pub fn solve() {
    const PHASES: [i32; 5] = [0, 1, 2, 3, 4];
    let program: Program = load_program();
    let mut max_signal: i32 = 0;
    PHASES.to_vec().permutation().for_each(|p| {
        max_signal = std::cmp::max(max_signal, run_permutation(&p, &program));
    });

    println!("Day seven, part 1: {}", max_signal);
}
