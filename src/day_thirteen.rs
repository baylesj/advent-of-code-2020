use crate::loadable::LoadableFromFile;
use std::fs;
use std::str::FromStr;

struct BusNotes {
    departure_time: i64,
    buses_in_service: Vec<i64>,
}

impl LoadableFromFile for BusNotes {
    fn load(filename: &str) -> Self {
        let data = fs::read_to_string(filename)
            .expect("Unable to read file");
        let mut lines = data.lines();

        BusNotes {
            departure_time: lines.next().unwrap().parse().unwrap(),
            buses_in_service: lines
                .next().unwrap()
                .split(",")
                .filter(|b| *b != "x")
                .map(|b| i64::from_str(b).unwrap())
                .collect(),
        }
    }
}

fn get_wait(departure_time: i64, bus_period: i64) -> i64 {
    let modulo = departure_time % bus_period;
    // A perfect fit!
    if modulo == 0 {
        return 0;
    }
    // Otherwise we have to wait for the rest of the bus's interval.
    bus_period - modulo
}

fn part_one(notes: &BusNotes) -> i64 {
    let mut best_bus = *notes.buses_in_service.first().unwrap();
    let mut best_val = get_wait(notes.departure_time, best_bus);
    for &bus in notes.buses_in_service.iter().skip(1) {
        let val = get_wait(notes.departure_time, bus);
        if val < best_val {
            best_val = val;
            best_bus = bus;
        }
    }
    println!("bus {} is best at {} minutes.", best_bus, best_val);
    best_bus * best_val
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let notes = BusNotes::load("input/day_thirteen.txt");
    format!("part one: {}, part two: {}", part_one(&notes), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 3966, part two: 0", solve());
    }

    #[test]
    pub fn test_example() {
        let notes = BusNotes::load("input/day_thirteen_example.txt");
        assert_eq!(295, part_one(&notes));
    }
}
