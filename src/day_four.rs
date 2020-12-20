use crate::loadable::LoadableFromFile;
use crate::validity::Validity;
use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Height {
    Unknown,
    Invalid,
    Inches(i64),
    Centimeters(i64),
}

impl Default for Height {
    fn default() -> Self {
        Height::Unknown
    }
}

impl FromStr for Height {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() <= 3 {
            return Err("invalid height");
        }
        let pair = s.split_at(s.len() - 2);
        let value: i64 = pair.0.parse().unwrap_or_default();
        match pair.1 {
            "in" => Ok(Height::Inches(value)),
            "cm" => Ok(Height::Centimeters(value)),
            _ => Err("failed to parse height"),
        }
    }
}

impl Validity for Height {
    fn is_valid(&self) -> bool {
        match self {
            Height::Inches(i) => (59..77).contains(i),
            Height::Centimeters(cm) => (150..194).contains(cm),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
    Unknown,
    Invalid,
}

impl Default for EyeColor {
    fn default() -> Self {
        EyeColor::Invalid
    }
}

impl FromStr for EyeColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Gray),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Err("Unknown eye color"),
        }
    }
}

impl Validity for EyeColor {
    fn is_valid(&self) -> bool {
        *self != EyeColor::Unknown && *self != EyeColor::Invalid
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RgbColor {
    red: i16,
    green: i16,
    blue: i16,
}

impl Default for RgbColor {
    fn default() -> Self {
        RgbColor {
            red: -1,
            green: -1,
            blue: -1,
        }
    }
}

impl FromStr for RgbColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("#") || s.len() != 7 {
            return Err("missing prepended hash or bad length");
        }
        let color = RgbColor {
            red: i16::from_str_radix(&s[1..3], 16).unwrap_or(-1),
            green: i16::from_str_radix(&s[3..5], 16).unwrap_or(-1),
            blue: i16::from_str_radix(&s[5..7], 16).unwrap_or(-1),
        };
        if !color.is_valid() {
            Err("Invalid produced color")
        } else {
            Ok(color)
        }
    }
}

impl Validity for RgbColor {
    fn is_valid(&self) -> bool {
        self.blue >= 0 && self.green >= 0 && self.red >= 0
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
struct Identifier {
    value: i32,
}

impl FromStr for Identifier {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err("incorrect length ID");
        }

        Ok(Identifier {
            value: s.parse().unwrap_or_default(),
        })
    }
}

impl Validity for Identifier {
    fn is_valid(&self) -> bool {
        self.value > 0
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Passport {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: Height,
    hair_color: RgbColor,
    eye_color: EyeColor,
    id: Identifier,
    country_id: String,
}

impl Validity for Passport {
    // For part one, country ID is optional.
    fn is_valid(&self) -> bool {
        return (1920..2003).contains(&self.birth_year)
            && (2010..2021).contains(&self.issue_year)
            && (2020..2031).contains(&self.expiration_year)
            && self.height.is_valid()
            && self.hair_color.is_valid()
            && self.eye_color.is_valid()
            && self.id.is_valid();
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait Initialized {
    fn was_initialized(&self) -> bool;
}

impl Initialized for Passport {
    // TODO: refactor to be cleaner.
    fn was_initialized(&self) -> bool {
        self.birth_year > 0
            && self.issue_year > 0
            && self.expiration_year > 0
            && self.height != Height::default()
            && self.hair_color != RgbColor::default()
            && self.eye_color != EyeColor::default()
            && self.id != Identifier::default()
    }
}

struct Passports {
    contains_invalid_fields: Vec<Passport>,
    valid: Vec<Passport>,
}

impl LoadableFromFile for Passports {
    fn load(filename: &str) -> Passports {
        let re: Regex = Regex::new("([a-z]+):([a-z0-9#]+)").unwrap();
        let file = File::open(filename).expect("Invalid filename");

        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut passports: Vec<Passport> = Vec::new();
        let mut passports_with_invalid_fields: Vec<Passport> = Vec::new();
        let mut current = Passport::default();
        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    // This either means we hit the EOF or a Passport separator.
                    if line.trim().is_empty() {
                        if current.is_valid() {
                            passports.push(current);
                        } else if current.was_initialized() {
                            passports_with_invalid_fields.push(current);
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
                            "byr" => current.birth_year = found[2].parse().unwrap_or(-1),
                            "iyr" => current.issue_year = found[2].parse().unwrap_or(-1),
                            "eyr" => current.expiration_year = found[2].parse().unwrap_or(-1),
                            "hgt" => {
                                current.height =
                                    Height::from_str(&found[2]).unwrap_or(Height::Invalid)
                            }
                            "hcl" => {
                                current.hair_color =
                                    RgbColor::from_str(&found[2]).unwrap_or(RgbColor {
                                        red: 0,
                                        green: 0,
                                        blue: -1,
                                    })
                            }
                            "ecl" => {
                                current.eye_color =
                                    EyeColor::from_str(&found[2]).unwrap_or(EyeColor::Invalid)
                            }
                            "pid" => {
                                current.id = Identifier::from_str(&found[2])
                                    .unwrap_or(Identifier { value: -1 })
                            }
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
        Passports {
            contains_invalid_fields: passports_with_invalid_fields,
            valid: passports,
        }
    }
}

fn part_one(passports: &Passports) -> i64 {
    passports.contains_invalid_fields.len() as i64 + passports.valid.len() as i64
}

fn part_two(passports: &Passports) -> i64 {
    passports.valid.len() as i64
}

pub fn solve() -> String {
    let passports = Passports::load("input/day_four.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&passports),
        part_two(&passports)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one_example() {
        let passports = Passports::load("input/day_four_part_one_example.txt");
        assert_eq!(2, part_one(&passports));
    }

    #[test]
    fn solves() {
        assert_eq!("part one: 176, part two: 137", solve());
    }
}
