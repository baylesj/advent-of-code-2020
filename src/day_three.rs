use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::*;

const INPUT_FILENAME: &'static str = "input/day_three.txt";

fn part_one(map: &Matrix2D<char>, slope: &Point2D) -> i64 {
    let mut tree_count: i64 = 0;
    let mut current = Point2D { x: 0, y: 0 };
    while current.y < map.size.y {
        // The map is infinite in a repeating pattern, but only in the X direction.
        current.x = current.x % map.size.x;
        if map.get(&current) == '#' {
            tree_count += 1;
        }
        current += *slope;
    }
    tree_count
}

fn part_two(map: &Matrix2D<char>) -> i64 {
    static SLOPES: &'static [Point2D] = &[
        Point2D { x: 1, y: 1 },
        Point2D { x: 3, y: 1 },
        Point2D { x: 5, y: 1 },
        Point2D { x: 7, y: 1 },
        Point2D { x: 1, y: 2 },
    ];

    let mut total: i64 = 1;
    for slope in SLOPES {
        total *= part_one(map, slope);
    }
    total
}

pub fn solve() -> String {
    let map = Matrix2D::<char>::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        part_one(&map, &Point2D { x: 3, y: 1 }),
        part_two(&map)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solves_part_one_example() {
        const SLOPE: Point2D = Point2D { x: 3, y: 1 };
        let map = Matrix2D::<char>::load("input/day_three_part_one_example.txt");
        assert_eq!(7, part_one(&map, &SLOPE));
    }

    #[test]
    pub fn solves() {
        assert_eq!("part one: 176, part two: 5872458240", solve());
    }
}
