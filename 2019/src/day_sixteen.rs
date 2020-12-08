use std::fs;

const INPUT_FILENAME: &'static str = "input/day_sixteen.txt";

fn load(input_filename: &str) -> Vec<i16> {
    fs::read_to_string(input_filename)
        .expect("valid file")
        .chars()
        .map(|c| c.to_digit(10).expect("NaN") as i16)
        .collect()
}

const BASE_PATTERN: [i16; 4] = [0, 1, 0, -1];
pub fn calculate_weights(length: usize) -> Vec<i16> {
    let mut output = vec![0i16; length * length];
    for index in 0..length {
        let mut is_first_digit = true;
        let mut i = 0;
        'per_digit: while i < length {
            for b in BASE_PATTERN.iter() {
                for _ in 0..index + 1 {
                    if is_first_digit {
                        is_first_digit = false;
                        continue;
                    }
                    output[(index * length) + i] = *b;
                    i += 1;
                    if i >= length {
                        break 'per_digit;
                    }
                }
            }
        }
    }
    output
}

pub fn run_phase(input: &Vec<i16>, weights: &Vec<i16>) -> Vec<i16> {
    let mut output = vec![0; input.len()];
    for j in 0..input.len() {
        for i in 0..input.len() {
            let w = weights[j * input.len() + i];
            if w > 0 {
                output[j] += input[i];
            } else if w < 0 {
                output[j] -= input[i];
            }
        }
        output[j] = output[j].abs() % 10;
    }

    output
}

/// can store partial sums
pub fn part_one(input_filename: &str) -> i64 {
    const NUM_PHASES: i64 = 100;
    let mut input = load(input_filename);
    let weights = calculate_weights(input.len());
    for _ in 0..NUM_PHASES {
        input = run_phase(&input, &weights);
    }

    const NUM_PREFIX: usize = 8;
    input.truncate(NUM_PREFIX);
    let mut output: i64 = 0;
    for i in 0..NUM_PREFIX {
        output *= 10;
        output += input[i] as i64;
    }
    output
}

fn get_first_n_as_num(n: usize, vector: &Vec<i16>) -> usize {
    let values = vector[0..n].to_vec();
    assert_eq!(values.len(), n);

    let mut num: usize = 0;
    for i in 0..n {
        num *= 10;
        num += values[i] as usize;
    }
    num
}

// For part two, we can avoid solving the general problem
pub fn part_two(input_filename: &str) -> i64 {
    const OFFSET_SIZE: usize = 7;
    const OUTPUT_SIZE: usize = 8;
    const NUM_PHASES: usize = 100;
    const NUM_TIMES_INPUT_REPEATED: usize = 10000;

    let input = load(input_filename);
    let offset = get_first_n_as_num(OFFSET_SIZE, &input);
    let final_input_size: usize = input.len() * NUM_TIMES_INPUT_REPEATED;
    // Position starts the character AFTER offset.
    let position_in_input = offset % final_input_size;
    let final_input_fragment_size = final_input_size - position_in_input;

    // This algorithm is only well formed for the last half of inputs.
    assert!(position_in_input >= final_input_size / 2);
    let mut final_input = vec![0; final_input_fragment_size];
    for i in position_in_input..final_input_size {
        final_input[i - position_in_input] = input[i % input.len()];
    }

    for _ in 0..NUM_PHASES {
        let mut sum: i64 = final_input[final_input.len() - 1] as i64;
        for i in (0..final_input_fragment_size - 1).rev() {
            // Sum has to use the "old" final_input, and final_input needs
            // the "old" sum, so store the calculation first.
            let next = ((final_input[i] as i64 + sum) % 10) as i16;
            sum += final_input[i] as i64;
            final_input[i] = next;
        }
    }

    get_first_n_as_num(OUTPUT_SIZE, &final_input[0..OUTPUT_SIZE].to_vec()) as i64
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two(INPUT_FILENAME)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_weights() {
        assert_eq!(vec![1, 0, 0, 1], calculate_weights(2));
        assert_eq!(vec![1, 0, -1, 0, 1, 1, 0, 0, 1], calculate_weights(3));
        assert_eq!(
            vec![1, 0, -1, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1],
            calculate_weights(4)
        );
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

    #[test]
    pub fn test_part_one() {
        assert_eq!(63483758, part_one("input/day_sixteen.txt"))
    }

    #[test]
    pub fn test_part_two() {
        assert_eq!(96099551, part_two("input/day_sixteen.txt"))
    }
}
