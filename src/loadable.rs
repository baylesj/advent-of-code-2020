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
