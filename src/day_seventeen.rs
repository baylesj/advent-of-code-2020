use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::{Matrix2D, Matrix2DLike, Point3D};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

// During a cycle, all cubes simultaneously change their state according to the following rules:

//     If a cube is active and exactly 2 or 3 of its neighbors are also active,
//         the cube remains active. Otherwise, the cube becomes inactive.
//     If a cube is inactive but exactly 3 of its neighbors are active,
//         the cube becomes active. Otherwise, the cube remains inactive.
//
// Strategy: load the original map as Matrix2D of char.
// Option 1: load into a 3D matrix. Then each iteration is O(N), since we just
// have to each field against its neighbors. Growing is a pain this way though.
// I could write a fancy growable matrix but that has its own problems.
//
// Option 2: Keep each "life form" as a Point3D in a list. For each life-form,
// iterate through every other life-form and figure out how many are neighbors.
// Easily infinitely growable, but O(N^2) for each iteration. Since there are
// only six in part one maybe that's okay. Obvious optimizations:
//   1. Can't exit early if we find three neighbors due to "exactly" clause.
//   2. If we kept points sorted on one dimension, then we would know that
//      as soon as that dimension is two away then they aren't neighbors.
//      Worst cast is still O(N^2) if all points are at the same point on
//      that axis, but likely to be faster.
//
// Option 3: Use a hashset of life forms. For each life form, just ask the
// list if each of its neighbors are there--O(N). For generating the iteration,
// hashmap points to number of neighbors, every time we ask about neighbors
// increase value by 1, then reduce based on state rules. Similar memory
// usage to Option 2 due to need for iteration arrays. Obvious choice.
fn get_point_from_index(index: usize, x_width: i64) -> Point3D {
    Point3D {
        x: index as i64 % x_width,
        y: index as i64 / x_width,
        z: 0,
    }
}

const OFFSETS: [i64; 3] = [-1, 0, 1];

fn get_neighbor_offsets() -> Vec<Point3D> {
    let mut offsets = Vec::with_capacity(OFFSETS.len().pow(3) - 1);
    for x in OFFSETS.iter() {
        for y in OFFSETS.iter() {
            for z in OFFSETS.iter() {
                if !(*x == 0 && *y == 0 && *z == 0) {
                    offsets.push(Point3D {
                        x: *x,
                        y: *y,
                        z: *z,
                    });
                }
            }
        }
    }
    offsets
}

lazy_static! {
    static ref NEIGHBOR_OFFSETS: Vec<Point3D> = get_neighbor_offsets();
}

fn run_iteration(lifeforms: &HashSet<Point3D>) -> HashSet<Point3D> {
    let mut possible_life = HashMap::<Point3D, usize>::new();
    for l in lifeforms.iter() {
        for o in NEIGHBOR_OFFSETS.iter() {
            let point = *l + *o;

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
        .filter(|kv| kv.1 == 3 || (kv.1 == 2 && lifeforms.contains(&kv.0)))
        .map(|kv| kv.0)
        .collect()
}

fn part_one(starting_lifeforms: &Matrix2D<char>) -> i64 {
    let mut lifeforms = HashSet::<Point3D>::new();

    println!("starting lifeforms: {}", starting_lifeforms);
    for s in starting_lifeforms.data.iter().enumerate() {
        if *s.1 == '#' {
            lifeforms.insert(get_point_from_index(s.0, starting_lifeforms.size().x));
        }
    }

    for _ in 0..6 {
        lifeforms = run_iteration(&lifeforms);
        println!("after iteration: {:?}\n", lifeforms);
    }
    lifeforms.len() as i64
}

fn part_two() -> i64 {
    0
}

pub fn solve() -> String {
    let starting_lifeforms = Matrix2D::<char>::load("input/day_seventeen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&starting_lifeforms),
        part_two()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 252, part two: 0", solve());
    }

    #[test]
    fn test_example() {
        let starting_lifeforms = Matrix2D::<char>::load("input/day_seventeen_example.txt");
        assert_eq!(112, part_one(&starting_lifeforms));
    }
}
