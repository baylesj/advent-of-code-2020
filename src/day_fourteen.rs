use crate::loadable::LoadableFromFile;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

// Program addresses are 16-bit.
const MAX_PROGRAM_ADDRESS: usize = 2usize.pow(16);

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
struct Mask {
    // Example mask: X1X0X
    // "1" values in the mask are applied using bitwise or,
    // e.g. for the example this is 01000.
    positive_mask: i64,
    // "0" values in the mask are applied using bitwise and.
    // e.g. for the example this is 11101.
    negative_mask: i64,
}
impl fmt::Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ positive: {:b}, negative: {:b} }}",
            self.positive_mask, self.negative_mask
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    SetMask(Mask),
    SetMemory(usize, i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        const MASK_STRING: &'static str = "mask = ";

        if s.starts_with(MASK_STRING) {
            let mut positive_mask: i64 = 0;
            let mut negative_mask: i64 = 0;
            for c in s.bytes().skip(MASK_STRING.len()).map(|b| b as char) {
                match c {
                    'X' => {
                        negative_mask += 1;
                    }
                    '1' => {
                        positive_mask += 1;
                        negative_mask += 1;
                    }
                    '0' => {
                        // Zero in the negative mask place, and in the
                        // positive mask case.
                    }
                    _ => panic!("unknown char"),
                }
                positive_mask <<= 1;
                negative_mask <<= 1;
            }
            // TODO: hate this.
            positive_mask >>= 1;
            negative_mask >>= 1;
            Instruction::SetMask(Mask {
                positive_mask: positive_mask,
                negative_mask: negative_mask,
            })
        } else {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
            }
            let m = RE.captures(s).unwrap();
            Instruction::SetMemory(
                m.get(1)
                    .map(|x| usize::from_str(x.as_str()).unwrap())
                    .unwrap(),
                m.get(2)
                    .map(|x| i64::from_str(x.as_str()).unwrap())
                    .unwrap(),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Program {
    instructions: Vec<Instruction>,
    memory: [i64; MAX_PROGRAM_ADDRESS],
    index: usize,
    mask: Mask,
}

impl LoadableFromFile for Program {
    fn load(filename: &str) -> Program {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        Program {
            instructions: reader
                .lines()
                .map(|l| Instruction::from(&(l.unwrap())[..]))
                .collect(),
            memory: [0; MAX_PROGRAM_ADDRESS],
            index: 0,
            mask: Mask::default(),
        }
    }
}

// TODO: can definitely clean up uses of traits.
trait Advanceable {
    fn advance(&mut self);
}

// TODO: turn into trait
impl Advanceable for Program {
    fn advance(&mut self) {
        match self.instructions[self.index] {
            Instruction::SetMask(mask) => {
                self.mask = mask;
            }
            Instruction::SetMemory(address, value) => {
                let result_value = (value & self.mask.negative_mask) | self.mask.positive_mask;
                self.memory[address] = result_value;
            }
        }
        self.index += 1;
    }
}

fn part_one(program: &Program) -> i64 {
    let mut p = program.clone();
    while p.index < p.instructions.len() {
        p.advance();
    }
    p.memory.iter().sum()
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let program = Program::load("input/day_fourteen.txt");
    format!("part one: {}, part two: {}", part_one(&program), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 13556564111697, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        let program = Program::load("input/day_fourteen_example.txt");
        assert_eq!(165, part_one(&program));
    }
}
