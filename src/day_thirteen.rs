use crate::loadable::LoadableFromFile;
use std::fs;
use std::str::FromStr;

struct BusNotes {
    departure_time: i64,
    buses_in_service: Vec<(usize, i64)>,
}

impl LoadableFromFile for BusNotes {
    fn load(filename: &str) -> Self {
        let data = fs::read_to_string(filename).expect("Unable to read file");
        let mut lines = data.lines();

        BusNotes {
            departure_time: lines.next().unwrap().parse().unwrap(),
            buses_in_service: lines
                .next()
                .unwrap()
                .split(",")
                .enumerate()
                .filter(|b| b.1 != "x")
                .map(|b| (b.0, i64::from_str(b.1).unwrap()))
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
    let mut best_val = get_wait(notes.departure_time, best_bus.1);
    for &bus in notes.buses_in_service.iter().skip(1) {
        let val = get_wait(notes.departure_time, bus.1);
        if val < best_val {
            best_val = val;
            best_bus = bus;
        }
    }
    println!("bus {} is best at {} minutes.", best_bus.1, best_val);
    best_bus.1 * best_val
}

// Got a hint: Chinese Remainder Theorem
// (https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
// Assumes everything is coprime, which makes sense otherwise one bus is
// strictly a child of another bus.
fn part_two(buses: &[(usize, i64)]) -> i64 {
    let modulo_equations: Vec<(i64, i64)> =
        buses.iter().map(|b| (-(b.0 as i64) % b.1, b.1)).collect();
    // |r|emainder and |c|oefficient.
    let mut rc = (0, 1);
    for eq in &modulo_equations {
        let coefficient = rc.1;
        for k in 1..eq.1 {
            if (coefficient * k) % eq.1 == 1 {
                rc = ((((eq.0 - rc.0) * k) % eq.1) * rc.1 + rc.0, rc.1 * eq.1);
            }
        }
    }
    rc.0
}

pub fn solve() -> String {
    let notes = BusNotes::load("input/day_thirteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&notes),
        part_two(&notes.buses_in_service)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 3966, part two: 800177252346225", solve());
    }

    #[test]
    pub fn test_example() {
        let notes = BusNotes::load("input/day_thirteen_example.txt");
        assert_eq!(295, part_one(&notes));
    }

    #[test]
    pub fn test_example_two() {
        let notes = BusNotes::load("input/day_thirteen_example.txt");
        assert_eq!(1068781, part_two(&notes.buses_in_service));
    }
}
