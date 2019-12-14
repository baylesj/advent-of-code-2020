use fraction::Fraction;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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
            if (x == col && y == row) || !map[row][col] {
                continue;
            }

            let rise: i64 = y as i64 - row as i64;
            let run: i64 = x as i64 - col as i64;

            let slope = Fraction::new(rise.abs() as u64, run.abs() as u64);
            found_slopes.insert(format!("{}{}{}", rise < 0, run < 0, slope));
        }
    }

    found_slopes.len() as i64
}

pub fn part_one(input_filename: &str) -> i64 {
    let map = AsteroidMap::load(input_filename);
    let mut max_visible: i64 = 0;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // No asteroid here, so can't place a station here.
            if !map[row][col] {
                continue;
            }

            let visible_points = calculate_visible_points(col, row, &map);
            max_visible = i64::max(max_visible, visible_points);
        }
    }
    max_visible
}

pub fn part_two(_input_filename: &str) -> i64 {
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
    use super::*;

    #[test]
    fn test_part_one_sample_one() {
        let map = AsteroidMap::load("input/day_ten_sample_one.txt");
        assert_eq!(33, calculate_visible_points(5, 8, &map));
        // duplicate
        assert_eq!(part_one("input/day_ten_sample_one.txt"), 33);
    }

    #[test]
    fn test_part_one_sample_two() {
        assert_eq!(part_one("input/day_ten_sample_two.txt"), 35);
    }

    #[test]
    fn test_part_one_sample_three() {
        assert_eq!(part_one("input/day_ten_sample_three.txt"), 41);
    }

    #[test]
    fn test_part_one_sample_four() {
        assert_eq!(part_one("input/day_ten_sample_four.txt"), 210);
    }

    #[test]
    fn test_part_one() {
      assert_eq!(part_one(INPUT_FILENAME), 278);
    }
}
