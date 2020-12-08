use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_fourteen.txt";

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

        reaction.chemical = ts[ts_last].0.clone();
        reaction.quantity = ts[ts_last].1;
        reactions.insert(reaction.chemical.to_string(), reaction);
    }

    reactions
}

fn reduce_to_ore_to_fuel(reactions: &mut HashMap<String, Reaction>, amount: i64) -> i64 {
    // Reduced reactions contains a list of fully reduced reactants, meaning
    //
    let mut total_ore_usage: i64 = 0;
    let mut current_reactions: Vec<Reactant> = vec![("FUEL".to_string(), amount)];
    let mut left_overs: HashMap<String, i64> = HashMap::new();

    while current_reactions.len() > 0 {
        let current: Vec<Reactant> = current_reactions;
        // TODO: presize?
        current_reactions = Vec::new();
        for reaction in current {
            let mut current_value;
            let mut value_count;
            if reaction.0 != "ORE" {
                let rq = reactions[&reaction.0].quantity;
                if left_overs.contains_key(&reaction.0) {
                    current_value = left_overs[&reaction.0];
                    value_count = 0;
                } else {
                    current_value = rq;
                    value_count = 1;
                }

                // LOL @ massive speed up from using while loop here.
                if current_value < reaction.1 {
                    let d = ((reaction.1 as f64 - current_value as f64) / rq as f64).ceil() as i64;
                    current_value += rq * d;
                    value_count += d;
                }

                left_overs.insert(reaction.0.to_string(), current_value - reaction.1);
                for reactant in &reactions[&reaction.0].reactants {
                    current_reactions.push((reactant.0.to_string(), reactant.1 * value_count))
                }
            } else {
                total_ore_usage += reaction.1;
            }
        }
    }
    total_ore_usage
}

pub fn part_one(input_filename: &str) -> i64 {
    let mut reactions = parse_reactions(input_filename);
    reduce_to_ore_to_fuel(&mut reactions, 1)
}

pub fn part_two(input_filename: &str, goal: i64) -> i64 {
    let reactions = parse_reactions(input_filename);

    let mut left = 1;
    let mut right = 10000000;
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
    }

    mid
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME, 1000000000000)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_sample_one() {
        assert_eq!(165, part_one("input/day_fourteen_sample_one.txt"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(202617, part_one("input/day_fourteen.txt"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(7863863, part_two("input/day_fourteen.txt", 1000000000000));
    }
}
