use crate::loadable::LoadableFromFile;
use bitvec::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    // NoOps have values because one of them may be a broken Jump.
    NoOp(i64),
    Accumulate(i64),
    Jump(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    LoopDetected,
    Halted,
    Running,
}

pub struct Program {
    instructions: Vec<Instruction>,
    visited: BitVec,
    index: usize,
    accumulator: i64,
    state: State,
}

trait Advanceable {
    // Returns false if advancing is not possible, e.g. due to loops.
    fn advance(&mut self) -> bool;
    fn advance_while_true(&mut self);
}

trait Resettable {
    fn reset(&mut self);
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name_and_value = s.split_at(3);
        println!("name and value: {:?}", name_and_value);
        // Rust does not approve of leading '+' on integers.
        let value: i64 = name_and_value
            .1
            .trim_start()
            .trim_start_matches('+')
            .parse()
            .unwrap();
        match name_and_value.0 {
            "nop" => Ok(Instruction::NoOp(value)),
            "acc" => Ok(Instruction::Accumulate(value)),
            "jmp" => Ok(Instruction::Jump(value)),
            _ => Err("invalid or unknown instruction"),
        }
    }
}

impl LoadableFromFile for Program {
    fn load(filename: &str) -> Program {
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);

        let instructions: Vec<Instruction> = reader
            .lines()
            .map(|l| Instruction::from_str(&l.unwrap()).expect("invalid instruction"))
            .collect();
        let instruction_count = instructions.len();

        Program {
            instructions: instructions,
            visited: bitvec![0; instruction_count],
            index: 0,
            accumulator: 0,
            state: State::Running,
        }
    }
}

impl Advanceable for Program {
    fn advance(&mut self) -> bool {
        if self.index == self.instructions.len() {
            self.state = State::Halted;
            return false;
        }
        if self.visited[self.index] {
            self.state = State::LoopDetected;
            return false;
        }
        self.visited.set(self.index, true);

        let mut next_index = self.index + 1;
        match &self.instructions[self.index] {
            // Jumping to an earlier instruction is negative, so we need a double
            // cast here.
            Instruction::Jump(to) => next_index = (self.index as i64 + *to) as usize,
            Instruction::Accumulate(value) => self.accumulator += *value,
            Instruction::NoOp(_) => (),
        }
        self.index = next_index;
        true
    }

    fn advance_while_true(&mut self) {
        while self.advance() {}
    }
}

impl Resettable for Program {
    fn reset(&mut self) {
        self.index = 0;
        self.accumulator = 0;
        self.visited.set_elements(0);
        self.state = State::Running;
    }
}

pub fn part_one(program: &mut Program) -> i64 {
    program.advance_while_true();
    program.accumulator
}

pub fn part_two(program: &mut Program) -> i64 {
    for enumerable in program.instructions.clone().iter().enumerate() {
        let original = *enumerable.1;
        let next;
        match original {
            Instruction::Accumulate(_) => continue,
            Instruction::NoOp(val) => next = Instruction::Jump(val),
            Instruction::Jump(val) => next = Instruction::NoOp(val),
        }

        program.instructions[enumerable.0] = next;
        let accumulator_value = part_one(program);
        if program.state == State::Halted {
            return accumulator_value;
        } else {
            assert_eq!(State::LoopDetected, program.state);
            program.instructions[enumerable.0] = original;
            program.reset();
            continue;
        }
    }
    program.accumulator
}

pub fn solve() -> String {
    let mut program = Program::load("input/day_eight.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&mut program),
        part_two(&mut program)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 1814, part two: 1056", solve());
    }

    #[test]
    pub fn test_example() {
        let mut program = Program::load("input/day_eight_example.txt");
        assert_eq!(5, part_one(&mut program));
    }
}
