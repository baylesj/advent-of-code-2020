use crate::loadable::LoadableFromFile;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Rule {
    Single(char),
    Referential(Vec<Vec<usize>>),
}

#[derive(Debug, PartialEq)]
struct RuleAndIndex {
    index: usize,
    rule: Rule,
}

#[derive(Debug, Default)]
struct Messages {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
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
            messages: lines.map(|l| l.unwrap()).collect(),
        }
    }
}

fn build_regex(
    r: usize,
    rules: &HashMap<usize, Rule>,
    memo: &mut HashMap<usize, String>,
) -> String {
    if let Some(value) = memo.get(&r) {
        return value.to_string();
    }

    let fragment;
    match rules.get(&r).unwrap() {
        // Base case, string should be of length 1.
        Rule::Single(c) => fragment = c.to_string(),

        Rule::Referential(rs) => {
            let mut regex = String::from("(");
            for rules_list in rs.iter().enumerate() {
                if rules_list.0 > 0 {
                    regex.push('|');
                }

                regex.push('(');
                for rule in rules_list.1.iter().enumerate() {
                    regex += &build_regex(*rule.1, rules, memo);
                }
                regex.push(')');
            }
            regex.push(')');
            fragment = regex;
        }
    }

    memo.insert(r, fragment);
    memo.get(&r).unwrap().to_string()
}

fn part_one(messages: &Messages) -> i64 {
    let mut raw_regex = String::from("^");
    let mut memo = HashMap::new();
    raw_regex += &build_regex(0, &messages.rules, &mut memo);
    raw_regex += "$";
    let re = Regex::from_str(&raw_regex).unwrap();

    messages.messages.iter().filter(|m| re.is_match(m)).count() as i64
}

pub fn solve() -> String {
    let mut messages = Messages::load("input/day_nineteen.txt");
    let part_one_answer = part_one(&messages);
    messages
        .rules
        .insert(8, Rule::Referential(vec![vec![42], vec![42, 8]]));
    messages
        .rules
        .insert(11, Rule::Referential(vec![vec![42, 31], vec![42, 11, 31]]));

    format!(
        "part one: {}, part two: {}",
        part_one_answer,
        0 // TODO: fix stack overflow: part_one(&messages)
    )
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
        // NOTE: the example is modified since the actual file starts with a
        // pair for rule 0, and there's really no reason to complicate things.
        assert_eq!(Rule::Referential(vec![vec![1, 5]]), messages.rules[&0]);
        assert_eq!(Rule::Single('a'), messages.rules[&4]);

        assert_eq!("babbb", messages.messages[0]);
    }

    #[test]
    fn test_example() {
        let messages = Messages::load("input/day_nineteen_example.txt");
        assert_eq!(2, part_one(&messages));
    }

    #[test]
    fn test_solve() {
        assert_eq!("part one: 213, part two: 0", solve());
    }
}
