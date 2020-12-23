use crate::loadable::LoadableFromFile;
use std::str::FromStr;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
enum Rule {
    None,
    Single(char),
    Referential(Vec<Vec<usize>>)
}

#[derive(Debug, PartialEq)]
struct RuleAndIndex {
    index: usize,
    rule: Rule,
}

#[derive(Debug, Default)]
struct Messages {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>
}

impl FromStr for RuleAndIndex {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index_and_rest = s.split(": ");
        let index: usize = index_and_rest.next().unwrap().parse().unwrap();
        let rest = index_and_rest.next().unwrap();

        let rule;
        if rest.starts_with("\"") {
            if rest.len() != 3 {
                return Err("invalid rule");
            }
            rule = Rule::Single(rest.as_bytes()[1] as char);
        } else {
            rule = Rule::Referential(
                rest.split(" | ")
                    .map(|p| p.split_whitespace().collect())
                    .map(|p: Vec<&str>| p.iter().map(|r| usize::from_str(r).unwrap()).collect())
                    .collect(),
            );
        }
        Ok(RuleAndIndex {
            index: index,
            rule: rule,
        })
    }
}

impl LoadableFromFile for Messages {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let mut rules = HashMap::new();
        loop {
            match lines.next().unwrap().unwrap().trim() {
                "" => break,
                s => {
                    let r = RuleAndIndex::from_str(s).unwrap();
                    rules.insert(r.index, r.rule);
                }
            }
        }
        Messages {
            rules: rules,
            messages: lines.map(|l| l.unwrap()).collect()
        }
    }
}

fn part_one() -> i64 {
    0
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            RuleAndIndex {
                index: 36,
                rule: Rule::Single('b')
            },
            RuleAndIndex::from_str("36: \"b\"").unwrap(),
        );
        assert_eq!(
            RuleAndIndex {
                index: 6,
                rule: Rule::Referential(vec![vec![54, 116]])
            },
            RuleAndIndex::from_str("6: 54 116").unwrap(),
        );
        assert_eq!(
            RuleAndIndex {
                index: 93,
                rule: Rule::Referential(vec![vec![99, 100], vec![36, 8]])
            },
            RuleAndIndex::from_str("93: 99 100 | 36 8").unwrap(),
        );
    }

    #[test]
    fn test_load() {
        let messages = Messages::load("input/day_nineteen_example.txt");
        assert_eq!(Rule::Referential(vec![vec![4, 1, 5]]),
        messages.rules[&0]);
        assert_eq!(Rule::Single('a'),
        messages.rules[&4]);

        assert_eq!("ababbb", messages.messages[0]);
    }

    #[test]
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }
}
