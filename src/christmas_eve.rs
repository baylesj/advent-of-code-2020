use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::Point3D;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

// e, se, sw, w, nw, and ne
#[derive(Debug, Copy, Clone, PartialEq)]
enum HexDirection {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq)]
enum State {
    SawNorth,
    SawSouth,
    Normal,
}

#[derive(Default, Debug)]
struct Instructions {
    value: Vec<HexDirection>,
}

impl FromStr for Instructions {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = Instructions::default();
        let mut state = State::Normal;
        for c in s.chars() {
            let mut next = HexDirection::East;
            match c {
                'e' => {
                    match state {
                        State::SawNorth => next = HexDirection::NorthEast,
                        State::SawSouth => next = HexDirection::SouthEast,
                        State::Normal => next = HexDirection::East,
                    }
                    state = State::Normal;
                }
                'w' => {
                    match state {
                        State::SawNorth => next = HexDirection::NorthWest,
                        State::SawSouth => next = HexDirection::SouthWest,
                        State::Normal => next = HexDirection::West,
                    }
                    state = State::Normal;
                }
                'n' => state = State::SawNorth,
                's' => state = State::SawSouth,
                _ => return Err("invalid character"),
            }

            if state == State::Normal {
                instructions.value.push(next);
            }
        }

        Ok(instructions)
    }
}

impl LoadableFromFile for Vec<Instructions> {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);

        reader
            .lines()
            .map(|l| Instructions::from_str(&l.unwrap()).unwrap())
            .collect()
    }
}

fn advance(loc: &Point3D, dir: &HexDirection) -> Point3D {
    let mut location = *loc;
    match dir {
        HexDirection::East => {
            location.x += 1;
            location.y -= 1;
        }
        HexDirection::West => {
            location.x -= 1;
            location.y += 1;
        }
        HexDirection::NorthEast => {
            location.z -= 1;
            location.x += 1;
        }
        HexDirection::NorthWest => {
            location.z -= 1;
            location.y += 1;
        }
        HexDirection::SouthEast => {
            location.y -= 1;
            location.z += 1;
        }
        HexDirection::SouthWest => {
            location.x -= 1;
            location.z += 1;
        }
    }
    location
}

// We can traverse using a hexagonal grid based on 3D coordinates:
// https://www.redblobgames.com/grids/hexagons/#coordinates
fn part_one(instructions_list: &[Instructions]) -> HashSet<Point3D> {
    let mut flipped_black = HashSet::new();
    for instructions in instructions_list.iter() {
        let mut cur = Point3D::default();
        for direction in instructions.value.iter() {
            let next = advance(&cur, direction);
            cur = next;
        }
        if flipped_black.contains(&cur) {
            flipped_black.remove(&cur);
        } else {
            flipped_black.insert(cur);
        }
    }
    flipped_black
}

const NEIGHBOR_OFFSETS_HEX: [HexDirection; 6] = [
    HexDirection::East,
    HexDirection::NorthEast,
    HexDirection::NorthWest,
    HexDirection::West,
    HexDirection::SouthWest,
    HexDirection::SouthEast,
];

// TODO: refactor to share from day 17.
fn run_iteration(
    lifeforms: &HashSet<Point3D>,
    neighbor_offsets: &[HexDirection],
) -> HashSet<Point3D> {
    let mut possible_life = HashMap::<Point3D, usize>::new();
    for l in lifeforms.iter() {
        for d in neighbor_offsets.iter() {
            let point = advance(&l, &d);

            let entry;
            if lifeforms.contains(&point) {
                // Neighbor is already alive, so modify this life-form. The
                // neighbor will be updated with this life form later.
                entry = possible_life.entry(*l).or_insert(0);
            } else {
                // Neighbor is not alive yet, so doesn't impact this life-form,
                // but may become alive itself.
                entry = possible_life.entry(point).or_insert(0);
            }
            *entry += 1;
        }
    }

    possible_life
        .into_iter()
        // If a tile is white, it will flip to black if it has two
        // black neighbors.
        // If a tile is black, this is also true (so color doesn't matter),
        // but it also stays black if it has 1 black neighbor.
        .filter(|kv| kv.1 == 2 || (kv.1 == 1 && lifeforms.contains(&kv.0)))
        .map(|kv| kv.0)
        .collect()
}

// Part two is basically Conways' Game of Life again.
fn part_two(flipped_day_one: &HashSet<Point3D>) -> i64 {
    let mut flipped: HashSet<Point3D> = flipped_day_one.clone();
    for i in 1..101 {
        flipped = run_iteration(&flipped, &NEIGHBOR_OFFSETS_HEX);
    }
    flipped.len() as i64
}

pub fn solve() -> String {
    let instructions_list = Vec::<Instructions>::load("input/day_twentyfour.txt");
    let flipped_day_one = part_one(&instructions_list);
    format!(
        "part one: {}, part two: {}",
        flipped_day_one.len(),
        part_two(&flipped_day_one)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 465, part two: 4078", solve());
    }

    #[test]
    fn test_example() {
        let il = Vec::<Instructions>::load("input/day_twentyfour_example.txt");

        let flipped_day_one = part_one(&il);
        assert_eq!(10, flipped_day_one.len());
        assert_eq!(2208, part_two(&flipped_day_one));
    }
}
