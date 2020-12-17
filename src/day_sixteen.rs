use crate::loadable::LoadableFromFile;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
struct TicketField {
    name: String,
    valid_ranges: Vec<Range<usize>>,
}

fn contains(t: &TicketField, i: usize) -> bool {
    for r in t.valid_ranges.iter() {
        if r.start <= i && i <= r.end {
            return true;
        }
    }
    false
}

impl FromStr for TicketField {
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
        Ok(TicketField {
            name: unwrapped.get(1).unwrap().as_str().to_string(),
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
    fields: Vec<TicketField>,
    your_ticket: Ticket,
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
            let t = TicketField::from_str(&buf);
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
            fields: types,
            your_ticket: your_ticket,
            nearby_tickets: tickets,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct SplitResult {
    good_tickets: Vec<Ticket>,
    bad_tickets: Vec<Ticket>,
    error_rate: i64,
    ticket_domain: Vec<HashSet<usize>>,
}

fn split_good_and_bad(ticketing: &Ticketing) -> SplitResult {
    const MAX_INDEX: usize = 1000;
    let mut result = SplitResult::default();
    result.ticket_domain = vec![HashSet::<usize>::new(); MAX_INDEX];

    // Building the ticket domain is pretty costly, at O(|F|*|R|*|R_L|).
    //      |F| = number of fields on each ticket
    //      |T| = number of tickets.
    //      |R| = integer range of potential ticket field values
    //      |R_L| = number of integer ranges (currently fixed at 2).
    for t in ticketing.fields.iter().enumerate() {
        for r in t.1.valid_ranges.iter() {
            for i in r.start..r.end + 1 {
                result.ticket_domain[i].insert(t.0);
            }
        }
    }

    // Now that we have a valid ticket domain, we need to filter tickets by
    // whether they are valid, meaning that each ticket entry has a value
    // from a valid ticket field. This part is O(|T|*|F|) worst case.
    for nearby_ticket in ticketing.nearby_tickets.iter() {
        let mut in_bad_list = false;
        for field_value in nearby_ticket {
            if result.ticket_domain[*field_value].is_empty() {
                if !in_bad_list {
                    result.bad_tickets.push(nearby_ticket.clone());
                }
                in_bad_list = true;
                result.error_rate += *field_value as i64;
            }
        }
        if !in_bad_list {
            result.good_tickets.push(nearby_ticket.clone());
        }
    }
    assert_eq!(
        ticketing.nearby_tickets.len(),
        result.good_tickets.len() + result.bad_tickets.len()
    );
    result
}

fn part_two(ticketing: &Ticketing, split: &SplitResult) -> i64 {
    // First we have to determine what TicketField values occupy each
    // spot on the ticket. This part of the algorithm is O(|F|*|T|*|F|) worst
    // case. Since at the very beginning of this algorithm any field could
    // occupy any spot on the ticket, I think this is best case for CPU usage.
    let mut candidate_fields = vec![HashSet::new(); ticketing.your_ticket.len()];
    for i in 0..ticketing.your_ticket.len() {
        let mut candidates = split.ticket_domain[split.good_tickets[0][i]].clone();
        for ticket in split.good_tickets.iter() {
            for candidate in candidates.clone().iter() {
                let candidate_type = &ticketing.fields[*candidate];
                if !contains(&candidate_type, ticket[i]) {
                    candidates.remove(candidate);
                }
            }
        }
        candidate_fields[i] = candidates;
    }

    // NOTE: at least one of the ticket fields must be known in order to
    // perform a reduction.
    let mut known_fields = HashSet::new();
    for t in candidate_fields.iter() {
        if t.len() == 1 {
            known_fields.insert(*t.iter().next().unwrap());
        }
    }

    // Now we have to loop through all of the candidates to reduce known
    // fields. Worst case here is O(|F|*|F|*|F|), which is not awesome.
    // Note that |F| is relatively small in comparison to |T| in our data set.
    loop {
        let mut changed_something = false;
        for i in 0..candidate_fields.len() {
            if candidate_fields[i].len() > 1 {
                changed_something = true;
                for candidate in candidate_fields[i].clone() {
                    if candidate_fields[i].len() > 1 && known_fields.contains(&candidate) {
                        candidate_fields[i].remove(&candidate);
                        if candidate_fields[i].len() == 1 {
                            known_fields.insert(*candidate_fields[i].iter().next().unwrap());
                            break;
                        }
                    }
                }
            }
        }
        if !changed_something {
            break;
        }
    }

    // Taking a step back, we obviously have to check every ticket value,
    // and we have to keep track of what ticket fields support what ranges.
    // so the hypothetical minimum for CPU runtime would be O(|F|*|T|) if we could
    // get constant lookup for ticket fields, which I don't think we can in the
    // best case.
    // Since |T| >(>?) than |F|, we really want to minimize |T| traversals where
    // possible.
    let mut result = 1;
    for i in 0..ticketing.your_ticket.len() {
        assert_eq!(1, candidate_fields[i].len());
        if ticketing.fields[*candidate_fields[i].iter().next().unwrap()]
            .name
            .starts_with("departure")
        {
            result *= ticketing.your_ticket[i] as i64;
        }
    }
    result
}

pub fn solve() -> String {
    let ticketing = Ticketing::load("input/day_sixteen.txt");
    let split_tickets = split_good_and_bad(&ticketing);
    format!(
        "part one: {}, part two: {}",
        split_tickets.error_rate,
        part_two(&ticketing, &split_tickets)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 21996, part two: 650080463519", solve());
    }

    #[test]
    fn test_example() {
        let tickets = Ticketing::load("input/day_sixteen_example.txt");
        let split_tickets = split_good_and_bad(&tickets);
        assert_eq!(71, split_tickets.error_rate);
    }
}
