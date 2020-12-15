use crate::loadable::LoadableFromFile;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Mask {
    // Example mask: X1X0X
    // "1" values in the mask are applied using bitwise or,
    // e.g. for the example this is 01000.
    positive_mask: i64,

    // "0" values in the mask are applied using bitwise and.
    // e.g. for the example this is 11101.
    negative_mask: i64,

    // All of the "X" indices, ignored in part one but critical in part two.
    floating_indices: Vec<usize>,
}
impl fmt::Binary for Mask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ positive: {:b}, negative: {:b}, floating: {:?} }}",
            self.positive_mask, self.negative_mask, self.floating_indices
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    SetMask(Mask),
    SetMemory(usize, i64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        const MASK_STRING: &'static str = "mask = ";

        if s.starts_with(MASK_STRING) {
            let mut positives: i64 = 0;
            let mut negatives: i64 = 0;
            let mut floaters = vec![];
            let mask_string: Vec<u8> = s.bytes().skip(MASK_STRING.len()).collect();
            for b in mask_string.iter().enumerate() {
                match *b.1 as char {
                    'X' => {
                        negatives += 1;
                        // We go from left to right, but the actual mask is right
                        // to left so we flip the indices here.
                        floaters.push(mask_string.len() - b.0 - 1);
                    }
                    '1' => {
                        positives += 1;
                        negatives += 1;
                    }
                    '0' => {
                        // Zero in the negative mask place, and in the
                        // positive mask case.
                    }
                    _ => panic!("unknown char"),
                }
                positives <<= 1;
                negatives <<= 1;
            }
            // TODO: hate this.
            positives >>= 1;
            negatives >>= 1;
            Instruction::SetMask(Mask {
                positive_mask: positives,
                negative_mask: negatives,
                floating_indices: floaters,
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
    memory: HashMap<usize, i64>,
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
            memory: HashMap::new(),
            index: 0,
            mask: Mask::default(),
        }
    }
}

fn get_addresses(value: usize, mask: &Mask) -> Vec<usize> {
    let mut addresses: Vec<usize> = Vec::with_capacity(2 << mask.floating_indices.len() + 1);
    addresses.push(0);
    let mut masked_value = value;
    for index in &mask.floating_indices {
        // Remove floating fields from value to avoid doubly counting them.
        masked_value &= !(1 << index);
        let contrib: usize = 1 << index;
        // Half of the address are left alone
        let addresses_to_add: Vec<usize> = addresses.iter().map(|a| a + contrib).collect();
        addresses.extend(addresses_to_add);
    }
    // Finally, after we have cropped the floating fields, we just need to
    // add the positive mask (the negative mask is ignored).
    masked_value |= mask.positive_mask as usize;
    for address in &mut addresses {
        *address += masked_value;
    }
    addresses
}

// TODO: can definitely clean up uses of traits.
trait Advanceable {
    fn advance(&mut self);
    fn advance_with_address_mask(&mut self);
}

// TODO: turn into trait
impl Advanceable for Program {
    fn advance(&mut self) {
        match &self.instructions[self.index] {
            Instruction::SetMask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::SetMemory(address, value) => {
                let result_value = (value & self.mask.negative_mask) | self.mask.positive_mask;
                self.memory.insert(*address, result_value);
            }
        }
        self.index += 1;
    }
    fn advance_with_address_mask(&mut self) {
        match &self.instructions[self.index] {
            Instruction::SetMask(mask) => {
                self.mask = mask.clone();
            }
            Instruction::SetMemory(address, value) => {
                for address in get_addresses(*address as usize, &self.mask) {
                    self.memory.insert(address, *value);
                }
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
    p.memory.iter().map(|(_, v)| v).sum()
}

// Part two approach:
// Dynamic programming. Iterate through each bit in mask, if 1 then override
// true, 0 then leave alone, floating then split
// great
fn part_two(program: &Program) -> i64 {
    let mut p = program.clone();
    while p.index < p.instructions.len() {
        p.advance_with_address_mask();
    }
    p.memory.iter().map(|(_, v)| v).sum()
}

pub fn solve() -> String {
    let program = Program::load("input/day_fourteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&program),
        part_two(&program)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 13556564111697, part two: 4173715962894", solve());
    }

    #[test]
    fn test_example() {
        let program = Program::load("input/day_fourteen_example.txt");
        assert_eq!(165, part_one(&program));
    }

    #[test]
    fn test_example_two() {
        let program = Program::load("input/day_fourteen_example_two.txt");
        assert_eq!(208, part_two(&program));
    }

    #[test]
    fn test_get_addresses() {
        assert_eq!(
            vec![26, 27, 58, 59],
            get_addresses(
                42,
                &Mask {
                    positive_mask: 0b10010,
                    negative_mask: 0b0,
                    floating_indices: vec![0, 5]
                }
            )
        );
    }
}
