use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &str = "input/day_one.txt";

// Algorithm:
//     sort list
// 1 4 5 7 9  adds to 13
// 1 needs 12, so go right
// 4 needs 9, done
// Advance left if too low, retreat right if too high.
// n*log(n) algorithm.
// Example 2:
// 3 5 7 9 15 22 50 100 adds to 29
// 3 + 100 too high, retreat
// 3 + 50 too high, retreat
// 3 + 22 too low, advance
// 5 + 22 too low, advance
// 7 + 22 = 29
// return 7 * 22
pub fn solve() -> String {
    let file = File::open(INPUT_FILENAME).expect("Invalid filename");
    let reader = BufReader::new(file);

    let mut data: Vec<i32> = reader
        .lines()
        .map(|r| match r {
            Ok(n) => n.parse::<i32>().unwrap(),
            Err(e) => panic!(e),
        })
        .collect();
    data.sort();

    let mut l_i: usize = 0;
    let mut r_i: usize = data.len() - 1;

    const DESIRED_SUM: i32 = 2020;

    let mut current_sum: i32 = data[l_i] + data[r_i];
    while current_sum != DESIRED_SUM {
        if DESIRED_SUM < current_sum {
            r_i -= 1;
        } else {
            l_i += 1;
        }
        assert!(l_i < r_i);
        current_sum = data[l_i] + data[r_i];
    }

    format!("part one: {}", data[l_i] * data[r_i])
}
