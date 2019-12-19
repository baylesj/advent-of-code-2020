use std::fs;

const INPUT_FILENAME: &str = "input/day_sixteen.txt";

fn load(input_filename: &str) -> Vec<i16> {
    fs::read_to_string(input_filename)
        .expect("valid file")
        .chars()
        .map(|c| c.to_digit(10).expect("NaN") as i16)
        .collect()
}

const BASE_PATTERN: [i16; 4] = [0, 1, 0, -1];
pub fn calculate_weights(output_index: usize, length: usize) -> Vec<i16> {
    let mut i = 0;
    let mut output = vec![0i16; length];
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

pub fn run_phase(input: &Vec<i16>) -> Vec<i16> {
    let mut output = vec![0; input.len()];
    for i in 0..input.len() {
        let weights = calculate_weights(i, input.len());
        output[i] = weights.iter().zip(input.iter()).map(|(a, b)| (a * b)).sum();
        output[i] = output[i].abs() % 10;
    }

    output
}

// Size 8 example:
// bottom to top:
// on row N, zero out N - 1 elements, sum N ones, zero N ones, subtract N
// 0 0 0 0 0 0 0 1 -> A8 = i8
// 0 0 0 0 0 0 1 1 -> B7 = A8 + i7
// 0 0 0 0 0 1 1 1 -> C6 = B7 + i6
// 0 0 0 0 1 1 1 1 -> D5 = C6 + i5 (row 5, zero out 4, sum 5 elemnets)
// 0 0 0 1 1 1 1 0 -> E4 = D5 + i4 - A8 (row 4, zero out 3, sum 4, ...)
// 0 0 1 1 1 0 0 0 -> F3 = E4 + i3 - C6
// 0 1 1 0 0 - - 0 -> G2 = F3 + i2 - E4 (row 1, zero out 1, sum 2, zero out 2, minus 2)
// 1 0 - 0 1 0 - 0 -> H1 = F + i1 - G (row 0, zero out 0, sum 1, zero out 0, minus 1, zero)

// can store partial sums
pub fn part_one(input_filename: &str) -> i64 {
    const NUM_PHASES: i64 = 100;
    let mut input = load(input_filename);
    for _ in 0..NUM_PHASES {
        input = run_phase(&input);
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

pub fn part_two() -> i64 {
    1
}

pub fn solve() {
    println!(
        "Day sixteen, part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_weights() {
        assert_eq!(vec![1, 0, -1, 0], calculate_weights(0, 4));
        assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0], calculate_weights(0, 8));
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0], calculate_weights(1, 8));
    }

    #[test]
    pub fn test_run_phase() {
        assert_eq!(
            vec![4, 8, 2, 2, 6, 1, 5, 8],
            run_phase(&vec![1, 2, 3, 4, 5, 6, 7, 8])
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
}
