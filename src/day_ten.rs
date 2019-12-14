use fraction::{GenericFraction, Sign};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[path = "loadable.rs"]
mod loadable;
use loadable::LoadableFromFile;

const INPUT_FILENAME: &'static str = "input/day_ten.txt";

const EMPTY_POSITION: char = '.';
const ASTEROID_POSITION: char = '#';

type AsteroidMap = Vec<Vec<bool>>;

impl LoadableFromFile for AsteroidMap {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("invalid file");
        let reader = BufReader::new(file);

        let mut output: Vec<Vec<bool>> = Vec::new();
        for line in reader.lines() {
            output.push(
                line.expect("invalid line")
                    .as_bytes()
                    .iter()
                    .map(|c| match *c as char {
                        ASTEROID_POSITION => true,
                        EMPTY_POSITION => false,
                        _ => panic!("invalid character encountered"),
                    })
                    .collect(),
            );
        }

        output
    }
}

// x equals column, y equals row. Top Left = (0, 0)
pub fn calculate_visible_points(x: usize, y: usize, map: &AsteroidMap) -> i64 {
    let mut found_slopes = HashSet::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (x == col && y == row) || !map[y][x] {
                continue;
            }

            let rise: i64 = y as i64 - row as i64;
            let run: i64 = x as i64 - col as i64;
            // TODO: sign here is dumb...
            let sign: Sign;
            if (rise < 0) == (run < 0) {
                sign = Sign::Plus;
            } else {
                sign = Sign::Minus;
            }

            let slope = GenericFraction::<u64>::new_generic(sign, rise.abs(), run.abs()).unwrap();
            found_slopes.insert(slope.to_string());
        }
    }

    found_slopes.len() as i64
}

pub fn part_one(input_filename: &str) -> i64 {
    let map = AsteroidMap::load(input_filename);
    let mut max_visible: i64 = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            max_visible = i64::max(max_visible, calculate_visible_points(col, row, &map));
        }
    }
    max_visible
}

pub fn part_two(input_filename: &str) -> i64 {
    128
}

pub fn solve() {
    println!(
        "Day ten, part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME)
    );
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_foo() {}
}
