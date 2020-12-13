pub fn part_one() -> i64 {
    0
}

pub fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    format!("part one: {}, part two: {}", part_one(), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }
}
