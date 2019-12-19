use std::fs;

const INPUT_FILENAME: &str = "input/day_sixteen.txt";

fn load(input_filename: &str) -> Vec<i8> {
    fs::read_to_string(input_filename).expect("valid file").chars().
        map(|c| c.to_digit(10).expect("NaN") as i8).collect()
}

const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];
pub fn calculate_other_vector(output_index: usize, length: usize) -> Vec<i8> {
    let mut i = 0;
    let mut output = vec![0i8; length];
    let mut is_first_digit = true;
    while i < length {
        for b in BASE_PATTERN.iter() {
            for _ in 0..output_index + 1 {
                if is_first_digit {
                    is_first_digit = false;
                    continue;
                }
                output[i] = *b;
                i += 1;
                if i >= length {
                  return output;
                }
            }
        }
    }
    output
}

pub fn part_one(input_filename: &str) -> i64 {
    let input = load(input_filename);
    1
}

pub fn part_two() -> i64 {
    1
}

pub fn solve() {
    println!("Day sixteen, part one: {}, part two: {}", part_one(INPUT_FILENAME), part_two());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_other_vector() {
        assert_eq!(vec![1, 0, -1, 0], calculate_other_vector(0, 4));
        assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0], calculate_other_vector(0, 8));
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0], calculate_other_vector(1, 8));
    }

    #[test]
    pub fn test_part_one_sample_one() {
        assert_eq!(24176176, part_one("input/day_sixteen_sample_one.txt"))
    }

    #[test]
    pub fn test_part_one_sample_two() {
        assert_eq!(73745418, part_one("input/day_sixteen_sample_two.txt"))

    }

    #[test]
    pub fn test_part_one_sample_three() {
        assert_eq!(52432133, part_one("input/day_sixteen_sample_three.txt"))
    }
}