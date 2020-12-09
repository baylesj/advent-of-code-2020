use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_one.txt";
const DESIRED_SUM: i32 = 2020;

pub fn solve_part_one(data: &[i32], left: usize, right: usize, desired_sum: i32) -> Option<i32> {
    let mut l_i: usize = left;
    let mut r_i: usize = right;
    let mut current_sum: i32 = data[l_i] + data[r_i];
    while current_sum != desired_sum && l_i < r_i {
        if desired_sum < current_sum {
            r_i -= 1;
        } else {
            l_i += 1;
        }
        current_sum = data[l_i] + data[r_i];
    }
    if current_sum == desired_sum {
        return Some(data[l_i] * data[r_i]);
    }
    None
}

pub fn solve_part_two(data: &[i32], desired_sum: i32) -> Option<i32> {
    let mut part_two_answer: Option<i32> = None;
    for i in 0..(data.len() - 2) {
        let partial = solve_part_one(
            &data[i + 1..],
            0,
            data.len() - i - 2,
            desired_sum - data[i] as i32,
        );
        match partial {
            Some(p) => {
                part_two_answer = Some(p * data[i] as i32);
                break;
            }
            None => continue,
        }
    }
    part_two_answer
}

pub fn load_data() -> Vec<i32> {
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
    data
}

pub fn solve() -> String {
    let data = load_data();

    format!(
        "part one: {}, part two: {}",
        solve_part_one(data.as_slice(), 0, data.len() - 1, DESIRED_SUM).unwrap(),
        solve_part_two(data.as_slice(), DESIRED_SUM).unwrap()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solves() {
        assert_eq!("part one: 751776, part two: 42275090", solve());
    }
}
