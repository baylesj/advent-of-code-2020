use crate::loadable::LoadableFromFile;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::{Matrix2D, Matrix2DLike, Point2D};

const INPUT_FILENAME: &'static str = "input/day_three.txt";

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Square {
    Blocked,
    Free,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if *self == Square::Blocked { '#' } else { ' ' })
    }
}

impl LoadableFromFile for Matrix2D<Square> {
    fn load(filename: &str) -> Matrix2D<Square> {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        let mut row_count: i64 = 0;
        let mut column_count: i64 = 0;
        let mut data = Vec::new();
        for line in reader.lines() {
            if let Ok(l) = line {
                if column_count == 0 {
                    column_count = l.len() as i64;
                    println!("setting column count to: {}", column_count);
                }
                row_count += 1;
                l.as_bytes().iter().for_each(|b| {
                    data.push(if *b == ('#' as u8) {
                        Square::Blocked
                    } else {
                        Square::Free
                    })
                })
            }
        }
        println!("row count total: {}", row_count);
        println!(
            "data size: {}, expected: {}",
            data.len(),
            row_count * column_count
        );

        Matrix2D::<Square> {
            data: data,
            size: Point2D {
                x: column_count,
                y: row_count,
            },
        }
    }
}

pub fn part_one(map: &Matrix2D<Square>, slope: &Point2D) -> i64 {
    let mut tree_count: i64 = 0;
    let mut current = Point2D { x: 0, y: 0 };
    while current.y < map.size.y {
        // The map is infinite in a repeating pattern, but only in the X direction.
        current.x = current.x % map.size.x;
        if map.get(&current) == Square::Blocked {
            tree_count += 1;
        }
        current += *slope;
    }
    tree_count
}

pub fn part_two(map: &Matrix2D<Square>) -> i64 {
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
    let map = Matrix2D::<Square>::load(INPUT_FILENAME);
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
        const INPUT_FILENAME: &'static str = "input/day_three_part_one_example.txt";
        const SLOPE: Point2D = Point2D { x: 3, y: 1 };
        let map = Matrix2D::<Square>::load(INPUT_FILENAME);
        assert_eq!(7, part_one(&map, &SLOPE));
    }

    #[test]
    pub fn solves() {
        assert_eq!("part one: 176, part two: 5872458240", solve());
    }
}
