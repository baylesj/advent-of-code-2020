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
struct VisiblePoint {
    x: usize,
    y: usize,
    distance: f64,
}

type VisiblePoints = BTreeMap<i64, Vec<VisiblePoint>>;

#[derive(Debug, Default)]
struct StationLocation {
    x: usize,
    y: usize,
    points: VisiblePoints,
}

fn to_key(rise: i64, run: i64) -> i64 {
    // For part two, we want to map "straight up" as the first entry,
    // and go counterclockwise. Arctangent gives us a direct way to
    // calculate radians from a given slope, however it doesn't have the
    // right limits for what we want: "straight up" is actually pi/2,
    // and radians sweep counter clockwise. We subtract pi/2 to get
    // straight up to be zero, and multiply by -1 to move clockwise.
    // Finally, the range of arctan is only -pi/2 -> pi/2, so we add
    // PI if the run is negative (i.e. we are in Cartesian quadrant II or III).
    let mut at: f64 = -1.0 * ((rise as f64 / run as f64).atan() - PI / 2.0);
    if run < 0 {
        at += PI;
    }

    // Done in degrees as integer for ease of testing, sorting.
    (at * 360.0 * 1000.0 / (2.0 * PI)).round() as i64
}

// x equals column, y equals row. Top Left = (0, 0)
fn calculate_visible_points(x: usize, y: usize, map: &AsteroidMap) -> VisiblePoints {
    let mut found_slopes: VisiblePoints = BTreeMap::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if (x == col && y == row) || !map[row][col] {
                continue;
            }

            // TODO: this inverse makes for an easier mental
            // model (moving from y = 12 to y = 11 is "up" according
            // to the spec), but is not congruent. Refactor.
            let rise: i64 = y as i64 - row as i64;
            let run: i64 = col as i64 - x as i64;
            let key = to_key(rise, run);

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

fn get_best_location(input_filename: &str) -> StationLocation {
    let map = AsteroidMap::load(input_filename);
    let mut max_visible: i64 = 0;
    let mut best_location = StationLocation::default();

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            // No asteroid here, so can't place a station here.
            if !map[row][col] {
                continue;
            }

            let visible_points = calculate_visible_points(col, row, &map);
            if visible_points.len() as i64 > max_visible {
                max_visible = visible_points.len() as i64;
                best_location = StationLocation {
                    x: col,
                    y: row,
                    points: visible_points,
                };
            }
        }
    }
    best_location
}

pub fn part_one(input_filename: &str) -> i64 {
    get_best_location(input_filename).points.len() as i64
}

pub fn part_two(input_filename: &str, nth: usize) -> i64 {
    let location = get_best_location(input_filename);
    let sorted_keys: Vec<&i64> = location.points.keys().collect();
    // TODO: generalize to any nth < number of asteroids.
    assert!(nth - 1 < sorted_keys.len());

    let visible_point = location.points[&sorted_keys[nth - 1]]
        .iter()
        .min_by(|a, b| a.distance.partial_cmp(&b.distance).expect("ordered"))
        .unwrap();

    (visible_point.x * 100 + visible_point.y) as i64
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME, 200)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_key() {
        assert_eq!(0, to_key(1, 0));
        assert_eq!(1, to_key(100000, 1));
        assert_eq!(45000, to_key(1, 1));
        assert_eq!(90000, to_key(0, 1));
        assert_eq!(135000, to_key(-1, 1));
        assert_eq!(180000, to_key(-1, 0));
        assert_eq!(225000, to_key(-1, -1));
        assert_eq!(270000, to_key(0, -1));
        assert_eq!(315000, to_key(1, -1));
        assert_eq!(360000, to_key(999999, -1));
    }

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

    #[test]
    fn test_part_two_sample_four() {
        assert_eq!(part_two("input/day_ten_sample_four.txt", 1), 1112);
        assert_eq!(part_two("input/day_ten_sample_four.txt", 2), 1201);
        assert_eq!(part_two("input/day_ten_sample_four.txt", 50), 1609);
        assert_eq!(part_two("input/day_ten_sample_four.txt", 100), 1016);
        assert_eq!(part_two("input/day_ten_sample_four.txt", 200), 802);

        // TODO: add support for "rolling over" e.g. values > the size
        // of the array of slopes.
        //assert_eq!(part_two("input/day_ten_sample_four.txt", 299), 1101);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_FILENAME, 200), 1417);
    }
}
