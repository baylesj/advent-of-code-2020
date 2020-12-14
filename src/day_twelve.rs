use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum MoveAction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl From<&str> for MoveAction {
    fn from(s: &str) -> Self {
        let action_and_value = s.split_at(1);
        let value: i64 = action_and_value.1.parse().unwrap();
        match action_and_value.0 {
            "N" => MoveAction::North(value),
            "S" => MoveAction::South(value),
            "E" => MoveAction::East(value),
            "W" => MoveAction::West(value),
            "L" => MoveAction::Left(value),
            "R" => MoveAction::Right(value),
            "F" => MoveAction::Forward(value),
            _ => panic!("unknown action"),
        }
    }
}

impl LoadableFromFile for Vec<MoveAction> {
    fn load(filename: &str) -> Self {
        let file = File::open(filename).expect("invalid filename");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| MoveAction::from(&l.unwrap()[..]))
            .collect()
    }
}

fn apply_action(action: &MoveAction, point: &mut Point2D, direction: &mut Direction) {
    match action {
        MoveAction::North(value) => point.y += *value,
        MoveAction::South(value) => point.y -= *value,
        MoveAction::East(value) => point.x -= *value,
        MoveAction::West(value) => point.y += *value,
        MoveAction::Left(value) => {
            *direction = match value {
                90 => direction.to_left(),
                180 => direction.inverse(),
                270 => direction.to_right(),
                _ => panic!("moving left: need to refactor this to support more degrees"),
            }
        }
        MoveAction::Right(value) => {
            *direction = match value {
                90 => direction.to_right(),
                180 => direction.inverse(),
                270 => direction.to_left(),
                _ => panic!("moving right: need to refactor this to support more degrees"),
            }
        }
        MoveAction::Forward(value) => point.advance_mult(*direction, *value),
    }
}

pub fn part_one(actions: &[MoveAction]) -> i64 {
    let mut point = Point2D::default();
    // Start facing "east".
    let mut direction = Direction::Left;

    for action in actions {
        apply_action(action, &mut point, &mut direction);
    }

    // TODO: point.magnitude?
    point.x.abs() + point.y.abs()
}

pub fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let actions = Vec::<MoveAction>::load("input/day_twelve.txt");
    format!("part one: {}, part two: {}", part_one(&actions), part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 1424, part two: 0", solve());
    }

    #[test]
    pub fn test_example() {
        let matrix = Vec::<MoveAction>::load("input/day_twelve_example.txt");
        assert_eq!(25, part_one(&matrix));
    }
}
