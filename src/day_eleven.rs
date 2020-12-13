use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::*;

/*
If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
Otherwise, the seat's state does not change.
*/

const EMPTY_SEAT: char = 'L';
const FULL_SEAT: char = '#';

fn count_full(mat: &Matrix2D<char>) -> usize {
    mat.data.iter().filter(|&c| *c == FULL_SEAT).count()
}

fn neighbors_are(mat: &Matrix2D<char>, point: &Point2D, c: char) -> i64 {
    let mut count = 0;
    // . . *
    // . o *
    // . . *
    if point.x < mat.size.x - 1 {
        // . . .
        // . o *
        // . . .
        if mat.get(&Point2D {
            x: point.x + 1,
            y: point.y,
        }) == c
        {
            count += 1;
        }
        // . . *
        // . o .
        // . . .
        if point.y > 0
            && mat.get(&Point2D {
                x: point.x + 1,
                y: point.y - 1,
            }) == c
        {
            count += 1;
        }
        // . . .
        // . o .
        // . . *
        if point.y < mat.size.y - 1
            && mat.get(&Point2D {
                x: point.x + 1,
                y: point.y + 1,
            }) == c
        {
            count += 1;
        }
    }

    // * . .
    // * o .
    // * . .
    if point.x > 0 {
        // . . .
        // * o .
        // . . .
        if mat.get(&Point2D {
            x: point.x - 1,
            y: point.y,
        }) == c
        {
            count += 1;
        }
        // * . .
        // . o .
        // . . .
        if point.y > 0
            && mat.get(&Point2D {
                x: point.x - 1,
                y: point.y - 1,
            }) == c
        {
            count += 1;
        }
        // . . .
        // . o .
        // * . .
        if point.y < mat.size.y - 1
            && mat.get(&Point2D {
                x: point.x - 1,
                y: point.y + 1,
            }) == c
        {
            count += 1;
        }
    }
    // . . .
    // . o .
    // . * .
    if point.y < mat.size.y - 1
        && mat.get(&Point2D {
            x: point.x,
            y: point.y + 1,
        }) == c
    {
        count += 1;
    }
    // . * .
    // . o .
    // . . .
    if point.y > 0
        && mat.get(&Point2D {
            x: point.x,
            y: point.y - 1,
        }) == c
    {
        count += 1;
    }
    count
}

fn prep(mat: &mut Matrix2D<char>) {
    for c in mat.data.iter_mut() {
        match *c {
            FULL_SEAT => panic!("don't prep already running matrices"),
            EMPTY_SEAT => *c = FULL_SEAT,
            _ => (),
        }
    }
}

fn swap_if_should(mat: &Matrix2D<char>, next: &mut Matrix2D<char>, index: &Point2D) -> bool {
    let neighbor_count = neighbors_are(mat, index, FULL_SEAT);
    let seat = mat.get(index);
    if seat == EMPTY_SEAT {
        if neighbor_count == 0 {
            next.set(index, FULL_SEAT);
            return true;
        }
    } else if seat == FULL_SEAT {
        if neighbor_count >= 4 {
            next.set(index, EMPTY_SEAT);
            return true;
        }
    }
    false
}

fn run_iter(mat: &Matrix2D<char>, next: &mut Matrix2D<char>) -> bool {
    let mut xy;
    let mut something_changed = false;
    for x in 0..mat.size.x {
        for y in 0..mat.size.y {
            xy = Point2D { x: x, y: y };
            something_changed |= swap_if_should(mat, next, &xy);
        }
    }

    something_changed
}

pub fn part_one(filename: &str) -> i64 {
    let mut mat = Matrix2D::<char>::load(filename);
    prep(&mut mat);
    let mut next = mat.clone();
    while run_iter(&mat, &mut next) {
        mat = next.clone();
    }

    count_full(&mat) as i64
}

pub fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    format!(
        "part one: {}, part two: {}",
        part_one("input/day_eleven.txt"),
        part_two()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_solve() {
        assert_eq!("part one: 2438, part two: 0", solve());
    }

    #[test]
    pub fn test_example_one() {
        assert_eq!(37, part_one("input/day_eleven_example.txt"));
    }
}
