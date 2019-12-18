use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// TODO: downloading MKL just for solving a linear set of equations is dumb.
use intel_mkl_src;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

const INPUT_FILENAME: &str = "input/day_fourteen.txt";

type Reaction = HashMap<String, i64>;

fn parse_reactions(input_filename: &str) -> Vec<Reaction> {
    let file = File::open(input_filename).expect("Invalid filename");
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"((?P<quantity>[0-9]+) (?P<chemical>[a-zA-Z]+))").unwrap();
    }

    let mut reactions = Vec::new();
    for line in reader.lines() {
        let mut reaction = Reaction::default();
        let l = line.expect("line should be valid");

        type Match = (String, i64);
        // TODO: avoid copy?
        let ts: Vec<Match> = RE
            .captures_iter(&l)
            .map(|c| {
                (
                    c["chemical"].to_string(),
                    c["quantity"].parse().expect("quantity is a number"),
                )
            })
            .collect();

        let ts_last = ts.len() - 1;
        for item in 0..ts_last {
            reaction.insert(ts[item].0.clone(), ts[item].1);
        }
        // We can move to ax + by + ... + cz = 0 form by "subtracting" the output.
        reaction.insert(ts[ts_last].0.clone(), -1 * ts[ts_last].1);
        reactions.push(reaction);
    }

    reactions
}

fn reduce_to_ore_to_fuel(reactions: Vec<Reaction>) -> i64 {
    let all_keys: Vec<String> = reactions.iter().flat_map(|r| r.keys()).cloned().collect();

    let num_rows = reactions.len() + 1;
    let num_cols = all_keys.len();
    let mut system = Array::from_elem((num_rows, num_cols), 0.);
    for row in 0..num_rows - 1 {
        for col in 0..num_cols {
            let entry = system.get_mut((row, col)).unwrap();
            *entry = *reactions[row].get(&all_keys[col]).unwrap_or(&0) as f64;
        }
    }

    let mut ore_idx: usize = 0;
    let mut fuel_idx: usize = 0;
    for i in 0..num_cols {
        if all_keys[i] == "ORE" {
            ore_idx = i;
        } else if all_keys[i] == "FUEL" {
            fuel_idx = i;
        }
    }
    let last_entry = system.get_mut((num_rows - 1, ore_idx)).unwrap();
    *last_entry = 1.;
    let mut b: Array1<f64> = Array::from_elem(num_rows, 0.);
    b[num_rows - 1] = 1.;
    println!("all_keys: {:#?}", all_keys);

    println!("system: {:#?}", system);
    println!("b: {:#?}", b);
    let solution = system.solve_into(b).unwrap();

    solution[fuel_idx as usize] as i64
}

pub fn part_one(input_filename: &str) -> i64 {
    let reactions = parse_reactions(input_filename);
    //println!("Reaction list: {:#?}", reactions);

    reduce_to_ore_to_fuel(reactions)
}

pub fn solve() {
    println!("Day fourteen, part one: {}", part_one(INPUT_FILENAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
        assert_eq!(165, part_one("input/day_fourteen_sample_one.txt"))
    }
}
