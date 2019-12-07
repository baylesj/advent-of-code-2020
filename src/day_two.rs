use std::fs;

const INPUT_FILENAME: &str = "input/day_two_input.txt";

fn operation_code_one(index: usize, program: &mut Vec<i32>) {
    let a_i: usize = program[index + 1] as usize;
    let b_i: usize = program[index + 2] as usize;
    let r_i: usize = program[index + 3] as usize;

    program[r_i] = program[a_i] + program[b_i];
}

// TODO: create generic and pass operator?
fn operation_code_two(index: usize, program: &mut Vec<i32>) {
    let a_i: usize = program[index + 1] as usize;
    let b_i: usize = program[index + 2] as usize;
    let r_i: usize = program[index + 3] as usize;

    program[r_i] = program[a_i] * program[b_i];
}

fn perform_operation(index: usize, program: &mut Vec<i32>) -> bool {
    match program[index] {
        1 => {
            operation_code_one(index, program);
            true
        }
        2 => {
            operation_code_two(index, program);
            true
        }
        // 99 is HALT.
        99 => false,
        _ => panic!("unknown op code received!"),
    }
}

fn fixup_program(program: &mut Vec<i32>) {
    program[1] = 12;
    program[2] = 2;
}

pub fn solve() -> String {
    let fc: String = fs::read_to_string(INPUT_FILENAME).expect("invalid filename");
    let mut program: Vec<i32> = fc.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    fixup_program(&mut program);

    let mut index: usize = 0;
    while perform_operation(index, &mut program) {
        index += 4;
    }

    program[0].to_string()
}
