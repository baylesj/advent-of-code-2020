use crate::yet_another_geometry_mod::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

pub trait LoadableFromFile {
    fn load(filename: &str) -> Self;
}

impl LoadableFromFile for Vec<i64> {
    fn load(filename: &str) -> Vec<i64> {
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| i64::from_str(&l.unwrap()).unwrap())
            .collect()
    }
}

impl LoadableFromFile for Matrix2D<char> {
    fn load(filename: &str) -> Matrix2D<char> {
        let file = File::open(filename).expect("Invalid filename");
        let reader = BufReader::new(file);

        let mut row_count: i64 = 0;
        let mut column_count: i64 = 0;
        let mut data = Vec::new();
        for line in reader.lines() {
            if let Ok(l) = line {
                if column_count == 0 {
                    column_count = l.len() as i64;
                }
                row_count += 1;
                l.as_bytes().iter().for_each(|b| {
                    data.push(*b as char);
                });
            }
        }

        Matrix2D::<char> {
            data: data,
            size: Point2D {
                x: column_count,
                y: row_count,
            },
        }
    }
}
