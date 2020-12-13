use crate::loadable::LoadableFromFile;

pub fn part_one(input: &[i64]) -> i64 {
  let mut one_diff = 0;
  // Your built in adapter always has a difference of three.
  let mut three_diff = 1;

  let mut last = 0;
  for i in input {
    let diff = *i - last;
    match diff {
      1 => one_diff += 1,
      3 => three_diff += 1,
      _ => ()
    }
    last = *i;
  }

  one_diff * three_diff
}

pub fn part_two(_input: &[i64]) -> i64 {
  0
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
    pub fn test_solve() {
        assert_eq!("part one: 2048, part two: 0", solve());
    }

    #[test]
    pub fn test_example() {
        let mut input = Vec::<i64>::load("input/day_ten_example.txt");
        input.sort();
        assert_eq!(7 * 5, part_one(&input));
    }

    #[test]
    pub fn test_example_two() {
        let mut input = Vec::<i64>::load("input/day_ten_example_two.txt");
        input.sort();
        assert_eq!(22 * 10, part_one(&input));
    }
}
