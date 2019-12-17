use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input/day_fourteen.txt";

#[derive(Debug, Default)]
struct ReactionFactor {
    chemical: String,
    quantity: i64,
}

#[derive(Debug, Default)]
struct Reaction {
    inputs: Vec<ReactionFactor>,
    // TODO: multiple outputs?
    output: ReactionFactor,
}

fn produce_reaction_list(input_filename: &str) -> Vec<Reaction> {
    let file = File::open(input_filename).expect("Invalid filename");
    let reader = BufReader::new(file);
    let reactions = Vec::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(([0-9]+) ([a-zA-Z]+))").unwrap();
    }

    for line in reader.lines() {
        let mut reaction = Reaction::default();
        let l = line.expect("line should be valid");
        let captures = RE.captures(&l).unwrap();
        for i in 0..captures.len() - 1 {}
    }

    reactions
}

pub fn part_one() -> i64 {
    produce_reaction_list(INPUT_FILENAME);
    128
}

pub fn solve() {
    println!("Day fourteen, part one: {}", part_one());
}
