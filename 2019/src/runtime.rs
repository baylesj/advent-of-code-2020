use log::debug;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::clone::Clone;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs;

#[path = "loadable.rs"]
mod loadable;
pub use loadable::LoadableFromFile;

// List of operations supported by this computer.
#[repr(i64)]
#[derive(IntoPrimitive, TryFromPrimitive, Debug, Clone, Copy, PartialEq)]
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
    Halt = 99,
}

pub type RuntimeBuffer = Vec<i64>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RuntimeState {
    Initialized,
    Running,
    Paused,
    PausedWaitingForInput,
    Stopped,
}

impl Default for RuntimeState {
    fn default() -> RuntimeState {
        RuntimeState::Initialized
    }
}

#[derive(Default, Debug)]
pub struct Runtime {
    pub buffer: RuntimeBuffer,
    pub state: RuntimeState,
    pub io: VecDeque<i64>,
    // TODO: callbacks?
    pub static_input: Option<i64>,
    relative_base: i64,
    ptr: usize,
}

impl Clone for Runtime {
    fn clone(&self) -> Runtime {
        Runtime {
            buffer: self.buffer.clone(),
            state: RuntimeState::Initialized,
            io: VecDeque::new(),
            static_input: None,
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

fn evaluate_output_index(index: usize, runtime: &mut Runtime, mode: ParameterMode) -> usize {
    let mut output_index = runtime.buffer[index] as i64;
    if mode == ParameterMode::Relative {
        output_index += runtime.relative_base as i64;
    }
    if output_index >= runtime.buffer.len() as i64 {
        let new_size: i64 = output_index * 2;
        debug!(
            "received index {}, resizing to new size: {}",
            output_index, new_size
        );

        runtime.buffer.resize(new_size.try_into().unwrap(), 0);
    }
    output_index.try_into().unwrap()
}

fn evaluate_index(index: usize, runtime: &mut Runtime, mode: ParameterMode) -> usize {
    let actual_index: usize;
    match mode {
        ParameterMode::Immediate => actual_index = index,
        ParameterMode::Position => actual_index = runtime.buffer[index] as usize,
        ParameterMode::Relative => {
            actual_index = (runtime.buffer[index] + runtime.relative_base) as usize
        }
    }

    if actual_index >= runtime.buffer.len() {
        let new_size: usize = actual_index * 2;
        debug!(
            "received index {}, resizing to new size: {}",
            actual_index, new_size
        );
        runtime.buffer.resize(new_size, 0);
    }
    actual_index
}

fn access_parameter(index: usize, runtime: &mut Runtime, mode: ParameterMode) -> i64 {
    let index: usize = evaluate_index(index, runtime, mode);
    runtime.buffer[index]
}

fn operation_add(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    let r_i: usize = evaluate_output_index(runtime.ptr + 3, runtime, modes[2]);
    runtime.buffer[r_i] = a + b;
    runtime.ptr += 4;
}

// TODO: create generic and pass operator?
fn operation_multiply(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    let r_i: usize = evaluate_output_index(runtime.ptr + 3, runtime, modes[2]);
    runtime.buffer[r_i] = a * b;
    runtime.ptr += 4;
}

fn operation_input(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    if runtime.io.len() == 0 {
        runtime.state = RuntimeState::PausedWaitingForInput;
        return;
    }

    let value: i64;
    value = runtime.io.pop_front().expect("checked");
    let r_i: usize = evaluate_output_index(runtime.ptr + 1, runtime, modes[0]);
    runtime.buffer[r_i] = value;
    runtime.ptr += 2;
}

fn operation_output(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let value: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    runtime.io.push_back(value);
    runtime.state = RuntimeState::Paused;
    runtime.ptr += 2;
}

fn operation_jump_if_true(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    if a != 0 {
        runtime.ptr = b as usize;
    } else {
        runtime.ptr += 3;
    }
}

fn operation_jump_if_false(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    if a == 0 {
        runtime.ptr = b as usize;
    } else {
        runtime.ptr += 3;
    }
}

fn operation_less_than(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    let r_i: usize = evaluate_output_index(runtime.ptr + 3, runtime, modes[2]);
    if a < b {
        runtime.buffer[r_i] = 1;
    } else {
        runtime.buffer[r_i] = 0;
    }
    runtime.ptr += 4;
}

fn operation_equals(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let a: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    let b: i64 = access_parameter(runtime.ptr + 2, runtime, modes[1]);
    let r_i: usize = evaluate_output_index(runtime.ptr + 3, runtime, modes[2]);
    if a == b {
        runtime.buffer[r_i] = 1;
    } else {
        runtime.buffer[r_i] = 0;
    }
    runtime.ptr += 4;
}

fn operation_set_relative_base(runtime: &mut Runtime, modes: &Vec<ParameterMode>) {
    let offset: i64 = access_parameter(runtime.ptr + 1, runtime, modes[0]);
    runtime.relative_base += offset;
    runtime.ptr += 2;
}

fn operation_halt(runtime: &mut Runtime) {
    runtime.state = RuntimeState::Stopped;
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

fn perform_operation(runtime: &mut Runtime) {
    let op = runtime.buffer[runtime.ptr];
    let op_code = OpCode::try_from(dig(op, 1) + dig(op, 0)).expect("invalid opcode");
    let modes: Vec<ParameterMode> = vec![dig_mode(op, 2), dig_mode(op, 3), dig_mode(op, 4)];

    match op_code {
        OpCode::Add => operation_add(runtime, &modes),
        OpCode::Multiply => operation_multiply(runtime, &modes),
        OpCode::Input => operation_input(runtime, &modes),
        OpCode::Output => operation_output(runtime, &modes),
        OpCode::JumpIfTrue => operation_jump_if_true(runtime, &modes),
        OpCode::JumpIfFalse => operation_jump_if_false(runtime, &modes),
        OpCode::LessThan => operation_less_than(runtime, &modes),
        OpCode::Equals => operation_equals(runtime, &modes),
        OpCode::SetRelativeBase => operation_set_relative_base(runtime, &modes),
        OpCode::Halt => operation_halt(runtime),
    }
}

pub trait Executable {
    fn run(self: &mut Self);
    fn run_until_halted(self: &mut Self);
}

impl Executable for Runtime {
    // The default run implementation only runs until we hit a pause event,
    // i.e. we output to the console.
    fn run(self: &mut Self) {
        self.state = RuntimeState::Running;
        while self.state == RuntimeState::Running {
            perform_operation(self);
        }
    }

    // This implementation runs until HALT operation occurs.
    fn run_until_halted(self: &mut Self) {
        self.state = RuntimeState::Running;
        while self.state != RuntimeState::Stopped
            && self.state != RuntimeState::PausedWaitingForInput
        {
            if self.state != RuntimeState::Running {
                self.state = RuntimeState::Running;
            }
            perform_operation(self);
        }
    }
}

impl LoadableFromFile for Runtime {
    fn load(filename: &str) -> Self {
        let fc: String = fs::read_to_string(filename).expect("invalid filename");

        let mut runtime = Runtime::default();
        runtime.buffer = fc.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        runtime
    }
}

// Some test assumptions:
// 1. data is well formed, any errors in input data should cause an assert.
//    hence, any major errors are "expect" or "assert" and we don't use any
//    result or option returns.
#[cfg(test)]
mod tests {
    use super::*;

    fn test_operation_immediate_mode(
        op_code: OpCode,
        input: &Vec<i64>,
        output: &Vec<i64>,
        io: &VecDeque<i64>,
    ) -> Runtime {
        let mut runtime = Runtime::default();
        runtime.io.append(&mut io.clone());

        // 1 = Immediate mode, so add for every possible parameter.
        let op: i64 = op_code.into();
        runtime.buffer.push(op + 11100);
        // TODO: avoid unnecessary copies?
        runtime.buffer.append(&mut input.clone());

        // Output is always position mode.
        let parameter_start_index: i64 = input.len() as i64 + output.len() as i64 + 2;
        for i in 0..output.len() {
            runtime.buffer.push(parameter_start_index + i as i64);
        }
        runtime.buffer.push(OpCode::Halt.into());
        runtime.buffer.append(&mut output.clone());
        runtime.run();

        let output_offset = runtime.buffer.len() - output.len();
        for i in 0..output.len() {
            assert_eq!(runtime.buffer[i + output_offset], output[i]);
        }
        runtime
    }

    fn test_operation_parameter_mode(
        op_code: OpCode,
        input: &Vec<i64>,
        output: &Vec<i64>,
        io: &VecDeque<i64>,
    ) -> Runtime {
        let mut runtime = Runtime::default();
        runtime.io.append(&mut io.clone());
        runtime.buffer.push(op_code.into());
        // +2 for op_code and OpCode::Halt.
        let parameter_start_index: i64 = input.len() as i64 + output.len() as i64 + 2;
        for i in 0..input.len() {
            runtime.buffer.push(parameter_start_index + i as i64);
        }
        for i in 0..output.len() {
            runtime
                .buffer
                .push(parameter_start_index + input.len() as i64 + i as i64);
        }
        runtime.buffer.push(OpCode::Halt.into());
        runtime.buffer.append(&mut input.clone());
        for o in output {
            runtime.buffer.push(-1 * o);
        }

        runtime.run();

        let output_offset = runtime.buffer.len() - output.len();
        for i in 0..output.len() {
            assert_eq!(runtime.buffer[i + output_offset], output[i]);
        }
        runtime
    }

    fn test_operation(
        op_code: OpCode,
        input_opt: Option<Vec<i64>>,
        output_opt: Option<Vec<i64>>,
        io_opt: Option<Vec<i64>>,
    ) -> Vec<Runtime> {
        let input = input_opt.unwrap_or_default();
        let output = output_opt.unwrap_or_default();
        let io: VecDeque<i64> = io_opt.unwrap_or_default().into_iter().collect();

        vec![
            test_operation_parameter_mode(op_code, &input, &output, &io),
            test_operation_immediate_mode(op_code, &input, &output, &io),
        ]
    }

    #[test]
    fn test_operation_halt() {
        test_operation(OpCode::Halt, None, None, None);
    }

    #[test]
    fn test_operation_add() {
        test_operation(OpCode::Add, Some(vec![10, 20]), Some(vec![30]), None);
        test_operation(OpCode::Add, Some(vec![-1, -2]), Some(vec![-3]), None);
        test_operation(OpCode::Add, Some(vec![0, 0]), Some(vec![0]), None);
    }

    #[test]
    fn test_operation_multiply() {
        test_operation(OpCode::Multiply, Some(vec![10, 20]), Some(vec![200]), None);
        test_operation(OpCode::Multiply, Some(vec![-1, -2]), Some(vec![2]), None);
        test_operation(OpCode::Multiply, Some(vec![0, 0]), Some(vec![0]), None);
    }

    #[test]
    fn test_operation_input() {
        test_operation(OpCode::Input, None, Some(vec![1337]), Some(vec![1337]));
    }

    #[test]
    #[should_panic]
    fn test_operation_input_empty_stack_asserts() {
        test_operation(OpCode::Input, None, Some(vec![1337]), None);
    }

    #[test]
    fn test_operation_output() {
        test_operation(OpCode::Output, Some(vec![42]), None, None)
            .iter()
            .for_each(|p| {
                assert_eq!(p.io.len(), 1);
                assert_eq!(p.io[0], 42);
            });
    }

    #[test]
    fn test_operation_jump_if_true() {
        // The jump instructions are a little tricky because we still need to
        // halt. So, this runtime has an extra output instruction that we should
        // skip if we perform a jump operation.
        test_operation(OpCode::JumpIfTrue, Some(vec![1, 5, 4, 13]), None, None)
            .iter()
            .for_each(|p| {
                assert_eq!(p.ptr, 5);
                assert_eq!(p.io.len(), 0);
            });
    }

    #[test]
    #[should_panic]
    fn test_operation_jump_if_true_false_case() {
        // Ensure that our strategy works by testing again with false.
        test_operation(OpCode::JumpIfTrue, Some(vec![0, 5, -1]), None, None);
    }

    #[test]
    fn test_operation_jump_if_false() {
        test_operation(OpCode::JumpIfFalse, Some(vec![0, 5, 4, 13]), None, None)
            .iter()
            .for_each(|p| {
                assert_eq!(p.ptr, 5);
                assert_eq!(p.io.len(), 0);
            });
    }

    #[test]
    #[should_panic]
    fn test_operation_jump_if_false_true_case() {
        // Ensure that our strategy works by testing again with false.
        test_operation(OpCode::JumpIfFalse, Some(vec![1, 5, -1]), None, None);
    }

    #[test]
    fn test_operation_less_than() {
        test_operation(OpCode::LessThan, Some(vec![1, 2]), Some(vec![1]), None);
        test_operation(OpCode::LessThan, Some(vec![10, 2]), Some(vec![0]), None);
        test_operation(OpCode::LessThan, Some(vec![0, 0]), Some(vec![0]), None);
    }

    #[test]
    fn test_operation_equals() {
        test_operation(OpCode::Equals, Some(vec![1, 2]), Some(vec![0]), None);
        test_operation(OpCode::Equals, Some(vec![10, 2]), Some(vec![0]), None);
        test_operation(OpCode::Equals, Some(vec![0, 0]), Some(vec![1]), None);
        test_operation(OpCode::Equals, Some(vec![-10, -10]), Some(vec![1]), None);
    }

    #[test]
    fn test_set_relative_base() {
        let add: i64 = OpCode::Add.into();
        let mut runtime = Runtime::default();
        runtime.buffer = vec![
            OpCode::SetRelativeBase.into(), // index 0
            4,                              // index 1
            // Now the relative base is four...
            2200 + add, // index 2
            3,          // value 3 + rel base = points to 7
            4,          // value 4 + rel base = points to 8
            9,          // index 5 -> points direct to 9
            99,         // index 6
            10,         // index 7
            20,         // index 8
            -1,         // index 9
        ];

        runtime.run();
        assert_eq!(runtime.buffer[9], 30);
    }
}
