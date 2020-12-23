use crate::loadable::LoadableFromFile;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Rule {
    Single(char),
    Referential(Vec<(usize, usize)>),
}

#[derive(Debug, PartialEq)]
struct RuleAndIndex {
    index: usize,
    rule: Rule,
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
                    .map(|p: Vec<&str>| (p[0].parse().unwrap(), p[1].parse().unwrap()))
                    .collect(),
            );
        }
        Ok(RuleAndIndex {
            index: index,
            rule: rule,
        })
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
            RuleAndIndex::from_str("36: \"b\"").unwrap(),
            RuleAndIndex {
                index: 36,
                rule: Rule::Single('b')
            }
        );
        assert_eq!(
            RuleAndIndex::from_str("6: 54 116").unwrap(),
            RuleAndIndex {
                index: 6,
                rule: Rule::Referential(vec![(54, 116)])
            }
        );
        assert_eq!(
            RuleAndIndex::from_str("93: 99 100 | 36 8").unwrap(),
            RuleAndIndex {
                index: 93,
                rule: Rule::Referential(vec![(99, 100), (36, 8)])
            }
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }
}
