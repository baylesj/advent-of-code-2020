use crate::loadable::LoadableFromFile;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT_FILENAME: &'static str = "input/day_two.txt";

struct PasswordAndPolicy {
    min_count: i8,
    max_count: i8,
    required_letter: char,
    password: String,
}

trait Validity {
    fn is_valid(&self) -> bool;
}

impl Validity for PasswordAndPolicy {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.required_letter).count() as i8;
        self.min_count <= count && count <= self.max_count
    }
}

trait NewValidity {
    fn is_new_valid(&self) -> bool;
}

impl NewValidity for PasswordAndPolicy {
    fn is_new_valid(&self) -> bool {
        (self.password.as_bytes()[self.min_count as usize - 1] as char == self.required_letter)
            != (self.password.as_bytes()[self.max_count as usize - 1] as char
                == self.required_letter)
    }
}

impl FromStr for PasswordAndPolicy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)$").unwrap();
        let caps = re.captures(s).unwrap();
        Ok(PasswordAndPolicy {
            min_count: caps[1].parse().unwrap(),
            max_count: caps[2].parse().unwrap(),
            required_letter: caps[3].parse().unwrap(),
            password: caps[4].to_string(),
        })
    }
}

impl LoadableFromFile for Vec<PasswordAndPolicy> {
    fn load(filename: &str) -> Vec<PasswordAndPolicy> {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|r| match r {
                Ok(n) => PasswordAndPolicy::from_str(&n).unwrap(),
                Err(e) => panic!(e),
            })
            .collect()
    }
}

pub fn solve() -> String {
    let passwords = Vec::<PasswordAndPolicy>::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        passwords
            .iter()
            .fold(0, |sum, p| { sum + p.is_valid() as i32 }),
        passwords
            .iter()
            .fold(0, |sum, p| { sum + p.is_new_valid() as i32 })
    )
}
