use crate::loadable::LoadableFromFile;

// The maximum spoken number was found experimentally by running
// part two and checking the largest spoken number.  In production
// this would probably be better as i32::MAX or similar.
const MAXIMUM_SPOKEN_NUMBER: usize = 29473358;

// NOTE: use of i32 here, while annoying, causes 1/2 of memory to be used
// and a subsequence ~250ms speedup in release.
fn part_one(starting_numbers: &[i64], term: usize, cache: &mut Vec<i32>) -> i64 {
    for i in 0..starting_numbers.len() {
        cache[starting_numbers[i] as usize] = i as i32;
    }
    let mut last_spoken = 0;
    // The prompt is 1-indexed, but we are 0-indexed.
    for i in starting_numbers.len()..term - 1 {
        let last = cache[last_spoken as usize];
        let been_spoken_before = last != -1;
        cache[last_spoken as usize] = i as i32;
        last_spoken = if been_spoken_before {
            i as i32 - last
        } else {
            0
        };
    }
    last_spoken as i64
}

fn create_cache() -> Vec<i32> {
    vec![-1; MAXIMUM_SPOKEN_NUMBER + 1]
}

pub fn solve() -> String {
    let mut cache = create_cache();
    let numbers = Vec::<i64>::load("input/day_fifteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&numbers, 2020, &mut cache.clone()),
        // Turns out the sequence has no cycles or patterns in variances after manual
        // inspection. Some digging shows that this sequence is actually Van Eck's,
        // which does not have a closed form. Performance comes from optimizations,
        // not reducing algorithmic complexity.
        part_one(&numbers, 30000000, &mut cache)
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
        let mut cache = create_cache();
        assert_eq!(436, part_one(&numbers, 2020, &mut cache));
    }
}
