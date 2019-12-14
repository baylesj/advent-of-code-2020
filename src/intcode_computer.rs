use queues::IsQueue;
use queues::Queue;
use std::clone::Clone;
use std::fs;
use num_traits::FromPrimitive;
use enum_primitive_derive::Primitive;

// List of operations supported by this computer.
#[derive(Primitive, Debug, Clone, Copy, PartialEq)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    SetRelativeBase = 9,
    Halt = 99
}

pub type ProgramBuffer = Vec<i64>;

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

#[derive(Default, Debug)]
pub struct Program {
    pub buffer: ProgramBuffer,
    pub state: ProgramState,
    pub io: Queue<i64>,
    relative_base: i64,
    ptr: usize,
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            buffer: self.buffer.clone(),
            state: ProgramState::Initialized,
            io: Queue::new(),
            relative_base: self.relative_base,
            ptr: 0,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

// NOTE: ONLY for read only params.
fn access_parameter(index: usize, program: &Program, mode: ParameterMode) -> i64 {
    match mode {
        ParameterMode::Immediate => program.buffer[index],
        ParameterMode::Position => program.buffer[program.buffer[index] as usize],
        ParameterMode::Relative => {
            program.buffer[program.buffer[index] as usize] + program.relative_base
        }
    }
}

fn operation_add(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    program.buffer[r_i] = a + b;
    program.ptr += 4;
}

// TODO: create generic and pass operator?
fn operation_multiply(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    program.buffer[r_i] = a * b;
    program.ptr += 4;
}

fn operation_input(program: &mut Program) {
    let value: i64 = program.io.remove().expect("requested input on empty stack");
    let r_i: usize = program.buffer[program.ptr + 1] as usize;
    program.buffer[r_i] = value;
    program.ptr += 2;
}

fn operation_output(program: &mut Program, modes: &Vec<ParameterMode>) {
    let value: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    program.io.add(value).ok();
    program.state = ProgramState::Paused;
    program.ptr += 2;
}

fn operation_jump_if_true(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    if a != 0 {
        program.ptr = b as usize;
    } else {
        program.ptr += 3;
    }
}

fn operation_jump_if_false(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    if a == 0 {
        program.ptr = b as usize;
    } else {
        program.ptr += 3;
    }
}

fn operation_less_than(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    if a < b {
        program.buffer[r_i] = 1;
    } else {
        program.buffer[r_i] = 0;
    }
    program.ptr += 4;
}

fn operation_equals(program: &mut Program, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    let b: i64 = access_parameter(program.ptr + 2, program, modes[1]);
    let r_i: usize = program.buffer[program.ptr + 3] as usize;
    if a == b {
        program.buffer[r_i] = 1;
    } else {
        program.buffer[r_i] = 0;
    }
    program.ptr += 4;
}

fn operation_set_relative_base(program: &mut Program, modes: &Vec<ParameterMode>) {
    let new_base: i64 = access_parameter(program.ptr + 1, program, modes[0]);
    program.relative_base = new_base;
    program.ptr += 2;
}

fn operation_halt(program: &mut Program) {
    program.state = ProgramState::Stopped;
}

fn dig(op: i64, place: u32) -> i64 {
    let t = i64::pow(10, place);
    ((op / t) % 10) * t
}

fn dig_mode(op: i64, place: u32) -> ParameterMode {
    match dig(op, place) / i64::pow(10, place) {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        _ => panic!("invalid parameter mode {}", dig(op, place)),
    }
}

fn perform_operation(program: &mut Program) {
    let op = program.buffer[program.ptr];
    let op_code = OpCode::from_i64(dig(op, 1) + dig(op, 0)).expect("invalid opcode");
    let modes: Vec<ParameterMode> = vec![dig_mode(op, 2), dig_mode(op, 3), dig_mode(op, 4)];

    match op_code {
        OpCode::Add => operation_add(program, &modes),
        OpCode::Multiply => operation_multiply(program, &modes),
        OpCode::Input => operation_input(program),
        OpCode::Output => operation_output(program, &modes),
        OpCode::JumpIfTrue => operation_jump_if_true(program, &modes),
        OpCode::JumpIfFalse => operation_jump_if_false(program, &modes),
        OpCode::LessThan => operation_less_than(program, &modes),
        OpCode::Equals => operation_equals(program, &modes),
        OpCode::SetRelativeBase => operation_set_relative_base(program, &modes),
        OpCode::Halt => operation_halt(program),
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

// TODO: move to growable buffer size.
const PROGRAM_MEMORY_SIZE: usize = 1 * 1024 * 1024;
impl LoadableFromFile for Program {
    fn load(filename: &str) -> Self {
        let fc: String = fs::read_to_string(filename).expect("invalid filename");

        let mut program = Program::default();
        program.buffer = fc.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        program.buffer.resize(PROGRAM_MEMORY_SIZE, 0);
        program
    }
}

// Some test assumptions:
// 1. data is well formed, any errors in input data should cause an assert.
//    hence, any major errors are "expect" or "assert" and we don't use any
//    result or option returns.
#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::ToPrimitive;

    fn test_operation(op_code: OpCode, input: &mut Vec<i64>, outputs: Vec<i64>, io: Option<Queue<i64>>)
    -> Program {
        let mut program = Program::default();
        program.io = io.unwrap_or_default();

        let parameter_start_index: i64 = input.len() as i64 + 2;
        program.buffer.push(op_code.to_i64().unwrap());
        for i in 0..input.len() + 1 {
            program.buffer.push(parameter_start_index + i as i64 + 1);
        }
        // Push HALT
        program.buffer.push(99);
        program.buffer.append(input);

        for output in &outputs {
            program.buffer.push(-1 * output);
        }

        program.run();

        let output_offset = program.buffer.len() - outputs.len();
        for i in 0..outputs.len() {
            assert_eq!(program.buffer[i + output_offset], outputs[i]);
        }
        program
    }

    #[test]
    fn test_operation_add() {
        test_operation(OpCode::Add, &mut vec![10, 20], vec![30], None);
        test_operation(OpCode::Add, &mut vec![-1, -2], vec![-3], None);
        test_operation(OpCode::Add, &mut vec![0, 0], vec![0], None);
    }

    #[test]
    fn test_operation_multiply() {
        test_operation(OpCode::Multiply, &mut vec![10, 20], vec![200], None);
        test_operation(OpCode::Multiply, &mut vec![-1, -2], vec![2], None);
        test_operation(OpCode::Multiply, &mut vec![0, 0], vec![0], None);
    }

    #[test]
    fn test_operation_input() {
        let mut io = Queue::new();
        io.add(1337).ok();
        test_operation(OpCode::Input, &mut vec![], vec![1337], Some(io));
    }

    #[test]
    #[should_panic]
    fn test_operation_input_empty_stack_asserts() {
        let io = Queue::new();
        test_operation(OpCode::Input, &mut vec![], vec![1337], Some(io));
    }
}
