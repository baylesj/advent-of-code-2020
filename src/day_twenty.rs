use crate::loadable::LoadableFromFile;

struct Tile {
    id: i32,
    edges: [i32; 4],
}

impl LoadableFromFile for Vec<Tile> {
    fn load(filename: &str) -> Self {
        vec![]
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
    fn test_solve() {
        assert_eq!("part one: 0, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        // input/day_twenty_example.txt
        // 1951 * 3079 * 2971 * 1171 = 20899048083289
    }
}
