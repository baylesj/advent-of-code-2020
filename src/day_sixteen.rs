use crate::loadable::LoadableFromFile;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::str::FromStr;

struct TicketType {
    _name: String,
    valid_ranges: Vec<Range<usize>>,
}

impl FromStr for TicketType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z ]*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
        }
        let cap = RE.captures(s);
        if cap.is_none() {
            return Err("not a ticket type");
        }
        let unwrapped = cap.unwrap();

        fn as_usize(captures: &regex::Captures, index: usize) -> usize {
            usize::from_str(captures.get(index).unwrap().as_str()).unwrap()
        }
        Ok(TicketType {
            _name: unwrapped.get(1).unwrap().as_str().to_string(),
            valid_ranges: vec![
                as_usize(&unwrapped, 2)..as_usize(&unwrapped, 3),
                as_usize(&unwrapped, 4)..as_usize(&unwrapped, 5),
            ],
        })
    }
}

type Ticket = Vec<usize>;

fn ticket_from_str(s: &str) -> Result<Ticket, &'static str> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("empty string")
    } else {
        Ok(trimmed
            .split(",")
            .map(|s| usize::from_str(s).unwrap())
            .collect())
    }
}

struct Ticketing {
    types: Vec<TicketType>,
    _your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl LoadableFromFile for Ticketing {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("invalid filename");
        let mut reader = BufReader::new(file);

        let mut types = Vec::new();
        let mut buf = String::new();
        loop {
            reader.read_line(&mut buf).expect("read");
            let t = TicketType::from_str(&buf);
            if t.is_err() {
                break;
            }
            types.push(t.unwrap());
            buf.clear();
        }

        reader.read_line(&mut buf).expect("read");
        assert_eq!("\nyour ticket:\n", buf);
        buf.clear();
        reader.read_line(&mut buf).expect("read");
        let your_ticket = ticket_from_str(&buf).unwrap();
        buf.clear();

        reader.read_line(&mut buf).expect("read");
        reader.read_line(&mut buf).expect("read");
        assert_eq!("\nnearby tickets:\n", buf);
        buf.clear();

        let mut tickets = Vec::new();
        loop {
            if reader.read_line(&mut buf).is_err() {
                break;
            }
            let t = ticket_from_str(&buf);
            if t.is_err() {
                break;
            }
            tickets.push(t.unwrap());
            buf.clear();
        }

        Ticketing {
            types: types,
            _your_ticket: your_ticket,
            nearby_tickets: tickets,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SplitResult {
    good_tickets: Vec<Ticket>,
    bad_tickets: Vec<Ticket>,
    error_rate: i64
}

fn split_good_and_bad(ticketing: &Ticketing) -> SplitResult {

    const MAX_INDEX: usize = 1000;
    let mut valid_set = vec![0; MAX_INDEX];
    for t in ticketing.types.iter().enumerate() {
        for r in t.1.valid_ranges.iter() {
            for i in r.start..r.end+1 {
                valid_set[i] = t.0 + 1;
            }
        }
    }

    let mut result = SplitResult::default();
    for n in ticketing.nearby_tickets.iter() {
        let mut in_bad_list = false;
        for t in n {
            if valid_set[*t] == 0 {
                if !in_bad_list {
                    result.bad_tickets.push(n.clone());
                }
                in_bad_list = true;
                result.error_rate += *t as i64;
            }
        }
        if !in_bad_list {
            result.good_tickets.push(n.clone());
        }
    }
    result
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let ticketing = Ticketing::load("input/day_sixteen.txt");
    let split_tickets = split_good_and_bad(&ticketing);
    format!(
        "part one: {}, part two: {}",
        split_tickets.error_rate,
        part_two()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 21996, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        let tickets = Ticketing::load("input/day_sixteen_example.txt");
        let split_tickets = split_good_and_bad(&tickets);
        assert_eq!(71, split_tickets.error_rate);
    }
}
