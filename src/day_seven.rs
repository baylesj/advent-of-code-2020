//use crate::loadable::LoadableFromFile;
//use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Stage 1: lazy string lookups everywhere.
#[derive(Debug, Clone)]
pub struct Bag {
    name: String,
    children: HashMap<String, i32>,
}

pub fn load(filename: &str) -> Vec<Bag> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\d+) ([a-z ]+)) bags?[., ]+").unwrap();
    }
    let file = File::open(filename).expect("invalid filename");
    let reader = BufReader::new(file);

    let mut bags = Vec::<Bag>::new();
    for line in reader.lines().map(|l| l.expect("bad input")) {
        let name_or: Vec<&str> = line.splitn(2, " bags contain ").collect();

        if name_or[1].starts_with("n") {
            // ...o other bags.
            // bag is a terminal node, we don't care.
            continue;
        } else {
            let mut bag = Bag {
                name: name_or[0].to_owned(),
                children: HashMap::<String, i32>::new(),
            };
            for child in RE.captures_iter(&name_or[1]) {
                let count: i32 = child[2].parse().unwrap();
                bag.children.insert(child[3].to_owned(), count);
            }
            bags.push(bag);
        }
    }

    bags
}

pub fn find_bags_that_can_hold_gold(bags: &[Bag]) -> i64 {
    let mut unknown = bags.to_vec();
    let mut can_carry = HashSet::new();
    can_carry.insert("shiny gold".to_owned());

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

pub fn solve() -> String {
    let bags = load("input/day_seven.txt");
    let part_one_answer = find_bags_that_can_hold_gold(&bags);
    format!("part one: {}, part two: {}", part_one_answer, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 222, part two: 0", solve());
    }

    #[test]
    pub fn test_loader() {
        let bags = load("input/day_seven_example.txt");
        println!("bags: {:?}", &bags);
        assert_eq!(4, find_bags_that_can_hold_gold(&bags));
    }
}
