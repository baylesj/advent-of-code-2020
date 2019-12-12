use log::debug;
use std::fs;

pub type ProgramBuffer = Vec<i32>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ProgramState {
    Initialized,
    Running,
    Paused,
    Stopped,
}

impl Default for ProgramState {
    fn default() -> ProgramState {
        ProgramState::Initialized
    }
}

#[derive(Default, Clone, Debug)]
pub struct Program {
    pub buffer: ProgramBuffer,
    pub state: ProgramState,
    pub io: Vec<i32>,
    ptr: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum ParameterMode {
    Immediate,
    Position,
}

// NOTE: ONLY for read only params.
fn access_parameter(index: usize, program: &Program, mode: ParameterMode) -> i32 {
    if mode == ParameterMode::Immediate {
        program.buffer[index]
    } else {
        program.buffer[program.buffer[index] as usize]
    }
}

fn operation_add(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    program.buffer[r_i] = a + b;
    program.ptr += 4;
}

// TODO: create generic and pass operator?
fn operation_multiply(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    program.buffer[r_i] = a * b;
    program.ptr += 4;
}

fn operation_input(program: &mut Program) {
    let value: i32 = program.io.pop().expect("requested input on empty stack");
    let r_i: usize = program.buffer[program.ptr + 1] as usize;
    debug!("operation input: {}", value);
    program.buffer[r_i] = value;
    program.ptr += 2;
}

fn operation_output(program: &mut Program, modes: &Vec<ParameterMode>) {
    let value: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    program.io.push(value);
    debug!("operation output: {}", value);
    program.state = ProgramState::Paused;
    program.ptr += 2;
}

fn operation_jump_if_true(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    if a != 0 {
        program.ptr = b as usize;
    } else {
        program.ptr += 3;
    }
}

fn operation_jump_if_false(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    if a == 0 {
        program.ptr = b as usize;
    } else {
        program.ptr += 3;
    }
}

fn operation_less_than(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    if a < b {
        program.buffer[r_i] = 1;
    } else {
        program.buffer[r_i] = 0;
    }
    program.ptr += 4;
}

fn operation_equals(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i32 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i32 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    if a == b {
        program.buffer[r_i] = 1;
    } else {
        program.buffer[r_i] = 0;
    }
    program.ptr += 4;
}

fn operation_halt(program: &mut Program) {
    program.state = ProgramState::Stopped;
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

fn perform_operation(program: &mut Program) {
    let op = program.buffer[program.ptr];
    let op_code: i32 = dig(op, 1) + dig(op, 0);
    let modes: Vec<ParameterMode> = vec![dig_mode(op, 2), dig_mode(op, 3), dig_mode(op, 4)];

    match op_code {
        1 => operation_add(program, &modes),
        2 => operation_multiply(program, &modes),
        3 => operation_input(program),
        4 => operation_output(program, &modes),
        5 => operation_jump_if_true(program, &modes),
        6 => operation_jump_if_false(program, &modes),
        7 => operation_less_than(program, &modes),
        8 => operation_equals(program, &modes),
        99 => operation_halt(program),
        _ => panic!("unknown OP code: {}", op_code),
    }
}

pub trait Runnable {
    fn run(self: &mut Self);
}

impl Runnable for Program {
    fn run(self: &mut Self) {
        self.state = ProgramState::Running;
        while self.state == ProgramState::Running {
            perform_operation(self);
        }
    }
}

// TODO: trait useful for other classes?
pub trait LoadableFromFile {
    fn load(filename: &str) -> Self;
}

impl LoadableFromFile for Program {
    fn load(filename: &str) -> Self {
        let fc: String = fs::read_to_string(filename).expect("invalid filename");

        let mut program = Program::default();
        program.buffer = fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        program
    }
}
