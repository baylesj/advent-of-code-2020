use crate::loadable::LoadableFromFile;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

impl LoadableFromFile for Vec<i64> {
    fn load(filename: &str) -> Vec<i64> {
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| i64::from_str(&l.unwrap()).unwrap())
            .collect()
    }
}

pub fn part_one(numbers: &[i64], preamble_length: usize) -> i64 {
    let mut set: HashSet<i64> = HashSet::new();
    set.extend(numbers[0..preamble_length].into_iter());

    for ei in numbers.iter().skip(preamble_length).enumerate() {
        let mut has_sum = false;
        for s in &set {
            let complement = ei.1 - s;
            if set.contains(&complement) {
                has_sum = true;
                break;
            }
        }
        if !has_sum {
            return *ei.1;
        }
        // Enumerate's first result is always zero index, so after skipping
        // 0 is actually 5.
        set.remove(&numbers[ei.0]);
        set.insert(*ei.1);
    }
    1
}

pub fn part_two(numbers: &[i64], weakness_number: i64) -> i64 {
    let mut current_sum = numbers[0];
    let mut left_index = 0;
    let mut right_index = 0;

    while current_sum != weakness_number {
        if current_sum > weakness_number {
            current_sum -= numbers[left_index];
            left_index += 1;
        } else if current_sum < weakness_number {
            right_index += 1;
            current_sum += numbers[right_index];
        }
    }

    let mut min = numbers[left_index];
    let mut max = min;
    for i in left_index + 1..right_index + 1 {
        min = std::cmp::min(min, numbers[i]);
        max = std::cmp::max(max, numbers[i]);
    }
    min + max
}

pub fn solve() -> String {
    let numbers = Vec::<i64>::load("input/day_nine.txt");
    let weakness_number = part_one(&numbers, 25);
    format!(
        "part one: {}, part two: {}",
        weakness_number,
        part_two(&numbers, weakness_number)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 530627549, part two: 77730285", solve());
    }

    #[test]
    pub fn test_example() {
        let numbers = Vec::<i64>::load("input/day_nine_example.txt");
        assert_eq!(127, part_one(&numbers, 5));
    }

    #[test]
    pub fn test_example_part_two() {
        let numbers = Vec::<i64>::load("input/day_nine_example.txt");
        let weakness_number = part_one(&numbers, 5);

        assert_eq!(62, part_two(&numbers, weakness_number));
    }
}
