#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::{LoadableFromFile, Program, Runnable};
use queues::IsQueue;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::{Advance, Direction, Matrix2D, Matrix2DLike, Point2D};

const INPUT_FILENAME: &str = "input/day_seventeen.txt";

fn get_matrix(input_filename: &str) -> Matrix2D<char> {
    let mut program = Program::load(input_filename);
    program.run_until_halted();

    let mut chars = Vec::new();
    let mut num_cols: i64 = 0;
    for n in 0..program.io.size() {
        let i: u8 = program.io.remove().expect("io size wrong") as u8;
        if i > 0 && i as char != '0' && i as char != '\n' {
            chars.push(i as char);
        } else if i as char == '\n' && num_cols == 0 {
            num_cols = n as i64;
        }
    }
    let num_rows: i64 = chars.len() as i64 / num_cols;
    Matrix2D::create_with_data(
        &Point2D {
            x: num_cols,
            y: num_rows,
        },
        chars,
    )
}

pub fn part_one(input_filename: &str) -> i64 {
    let matrix = get_matrix(input_filename);
    println!("Matrix: {}", matrix);

    // exclude the "walls" from consideration.
    let mut scaffold_intersections = Vec::new();
    let mut current = Point2D::default();
    for y in 1..matrix.size().y - 1 {
        current.y = y;
        for x in 1..matrix.size().x - 1 {
            current.x = x;

            if matrix.get(&current) == '#'
                && matrix.get(&current.advance_copy(Direction::Left)) == '#'
                && matrix.get(&current.advance_copy(Direction::Right)) == '#'
                && matrix.get(&current.advance_copy(Direction::Up)) == '#'
                && matrix.get(&current.advance_copy(Direction::Down)) == '#'
            {
                scaffold_intersections.push(current.clone());
            }
        }
    }
    scaffold_intersections.iter().map(|i| i.x * i.y).sum()
}

pub fn part_two() -> i64 {
    1
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one(INPUT_FILENAME),
        part_two()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(6212, part_one(INPUT_FILENAME));
    }
}
