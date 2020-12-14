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

// TODO: multiple factors?
fn find_a_factor(buses: &[(usize, i64)]) -> i64 {
    // if the first bus is 7, it leaves every 7 minutes.
    // if there is a bus at INDEX 7, the 7 bus also leaves at that time, e.g.
    // x % 19 = 7
    // x % 7 = 0

    // The time that 19 leaves at has to be divisible by 19, but it also
    // has to be divisible by 7, which means that the number has to be a
    // LEAST COMMON MULTIPLE.
    let first = buses.first().unwrap().1;
    for b in buses.iter().skip(1) {
        if b.0 as i64 % first == 0 {
            println!("I think bus {:?} is a factor.", b);
            return first * b.1;
        }
    }
    panic!("no factors");
}

fn part_two(notes: &BusNotes) -> i64 {
    let first = notes.buses_in_service.first().unwrap().1;
    let lcmish = find_a_factor(&notes.buses_in_service);
    let mut cur = lcmish - first;
    while !notes
        .buses_in_service
        .iter()
        .all(|b| get_wait(cur, b.1) == b.0 as i64)
    {
        cur += lcmish;
    }
    cur
}

pub fn solve() -> String {
    let notes = BusNotes::load("input/day_thirteen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&notes),
        part_two(&notes)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        // TODO: delete load and part_one() call and reenable solve when performance is fixed for part two.
        let notes = BusNotes::load("input/day_thirteen.txt");
        assert_eq!(3966, part_one(&notes));
        //assert_eq!("part one: 3966, part two: 0", solve());
    }

    #[test]
    pub fn test_example() {
        let notes = BusNotes::load("input/day_thirteen_example.txt");
        assert_eq!(295, part_one(&notes));
    }

    #[test]
    pub fn test_example_two() {
        let notes = BusNotes::load("input/day_thirteen_example.txt");
        assert_eq!(1068781, part_two(&notes));
    }
}
