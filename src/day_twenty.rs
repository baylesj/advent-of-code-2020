use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::{Matrix2D, Point2D};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Note: tiles are 10x10
#[derive(Debug)]
struct Tile {
    id: i32,
    data: Matrix2D<char>,
}

impl LoadableFromFile for Vec<Tile> {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        let mut lines = reader.lines();
        let mut tiles = vec![];
        while let Some(l) = lines.next() {
            let tile_name = l.unwrap()[5..9].parse::<i32>().unwrap();
            let mut data = Vec::new();
            for _ in 0..10 {
                lines
                    .next()
                    .expect("valid")
                    .unwrap()
                    .chars()
                    .for_each(|c| data.push(c))
            }
            // Grab empty line between tiles (or EOF).
            lines.next();

            tiles.push(Tile {
                id: tile_name,
                data: Matrix2D::<char> {
                    data: data,
                    size: Point2D { x: 10, y: 10 },
                },
            });
        }
        tiles
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
        let tiles = Vec::<Tile>::load("input/day_twenty_example.txt");
        println!("tiles: {:?}", tiles);
        // input/day_twenty_example.txt
        // 1951 * 3079 * 2971 * 1171 = 20899048083289
    }
}
