use crate::loadable::LoadableFromFile;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

const INPUT_FILENAME: &'static str = "input/day_five.txt";

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct BoardingPass {
    row: i32,
    column: i32,
    seat_id: i64,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
struct Range {
    left: i32,
    right: i32,
}

trait BinarySearch {
    fn midpoint(&self) -> i32;
    fn pick_left(&mut self);
    fn pick_right(&mut self);
}

impl BinarySearch for Range {
    fn midpoint(&self) -> i32 {
        (self.right + self.left) / 2
    }

    fn pick_left(&mut self) {
        self.right = self.midpoint();
    }

    fn pick_right(&mut self) {
        let mut mid = self.midpoint();
        if mid < self.right {
            mid += 1;
        }
        self.left = mid;
    }
}

impl FromStr for BoardingPass {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_range = Range {
            left: 0,
            right: 127,
        };
        for i in 0..7 {
            if s.as_bytes()[i] as char == 'F' {
                row_range.pick_left();
            } else {
                row_range.pick_right();
            }
        }
        assert_eq!(row_range.left, row_range.right);

        let mut col_range = Range { left: 0, right: 7 };
        for i in 7..10 {
            if s.as_bytes()[i] as char == 'L' {
                col_range.pick_left()
            } else {
                col_range.pick_right()
            }
        }
        assert_eq!(col_range.left, col_range.right);

        Ok(BoardingPass {
            row: row_range.left,
            column: col_range.left,
            seat_id: row_range.left as i64 * 8 + col_range.left as i64,
        })
    }
}

impl LoadableFromFile for Vec<BoardingPass> {
    fn load(filename: &str) -> Vec<BoardingPass> {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| BoardingPass::from_str(&l.unwrap()).unwrap())
            .collect()
    }
}

fn part_one(passes: &[BoardingPass]) -> i64 {
    let mut maximum: i64 = 0;
    for pass in passes {
        maximum = std::cmp::max(pass.seat_id, maximum);
    }
    maximum
}

fn part_two(passes: &[BoardingPass]) -> i64 {
    let mut ids: Vec<i64> = passes.iter().map(|p| p.seat_id).collect();
    ids.sort();
    for i in 1..ids.len() - 1 {
        if ids[i] + 1 != ids[i + 1] {
            return ids[i] + 1;
        }
    }
    return -1;
}

pub fn solve() -> String {
    let passes = Vec::<BoardingPass>::load(INPUT_FILENAME);
    format!(
        "part one: {}, part two: {}",
        part_one(&passes),
        part_two(&passes)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 892, part two: 625", solve());
    }

    #[test]
    pub fn test_ranges() {
        let mut range = Range {
            left: 0,
            right: 127,
        };
        range.pick_left();
        assert_eq!(range, Range { left: 0, right: 63 });
        range.pick_right();
        assert_eq!(
            range,
            Range {
                left: 32,
                right: 63
            }
        );
        range.pick_left();
        assert_eq!(
            range,
            Range {
                left: 32,
                right: 47
            }
        );
        range.pick_right();
        assert_eq!(
            range,
            Range {
                left: 40,
                right: 47
            }
        );
        range.pick_right();
        assert_eq!(
            range,
            Range {
                left: 44,
                right: 47
            }
        );
        range.pick_left();
        assert_eq!(
            range,
            Range {
                left: 44,
                right: 45
            }
        );
        range.pick_right();
        assert_eq!(
            range,
            Range {
                left: 45,
                right: 45
            }
        );
    }

    #[test]
    pub fn check_boarding_pass_examples() {
        assert_eq!(
            BoardingPass {
                row: 70,
                column: 7,
                seat_id: 567
            },
            BoardingPass::from_str("BFFFBBFRRR").unwrap()
        );
        assert_eq!(
            BoardingPass {
                row: 14,
                column: 7,
                seat_id: 119
            },
            BoardingPass::from_str("FFFBBBFRRR").unwrap()
        );
        assert_eq!(
            BoardingPass {
                row: 102,
                column: 4,
                seat_id: 820
            },
            BoardingPass::from_str("BBFFBBFRLL").unwrap()
        );
    }
}
