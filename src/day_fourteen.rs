use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

const INPUT_FILENAME: &str = "input/day_fourteen.txt";

type Reactant = (String, i64);

#[derive(Debug, Default, Clone)]
struct Reaction {
    chemical: String,
    quantity: i64,
    reactants: Vec<Reactant>,
}

fn parse_reactions(input_filename: &str) -> HashMap<String, Reaction> {
    let file = File::open(input_filename).expect("Invalid filename");
    let reader = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"((?P<quantity>[0-9]+) (?P<chemical>[a-zA-Z]+))").unwrap();
    }

    let mut reactions = HashMap::new();
    for line in reader.lines() {
        let mut reaction = Reaction::default();
        let l = line.expect("line should be valid");

        // TODO: avoid copy?
        let ts: Vec<Reactant> = RE
            .captures_iter(&l)
            .map(|c| {
                (
                    c["chemical"].to_string(),
                    c["quantity"].parse().expect("quantity is a number"),
                )
            })
            .collect();

        let ts_last = ts.len() - 1;
        for i in 0..ts_last {
            reaction.reactants.push(ts[i].clone());
        }

        // We can move to ax + by + ... + cz = 0 form by "subtracting" the output.
        reaction.chemical = ts[ts_last].0.clone();
        reaction.quantity = ts[ts_last].1;
        reactions.insert(reaction.chemical.to_string(), reaction);
    }

    reactions
}

// TODO: perf is terrible.
fn reduce_to_ore_to_fuel(reactions: &mut HashMap<String, Reaction>, amount: i64) -> i64 {
    // start with FUEL, not with ORE
    let mut reduced_reactions = Vec::new();
    let mut current_reactions: Vec<Reactant> = vec![("FUEL".to_string(), amount)];
    let mut left_overs: HashMap<String, i64> = HashMap::new();

    while current_reactions.len() > 0 {
        let tmp_reactions = current_reactions.clone();
        current_reactions.clear();
        for reaction in tmp_reactions {
            let mut current_value;
            let mut value_count;
            if reaction.0 != "ORE" {
                if left_overs.contains_key(&reaction.0) {
                    current_value = left_overs[&reaction.0];
                    value_count = 0;
                } else {
                    current_value = reactions[&reaction.0].quantity;
                    value_count = 1;
                }

                while current_value < reaction.1 {
                    current_value += reactions[&reaction.0].quantity;
                    value_count += 1;
                }

                left_overs.insert(reaction.0.to_string(), current_value - reaction.1);
                for reactant in &reactions[&reaction.0].reactants {
                    current_reactions.push((reactant.0.to_string(), reactant.1 * value_count))
                }
            } else {
                reduced_reactions.push(reaction.clone());
            }
        }
    }

    let mut total_ore = 0;
    for reaction in reduced_reactions {
        total_ore += reaction.1;
    }
    total_ore
}

pub fn part_one(input_filename: &str) -> i64 {
    let mut reactions = parse_reactions(input_filename);
    reduce_to_ore_to_fuel(&mut reactions, 1)
}

pub fn part_two(input_filename: &str, goal: i64) -> i64 {
    let reactions = parse_reactions(input_filename);

    let mut left = 4935420; // guess taken from part one.
    let mut right = 5500000; // random guess.
    let mut mid = (left + right) / 2;
    while left != mid && mid != right {
        let mid_result = reduce_to_ore_to_fuel(&mut reactions.clone(), mid);
        if mid_result == goal {
            return mid;
        } else if mid_result < goal {
            left = mid;
        } else {
            right = mid;
        }

        mid = (left + right) / 2;
        println!(
            "left: {}, mid: {}, right: {}, res: {}",
            left, mid, right, mid_result
        );
    }

    mid
}

pub fn solve() {
    let now = Instant::now();
    println!(
        "Day fourteen, part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME, 1000000000000)
    );
    println!("Time elapsed: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_sample_one() {
        assert_eq!(165, part_one("input/day_fourteen_sample_one.txt"))
    }
}
