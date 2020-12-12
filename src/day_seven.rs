use crate::loadable::LoadableFromFile;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const SPECIAL_BAG_NAME: &'static str = "shiny gold";

// Stage 1: lazy string lookups everywhere.
#[derive(Debug, Clone)]
pub struct Bag {
    name: String,
    children: HashMap<String, i32>,
}

// TODO: not thrilled about all the Strings everywhere, but need to do more
// research on lifetimes.
impl LoadableFromFile for HashMap<String, Bag> {
    fn load(filename: &str) -> HashMap<String, Bag> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"((\d+) ([a-z ]+)) bags?[., ]+").unwrap();
        }
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);

        let mut bags = HashMap::<String, Bag>::new();
        for line in reader.lines().map(|l| l.expect("bad input")) {
            let name_or: Vec<&str> = line.splitn(2, " bags contain ").collect();

            let mut bag = Bag {
                name: name_or[0].to_owned(),
                children: HashMap::<String, i32>::new(),
            };
            if !name_or[1].starts_with("n") {
                for child in RE.captures_iter(&name_or[1]) {
                    let count: i32 = child[2].parse().unwrap();
                    bag.children.insert(child[3].to_string(), count);
                }
            }
            bags.insert(bag.name.to_string(), bag);
        }

        bags
    }
}

pub fn find_bags_that_can_hold_gold(bags: &HashMap<String, Bag>) -> i64 {
    let mut unknown: Vec<Bag> = bags.values().cloned().collect();
    let mut can_carry = HashSet::new();
    can_carry.insert(SPECIAL_BAG_NAME.to_owned());

    // TODO: improve algorithm here, it's kind of slow (448ms).
    loop {
        let mut next_round = vec![];
        unknown.retain(|bag| {
            for bag_name in &can_carry {
                if bag.children.contains_key(&bag_name[..]) {
                    next_round.push(bag.name.to_owned());
                    return false;
                }
            }
            true
        });

        if next_round.len() == 0 {
            break;
        }
        can_carry.extend(next_round.into_iter());
    }

    // We don't actually count the gold bag here.
    can_carry.len() as i64 - 1
}

pub fn find_total_bag_count(
    name: &str,
    bags: &HashMap<String, Bag>,
    memo: &mut HashMap<String, i32>,
) -> i32 {
    if memo.contains_key(name) {
        return memo[name];
    }
    let bag = &bags[name];
    if bag.children.len() == 0 {
        memo.insert(bag.name.to_string(), 1);
        return 1;
    }

    let child_count: i32 = bag
        .children
        .iter()
        .map(|child| child.1 * find_total_bag_count(child.0, bags, memo))
        .sum();
    memo.insert(name.to_string(), child_count + 1);
    child_count + 1
}

pub fn find_total_bag_count_in_gold(bags: &HashMap<String, Bag>) -> i32 {
    let mut memo = HashMap::new();

    // Minus one because we don't contain the gold bag in the answer.
    find_total_bag_count(SPECIAL_BAG_NAME, bags, &mut memo) - 1
}

pub fn solve() -> String {
    let bags = HashMap::<String, Bag>::load("input/day_seven.txt");
    format!(
        "part one: {}, part two: {}",
        find_bags_that_can_hold_gold(&bags),
        find_total_bag_count_in_gold(&bags)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 222, part two: 13264", solve());
    }

    #[test]
    pub fn test_example_one() {
        let bags = HashMap::<String, Bag>::load("input/day_seven_example.txt");
        assert_eq!(4, find_bags_that_can_hold_gold(&bags));
    }

    #[test]
    pub fn test_example_two() {
        let bags = HashMap::<String, Bag>::load("input/day_seven_example_two.txt");
        assert_eq!(32, find_total_bag_count_in_gold(&bags));
    }

    #[test]
    pub fn test_example_three() {
        let bags = HashMap::<String, Bag>::load("input/day_seven_example_three.txt");
        assert_eq!(126, find_total_bag_count_in_gold(&bags));
    }
}
