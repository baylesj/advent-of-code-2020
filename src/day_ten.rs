use crate::loadable::LoadableFromFile;

fn part_one(input: &[i64]) -> i64 {
    let mut one_diff = 0;
    // Your built in adapter always has a difference of three.
    let mut three_diff = 1;

    let mut last = 0;
    for i in input {
        let diff = *i - last;
        match diff {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => (),
        }
        last = *i;
    }

    one_diff * three_diff
}

fn find_arrangements(input: &[i64], index: usize, memo: &mut Vec<Option<i64>>) -> i64 {
    if index == input.len() - 1 {
        return 1;
    }

    if memo[index].is_some() {
        return memo[index].unwrap();
    }

    let mut total_arrangements = 0;
    let mut next_index = index + 1;
    while next_index < input.len() && input[next_index] - input[index] <= 3 {
        total_arrangements += find_arrangements(input, next_index, memo);
        next_index += 1
    }

    memo[index] = Some(total_arrangements);
    total_arrangements
}

fn part_two(input: &[i64]) -> i64 {
    let mut memo = vec![None; input.len()];
    let mut sum = 0;
    let mut index = 0;
    while input[index] <= 3 {
        sum += find_arrangements(input, index, &mut memo);
        index += 1;
    }
    sum
}

pub fn solve() -> String {
    let mut input = Vec::<i64>::load("input/day_ten.txt");
    input.sort();
    format!(
        "part one: {}, part two: {}",
        part_one(&input),
        part_two(&input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 2048, part two: 1322306994176", solve());
    }

    #[test]
    fn test_example() {
        let mut input = Vec::<i64>::load("input/day_ten_example.txt");
        input.sort();
        assert_eq!(7 * 5, part_one(&input));
        assert_eq!(8, part_two(&input));
    }

    #[test]
    fn test_example_two() {
        let mut input = Vec::<i64>::load("input/day_ten_example_two.txt");
        input.sort();
        assert_eq!(22 * 10, part_one(&input));
        assert_eq!(19208, part_two(&input));
    }
}
