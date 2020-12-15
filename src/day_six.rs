use crate::loadable::LoadableFromFile;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_six.txt";

type Group = Vec<HashSet<char>>;

impl LoadableFromFile for Vec<Group> {
    fn load(filename: &str) -> Vec<Group> {
        let file = File::open(filename).expect("Invalid filename");
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut groups: Vec<Group> = Vec::new();
        let mut group = Group::default();
        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    // This either means we hit the EOF or a Group separator.
                    line = line.trim().to_string();
                    if line.is_empty() {
                        groups.push(group);
                        if bytes_read == 0 {
                            break;
                        } else {
                            group = Group::default();
                            continue;
                        }
                    }

                    let mut current = HashSet::<char>::default();
                    for byte in line.as_bytes() {
                        current.insert(*byte as char);
                    }
                    group.push(current);
                }
                Err(err) => {
                    panic!(err);
                }
            }
            line.clear();
        }
        groups
    }
}

fn union(group: &Group) -> HashSet<char> {
    let mut out: HashSet<char> = group.first().unwrap().clone();
    for member in group.iter().skip(1) {
        out = out.union(&member).copied().collect();
    }
    out
}

fn intersection(group: &Group) -> HashSet<char> {
    let mut out: HashSet<char> = group.first().unwrap().clone();
    for member in group.iter().skip(1) {
        out = out.intersection(&member).copied().collect();
    }
    out
}

fn part_one(groups: &[Group]) -> usize {
    let mut count: usize = 0;
    for group in groups {
        count += union(group).len();
    }
    count
}

fn part_two(groups: &[Group]) -> usize {
    let mut count: usize = 0;
    for group in groups {
        count += intersection(group).len();
    }
    count
}

pub fn solve() -> String {
    let groups = Vec::<Group>::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        part_one(&groups),
        part_two(&groups)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 6549, part two: 3466", solve());
    }

    #[test]
    fn test_part_one_example() {
        let groups = Vec::<Group>::load("input/day_six_example.txt");
        assert_eq!(11, part_one(&groups));
    }

    #[test]
    fn test_part_two_example() {
        let groups = Vec::<Group>::load("input/day_six_example.txt");
        assert_eq!(6, part_two(&groups));
    }
}
