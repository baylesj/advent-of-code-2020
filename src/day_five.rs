use std::fs;
//use text_io::read;

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

fn operation_add(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a + b;
}

// TODO: create generic and pass operator?
fn operation_multiply(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    program[r_i] = a * b;
}

fn read() -> i32 {
    // Chosen by dice roll. Guaranteed to be unique.
    1
}

fn operation_input(index: usize, program: &mut Vec<i32>) {
    let r_i: usize = program[index + 1] as usize;

    println!("SYSTEM INPUT REQUESTED: 1");
    let i: i32 = read(); //read!();
    program[r_i] = i;
}

fn operation_output(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    if a > 0 {
        println!("SYSTEM OUTPUT PROVIDED: {}", a);
    }
}

fn operation_jump_if_true(
    index: usize,
    program: &mut Vec<i32>,
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
    program: &mut Vec<i32>,
    modes: &Vec<ParameterMode>,
) -> usize {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    if a == 0 {
        return b as usize;
    }
    index + 3
}

fn operation_less_than(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(index + 1, program, modes[0]);
    let b: i32 = access_parameter(index + 2, program, modes[1]);
    let r_i: usize = program[index + 3] as usize;
    if a < b {
        program[r_i] = 1;
    } else {
        program[r_i] = 0;
    }
}

fn operation_equals(index: usize, program: &mut Vec<i32>, modes: &Vec<ParameterMode>) {
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

fn perform_operation(index: usize, program: &mut Vec<i32>) -> Option<usize> {
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

fn load_program() -> Vec<i32> {
    let fc: String = fs::read_to_string(INPUT_FILENAME).expect("invalid filename");
    fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}

fn run_program(program: &mut Vec<i32>) {
    let mut index: usize = 0;

    while let Some(new_index) = perform_operation(index, program) {
        index = new_index;
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
