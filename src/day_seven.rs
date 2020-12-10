//use crate::loadable::LoadableFromFile;
//use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use lazy_static::lazy_static;
use regex::Regex;

pub fn load(filename: &str) {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"((\d+) ([a-z ]+)) bags?[., ]+").unwrap();
    }
    let file = File::open(filename).expect("invalid filename");
    let reader = BufReader::new(file);

    for line in reader.lines().map(|l| l.expect("bad input")) {
        let name_or: Vec<&str> = line.splitn(2, " bags contain ").collect();

        println!("'{}':", name_or[0]);
        if name_or[1].starts_with("n") { // ...o other bags.
            // bag is a terminal node, we don't care.
            continue;
        } else {
            for child in RE.captures_iter(&name_or[1]) {
                println!("\tchild: {} of {}", &child[2], &child[3]);
            }
        }
    }
}

pub fn solve() -> String {
  format!(
      "part one: {}, part two: {}", 0, 0
  )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }

    #[test]
    pub fn test_loader() {
        load("input/day_seven_example.txt");
        panic!();
    }
}