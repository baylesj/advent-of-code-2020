use std::fs;
use text_io::read;

const INPUT_FILENAME: &str = "input/day_five.txt";

#[derive(PartialEq, Clone, Copy)]
enum ParameterMode {
    Immediate,
    Position,
}

// NOTE: ONLY for read only params.
fn access_parameter(index: usize, program: &Vec<i32>, mode: ParameterMode) -> i32 {
    if mode == ParameterMode::Immediate {
        program[index]
    } else {
        program[program[index] as usize]
    }
}

fn operation_code_one(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a + b;
}

// TODO: create generic and pass operator?
fn operation_code_two(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a * b;
}

fn operation_code_three(index: usize, program: &mut Vec<i32>) {
    let r_i: usize = program[index + 1] as usize;

    println!("Enter system ID:");
    let i: i32 = read!();
    program[r_i] = i;
}

fn operation_code_four(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    println!("Output: {}", a);
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

fn perform_operation(index: usize, program: &mut Vec<i32>) -> Option<usize> {
    let op = program[index];
    let op_code: i32 = dig(op, 1) + dig(op, 0);
    let modes: Vec<ParameterMode> = vec![dig_mode(op, 2), dig_mode(op, 3), dig_mode(op, 4)];

    match op_code {
        1 => {
            operation_code_one(index, program, &modes);
            Some(4)
        }
        2 => {
            operation_code_two(index, program, &modes);
            Some(4)
        }
        3 => {
            operation_code_three(index, program);
            Some(2)
        }
        4 => {
            operation_code_four(index, program, &modes);
            Some(2)
        }
        // 99 is HALT.
        99 => None,
        _ => panic!("unknown OP code: {}", op_code),
    }
}

fn load_program() -> Vec<i32> {
    let fc: String = fs::read_to_string(INPUT_FILENAME).expect("invalid filename");
    fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn run_program(program: &mut Vec<i32>) {
    let mut index: usize = 0;

    while let Some(adv) = perform_operation(index, program) {
        index += adv
    }
}

fn part_one() {
    let mut program = load_program();
    run_program(&mut program);
}

pub fn solve() -> String {
    part_one();
    String::from("Check stdout.")
}
