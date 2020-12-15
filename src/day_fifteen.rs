use crate::loadable::LoadableFromFile;
use std::collections::HashMap;

fn part_one(starting_numbers: &[i64], limit: usize) -> i64 {
    println!("starting numbers are: {:?}", starting_numbers);
    let mut last_log = (0, 0);
    let mut last_spoken = 0;
    let mut spoken_log = HashMap::<i64, (i64, i64)>::new();
    for i in 0..starting_numbers.len() {
        last_spoken = starting_numbers[i];
        last_log = (i as i64, -1);
        spoken_log.insert(last_spoken, last_log.clone());
    }

    for i in starting_numbers.len()..limit {
        if last_log.1 == -1 {
            last_spoken = 0;
        } else {
            last_spoken = last_log.0 - last_log.1;
        }
        last_log = (i as i64, spoken_log.get(&last_spoken).unwrap_or(&(-1, 0)).0);
        spoken_log.insert(last_spoken, last_log);
    }
    last_spoken
}

pub fn solve() -> String {
    let numbers = Vec::<i64>::load("input/day_fifteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&numbers, 2020),
        // TODO: part two trick is performance. This naive approach takes
        // 30 seconds, which is approximately 300 times too slow.
        part_one(&numbers, 30000000)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 203, part two: 9007186", solve());
    }

    #[test]
    fn test_example() {
        let numbers = Vec::<i64>::load("input/day_fifteen_example.txt");
        assert_eq!(436, part_one(&numbers, 2020));
    }
}
