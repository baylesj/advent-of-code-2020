use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::*;
use std::convert::TryInto;

/*
If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
*/

const EMPTY_SEAT: char = 'L';
const FULL_SEAT: char = '#';
const FLOOR: char = '.';

const NEIGHBORING_UNIT_INDICES: [Point2D; 8] = [
    Point2D { x: -1, y: -1 },
    Point2D { x: -1, y: 0 },
    Point2D { x: -1, y: 1 },
    Point2D { x: 0, y: 1 },
    Point2D { x: 1, y: 1 },
    Point2D { x: 1, y: 0 },
    Point2D { x: 1, y: -1 },
    Point2D { x: 0, y: -1 },
];

fn count_full(matrix: &Matrix2D<char>) -> usize {
    matrix.data.iter().filter(|&c| *c == FULL_SEAT).count()
}

fn in_bounds(point: &Point2D, matrix: &Matrix2D<char>) -> bool {
    0 <= point.x && point.x < matrix.size.x && 0 <= point.y && point.y < matrix.size.y
}

fn full_neighbors(matrix: &Matrix2D<char>, point: &Point2D) -> i8 {
    NEIGHBORING_UNIT_INDICES
        .iter()
        .map(|i| *point + *i)
        .filter(|n| in_bounds(n, matrix) && matrix.get(n) == FULL_SEAT)
        .count()
        .try_into()
        .unwrap()
}

fn full_visibles(matrix: &Matrix2D<char>, point: &Point2D) -> i8 {
    let mut count = 0;
    let mut current;
    for index in NEIGHBORING_UNIT_INDICES.iter() {
        current = *point + *index;
        // We only care about the first visible seat (meaning first non-floor).
        while in_bounds(&current, matrix) && matrix.get(&current) == FLOOR {
            current += *index;
        }
        if in_bounds(&current, matrix) && matrix.get(&current) == FULL_SEAT {
            count += 1;
        }
    }
    count
}

fn prep(matrix: &mut Matrix2D<char>) {
    for c in matrix.data.iter_mut() {
        match *c {
            FULL_SEAT => panic!("don't prep already running matrices"),
            EMPTY_SEAT => *c = FULL_SEAT,
            _ => (),
        }
    }
}

fn swap_if_should(
    matrix: &Matrix2D<char>,
    next: &mut Matrix2D<char>,
    index: &Point2D,
    only_immediate: bool,
) -> bool {
    let seat = matrix.get(index);
    if seat != EMPTY_SEAT && seat != FULL_SEAT {
        return false;
    }

    let neighbor_count;
    if only_immediate {
        neighbor_count = full_neighbors(matrix, index);
    } else {
        neighbor_count = full_visibles(matrix, index);
    }

    if seat == EMPTY_SEAT {
        if neighbor_count == 0 {
            next.set(index, FULL_SEAT);
            return true;
        }
    } else if seat == FULL_SEAT {
        // TODO: refactor parts one and two into more elegant solution.
        let required = if only_immediate { 4 } else { 5 };
        if neighbor_count >= required {
            next.set(index, EMPTY_SEAT);
            return true;
        }
    }
    false
}

fn run_iter(matrix: &Matrix2D<char>, next: &mut Matrix2D<char>, only_immediate: bool) -> bool {
    let mut xy;
    let mut something_changed = false;
    for x in 0..matrix.size.x {
        for y in 0..matrix.size.y {
            xy = Point2D { x: x, y: y };
            something_changed |= swap_if_should(matrix, next, &xy, only_immediate);
        }
    }

    something_changed
}

pub fn run_until_halted(matrix: &mut Matrix2D<char>, only_immediate: bool) {
    prep(matrix);
    let mut next = matrix.clone();
    while run_iter(matrix, &mut next, only_immediate) {
        *matrix = next.clone();
    }
}

pub fn part_one(matrix: &Matrix2D<char>) -> i64 {
    let mut m = matrix.clone();
    run_until_halted(&mut m, true);
    count_full(&m) as i64
}

pub fn part_two(matrix: &Matrix2D<char>) -> i64 {
    let mut m = matrix.clone();
    run_until_halted(&mut m, false);
    count_full(&m) as i64
}

pub fn solve() -> String {
    let matrix = Matrix2D::<char>::load("input/day_eleven.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&matrix),
        part_two(&matrix)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 2438, part two: 2174", solve());
    }

    #[test]
    pub fn test_example() {
        let matrix = Matrix2D::<char>::load("input/day_eleven_example.txt");
        assert_eq!(37, part_one(&matrix));
    }

    #[test]
    pub fn test_example_part_two() {
        let matrix = Matrix2D::<char>::load("input/day_eleven_example.txt");
        assert_eq!(26, part_two(&matrix));
    }
}
