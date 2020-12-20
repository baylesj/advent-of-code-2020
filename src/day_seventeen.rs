use crate::loadable::LoadableFromFile;
use crate::yet_another_geometry_mod::{Matrix2D, Matrix2DLike, Point2D, Point3D, Point4D};
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

fn get_as_point(index: usize, x_width: i64) -> Point2D {
    Point2D {
        x: index as i64 % x_width,
        y: index as i64 / x_width,
    }
}

const OFFSETS: [i64; 3] = [-1, 0, 1];

fn get_neighbor_offsets_3d() -> Vec<Point3D> {
    let mut offsets = Vec::with_capacity(OFFSETS.len().pow(3));
    for xn in OFFSETS.iter() {
        for yn in OFFSETS.iter() {
            for zn in OFFSETS.iter() {
                if *xn != 0 || *yn != 0 || *zn != 0 {
                    offsets.push(Point3D {
                        x: *xn,
                        y: *yn,
                        z: *zn,
                    });
                }
            }
        }
    }
    offsets
}

fn get_neighbor_offsets_4d() -> Vec<Point4D> {
    let mut offsets = Vec::with_capacity(OFFSETS.len().pow(3));
    for xn in OFFSETS.iter() {
        for yn in OFFSETS.iter() {
            for zn in OFFSETS.iter() {
                for wn in OFFSETS.iter() {
                    if *xn != 0 || *yn != 0 || *zn != 0 || *wn != 0 {
                        offsets.push(Point4D {
                            x: *xn,
                            y: *yn,
                            z: *zn,
                            w: *wn,
                        });
                    }
                }
            }
        }
    }
    offsets
}

fn run_iteration<P: PartialEq + Eq + Hash + Add + Copy>(
    lifeforms: &HashSet<P>,
    neighbor_offsets: &Vec<P>,
) -> HashSet<P>
where
    P: Add<Output = P>,
{
    let mut possible_life = HashMap::<P, usize>::new();
    for l in lifeforms.iter() {
        for o in neighbor_offsets.iter() {
            let point: P = *l + *o;

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

    for s in starting_lifeforms.data.iter().enumerate() {
        if *s.1 == '#' {
            let p = get_as_point(s.0, starting_lifeforms.size().x);
            lifeforms.insert(Point3D {
                x: p.x,
                y: p.y,
                z: 0,
            });
        }
    }

    let offsets = get_neighbor_offsets_3d();
    for _ in 0..6 {
        lifeforms = run_iteration(&lifeforms, &offsets);
    }
    lifeforms.len() as i64
}

// Part two is the same as part one, except four dimensional.
fn part_two(starting_lifeforms: &Matrix2D<char>) -> i64 {
    let mut lifeforms = HashSet::<Point4D>::new();

    for s in starting_lifeforms.data.iter().enumerate() {
        if *s.1 == '#' {
            let p = get_as_point(s.0, starting_lifeforms.size().x);
            lifeforms.insert(Point4D {
                x: p.x,
                y: p.y,
                z: 0,
                w: 0,
            });
        }
    }

    let offsets = get_neighbor_offsets_4d();
    for _ in 0..6 {
        lifeforms = run_iteration(&lifeforms, &offsets);
    }
    lifeforms.len() as i64
}

pub fn solve() -> String {
    let starting_lifeforms = Matrix2D::<char>::load("input/day_seventeen.txt");
    format!(
        "part one: {}, part two: {}",
        part_one(&starting_lifeforms),
        part_two(&starting_lifeforms)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!("part one: 252, part two: 2160", solve());
    }

    #[test]
    fn test_example() {
        let starting_lifeforms = Matrix2D::<char>::load("input/day_seventeen_example.txt");
        assert_eq!(112, part_one(&starting_lifeforms));
    }

    #[test]
    fn test_example_part_two() {
        let starting_lifeforms = Matrix2D::<char>::load("input/day_seventeen_example.txt");
        assert_eq!(848, part_two(&starting_lifeforms));
    }
}
