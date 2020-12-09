use crate::loadable::LoadableFromFile;
use crate::validity::Validity;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_four.txt";

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Passport {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: String,
    hair_color: String,
    eye_color: String,
    id: String,
    country_id: String,
}

impl Validity for Passport {
    // For part one, country ID is optional.
    fn is_valid(&self) -> bool {
        return self.birth_year > 0
            && self.issue_year > 0
            && self.expiration_year > 0
            && !self.height.is_empty()
            && !self.hair_color.is_empty()
            && !self.eye_color.is_empty()
            && !self.id.is_empty();
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl LoadableFromFile for Vec<Passport> {
    fn load(filename: &str) -> Vec<Passport> {
        let re: Regex = Regex::new("([a-z]+):([a-z0-9#]+)").unwrap();
        let file = File::open(filename).expect("Invalid filename");

        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut passports: Vec<Passport> = Vec::new();
        let mut current = Passport::default();
        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    // This either means we hit the EOF or a Passport separator.
                    if line.trim().is_empty() {
                        if current.is_valid() {
                            passports.push(current);
                        }
                        if bytes_read == 0 {
                            break;
                        } else {
                            // We just ignore invalid passports currently.
                            current = Passport::default();
                        }
                    }
                    for found in re.captures_iter(&line) {
                        match &found[1] {
                            "byr" => current.birth_year = found[2].parse().unwrap(),
                            "iyr" => current.issue_year = found[2].parse().unwrap(),
                            "eyr" => current.expiration_year = found[2].parse().unwrap(),
                            "hgt" => current.height = found[2].to_owned(),
                            "hcl" => current.hair_color = found[2].to_owned(),
                            "ecl" => current.eye_color = found[2].to_owned(),
                            "pid" => current.id = found[2].to_owned(),
                            "cid" => current.country_id = found[2].to_owned(),
                            _ => panic!(),
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
            line.clear();
        }
        passports
    }
}

pub fn part_two(passports: &Vec<Passport>) -> i64 {
    passports.len() as i64
}

pub fn solve() -> String {
    let passports = Vec::<Passport>::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        passports.len(),
        part_two(&passports)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solves_part_one_example() {
        const INPUT_FILENAME: &'static str = "input/day_four_part_one_example.txt";
        let passports = Vec::<Passport>::load(INPUT_FILENAME);
        assert_eq!(2, passports.len());
    }

    #[test]
    pub fn solves() {
        //assert_eq!("part one: 176, part two: 5872458240", solve());
    }
}
