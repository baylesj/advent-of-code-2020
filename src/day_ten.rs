use std::collections::BTreeMap;
use std::f64::consts::PI;
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

#[derive(Debug, Default)]
pub struct VisiblePoint {
    pub x: usize,
    pub y: usize,
    pub distance: f64,
}

pub type VisiblePoints = BTreeMap<String, Vec<VisiblePoint>>;

// x equals column, y equals row. Top Left = (0, 0)
pub fn calculate_visible_points(x: usize, y: usize, map: &AsteroidMap) -> VisiblePoints {
    let mut found_slopes: VisiblePoints = BTreeMap::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (x == col && y == row) || !map[row][col] {
                continue;
            }

            let rise: i64 = y as i64 - row as i64;
            let run: i64 = x as i64 - col as i64;

            // UR - 4, LR - 3, LL - 2, UL - 1
            let mut arctan: f64 = (rise as f64 / run as f64).atan();
            if run < 0 {
                arctan += PI;
            }

            let key = format!("{:.1$}", arctan, 9);
            let point = VisiblePoint {
                x: col,
                y: row,
                distance: ((rise * rise + run * run) as f64).sqrt(),
            };

            let points_with_same_slope = found_slopes.entry(key).or_insert(vec![]);
            points_with_same_slope.push(point);
        }
    }

    found_slopes
}

pub fn get_best_location(input_filename: &str) -> VisiblePoints {
    let map = AsteroidMap::load(input_filename);
    let mut max_visible: i64 = 0;
    let mut max_visible_points = VisiblePoints::default();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // No asteroid here, so can't place a station here.
            if !map[row][col] {
                continue;
            }

            let visible_points = calculate_visible_points(col, row, &map);
            if visible_points.len() as i64 > max_visible {
                max_visible = visible_points.len() as i64;
                max_visible_points = visible_points;
            }
        }
    }
    max_visible_points
}

pub fn part_one(input_filename: &str) -> i64 {
    get_best_location(input_filename).len() as i64
}

// Question is, what is the 200th asteroid to be vaporized?
// Minus one to account for 0-indexing.
const NTH_ASTEROID_PLACE: usize = 200 - 1;
pub fn part_two(input_filename: &str) -> i64 {
    let location = get_best_location(input_filename);
    let sorted_keys: Vec<&String> = location.keys().rev().collect();

    // Found experimentally.
    // TODO: generalize to any case.
    assert!(NTH_ASTEROID_PLACE < sorted_keys.len());

    println!("sorted keys: {:#?}", sorted_keys);
    let visible_point = location[sorted_keys[NTH_ASTEROID_PLACE]]
        .iter()
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).expect("ordered"))
        .unwrap();
    println!(
        "sorted keys: {:#?}",
        location[sorted_keys[NTH_ASTEROID_PLACE]]
    );
    (visible_point.x * 100 + visible_point.y) as i64
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
