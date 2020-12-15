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

// Special thanks to:
// https://math.stackexchange.com/questions/1322845/how-to-find-the-coefficients-of-bezouts-lemma
fn compute_bezoit_coprime(a: i64, b: i64) -> (i64, i64) {
    // Note on tuplet form: 0 is the ith term, 1 is the i-1th term.
    let mut rs = (b, a);
    let mut us = (0, 1);
    let mut vs = (1, 0);
    let mut quotient;
    while rs.0 > 1 {
        quotient = rs.1 / rs.0;
        us = (us.1 - quotient * us.0, us.0);
        vs = (vs.1 - quotient * vs.0, vs.0);
        rs = (rs.1 - quotient * rs.0, rs.0);
    }
    (us.0, vs.0)
}


// Got a hint: Chinese Remainder Theorem
// (https://en.wikipedia.org/wiki/Chinese_remainder_theorem)
// Assumes everything is coprime, which makes sense otherwise one bus is
// strictly a child of another bus.
fn part_two(buses: &[(usize, i64)]) -> i64 {
    // Constructive existence proof:
    // x = a_1 mod n_1
    // x = a_2 mod n_2
    // bezout gives m_1*n_1 + m_2*n_2 = 1
    // compute_bezoit_coprime -> m_1, m_2
    // x = a_1*m_2*n_2 + a_2*m_1*n_1
    // let first = notes.buses_in_service.first().unwrap();
    // let mut a_prime: i64 = first.0 as i64;
    // let mut mod_prime: i64 = first.1;

    // for &bus in notes.buses_in_service.iter().skip(1) {
    //     let bus_index = bus.0 as i64;
    //     let bezoits = compute_bezoit_coprime(mod_prime, bus_index);
    //     a_prime = a_prime * bezoits.1 * bus.1 + bus_index * bezoits.0 * mod_prime;
    //     mod_prime *= bus.1;
    // }
    // a_prime

    println!("buses: {:?}", buses);
    // Big M is the resulting modulus/all of the modulo values (n) multiplied
    // together.
    let mut big_m = 1;
    for bus in buses.iter() {
        big_m *= bus.1;
    }
    println!("big m: {}", big_m);
    let length = buses.len();

    // ms are the big modulus divided by the respective modulo value,
    // i.e. big m without the respective value in it.
    let mut ms = vec![0; length];
    for k in 0..length {
        ms[k] = big_m / buses[k].1;
    }

    // mis are the inverse of the ms, which is m_k modulo n_k
    let mut mis = vec![0; length];
    for k in 0..length {
        mis[k] = ms[k] % buses[k].1;
    }

    // out is the "Chinese Remainder", which is defined as the
    // (sum_{n=0}^{n} (mi_k * m_k * a_k)) % big_m, and is
    // the reaminder of the division some integer N by the product
    // of the integers.
    let mut out: i64 = 0;
    for k in 0..length {
        out += mis[k] * ms[k] * (buses[k].0) as i64
    }
    out %= big_m;
    println!("ms: {:?}\nmis: {:?}\nout: {}", ms, mis, out);
    for bus in buses {
        println!(
            "expected: {}, actual: {}, bus: {}",
            bus.0,
            out % bus.1,
            bus.1
        )
    }
    out
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
        assert_eq!(1068781, part_two(&notes.buses_in_service));
    }

    #[test]
    pub fn internet_example() {
        // https://www.dcode.fr/chinese-remainder
        assert_eq!(23, part_two(&[(2, 3), (3, 5), (2, 7)]));
    }

    #[test]
    pub fn test_bezoit() {
        assert_eq!((-59, 1709), compute_bezoit_coprime(4258, 147));
    }
}
