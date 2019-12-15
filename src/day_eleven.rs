use enum_primitive_derive::Primitive;
use num_traits::ToPrimitive;

#[path = "intcode_computer.rs"]
mod intcode_computer;
use intcode_computer::LoadableFromFile;
use intcode_computer::Program;
use intcode_computer::Runnable;
use queues::IsQueue;

const INPUT_FILENAME: &'static str = "input/day_eleven.txt";

#[derive(Primitive)]
enum TileColor {
    Black = 0,
    White = 1,
}

struct Point {
  x: i64,
  y: i64
}

pub fn part_one(input_filename: &str) -> i64 {
    let mut program = Program::load(input_filename);
    program.io.add(TileColor::Black.to_i64().unwrap()).ok();
    program.run();

    println!("io: {:#?}", program.io);

    128
}

pub fn solve() {
    println!("Day eleven, part one: {}", part_one(INPUT_FILENAME));
}
