use crate::loadable::LoadableFromFile;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[path = "yet_another_geometry_mod.rs"]
mod yet_another_geometry_mod;
use yet_another_geometry_mod::{Matrix2D, Point2D};

const INPUT_FILENAME: &'static str = "input/day_three.txt";

type Square = bool;

impl LoadableFromFile for Matrix2D<Square> {
    fn load(filename: &str) -> Matrix2D<Square> {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        let mut row_count: i64 = 0;
        let mut column_count: i64 = 0;
        let mut data = Vec::new();
        for line in reader.lines() {
            if let Ok(l) = line {
                if column_count == 0 {
                    column_count = l.len() as i64;
                    println!("setting column count to: {}", column_count);
                }
                row_count += 1;
                l.as_bytes()
                    .iter()
                    .for_each(|b| data.push(*b == ('#' as u8)))
            }
        }
        println!("row count total: {}", row_count);
        println!(
            "data size: {}, expected: {}",
            data.len(),
            row_count * column_count
        );

        Matrix2D::<Square> {
            data: data,
            size: Point2D {
                x: column_count,
                y: row_count,
            },
        }
    }
}

pub fn solve() -> String {
    let map = Matrix2D::<Square>::load(INPUT_FILENAME);
    format!("part one: {}, part two: {}", map, 2)
}
