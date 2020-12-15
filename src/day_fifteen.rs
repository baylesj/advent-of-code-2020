use crate::loadable::LoadableFromFile;

// The maximum spoken number was found experimentally by running
// part two and checking the largest spoken number.  In production
// this would probably be better as i32::MAX or similar.
const MAXIMUM_SPOKEN_NUMBER: usize = 29473358;

fn part_one(starting_numbers: &[i64], limit: usize) -> i64 {
    let mut last_spoken_cache = vec![-1; MAXIMUM_SPOKEN_NUMBER + 1];
    for i in 0..starting_numbers.len() {
        last_spoken_cache[starting_numbers[i] as usize] = i as i64;
    }

    let mut last_spoken = 0;
    for i in starting_numbers.len()..limit - 1 {
        let last = last_spoken_cache[last_spoken as usize];
        let been_spoken_before = last != -1;
        last_spoken_cache[last_spoken as usize] = i as i64;
        last_spoken = if been_spoken_before {
            i as i64 - last
        } else {
            0
        };
    }
    last_spoken
}

pub fn solve() -> String {
    let numbers = Vec::<i64>::load("input/day_fifteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&numbers, 2020),
        // Turns out the sequence has no cycles or patterns in variances after manual
        // inspection. Some digging shows that this sequence is actually Van Eck's,
        // which does not have a closed form. Performance comes from optimizations,
        // not reducing algorithmic complexity.
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
