use crate::loadable::LoadableFromFile;

fn part_one(starting_numbers: &[i64]) -> i64 {
    println!("starting numbers are: {:?}", starting_numbers);
    0
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let numbers = Vec::<i64>::load("input/day_fifteen.txt");
    format!("part one: {}, part two: {}", part_one(&numbers), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }
}
