use num::integer::lcm;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Sum;
use std::ops::Add;

#[path = "loadable.rs"]
mod loadable;
use loadable::LoadableFromFile;

const INPUT_FILENAME: &'static str = "input/day_twelve.txt";

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point3D {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

pub trait ArrayLike {
    fn size() -> usize;
    fn get(&self, i: usize) -> i128;
    fn set(&mut self, i: usize, v: i128);
}

impl ArrayLike for Point3D {
    fn size() -> usize {
        3
    }

    fn get(&self, i: usize) -> i128 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("out of bounds"),
        }
    }

    fn set(&mut self, i: usize, v: i128) {
        match i {
            0 => self.x = v,
            1 => self.y = v,
            2 => self.z = v,
            _ => panic!("out of bounds"),
        }
    }
}

impl Add for Point3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Moon {
    pub position: Point3D,
    pub velocity: Point3D,
}

impl Display for Moon {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct OrbitalSystem {
    pub moons: Vec<Moon>,
    pub step_count: i128,
}

impl Display for OrbitalSystem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Moons:\n").ok();
        for moon in &self.moons {
            write!(f, "{}\n", moon).ok();
        }
        Ok(())
    }
}

impl LoadableFromFile for OrbitalSystem {
    fn load(filename: &str) -> OrbitalSystem {
        let file = File::open(filename).expect("orbital system file open");
        let reader = BufReader::new(file);

        let mut orbital_system = OrbitalSystem::default();
        for line in reader.lines() {
            const CHARS_TO_TRIM: &[char] = &['<', '>', 'x', 'y', 'z', '=', ' '];
            let fields: Vec<i128> = line
                .unwrap()
                .split(',')
                .map(|m| {
                    m.trim_matches(|c: char| CHARS_TO_TRIM.contains(&c))
                        .parse::<i128>()
                        .expect("invalid point")
                })
                .collect();
            let mut new_moon = Moon::default();
            new_moon.position = Point3D {
                x: fields[0],
                y: fields[1],
                z: fields[2],
            };
            orbital_system.moons.push(new_moon);
        }
        orbital_system
    }
}

trait TakeSteps {
    fn take_steps(&mut self, step_count: i128);
    fn take_step(&mut self);
}

impl TakeSteps for OrbitalSystem {
    fn take_steps(&mut self, step_count: i128) {
        for _ in 0..step_count {
            self.take_step();
        }
    }

    fn take_step(&mut self) {
        fn adj(l: i128, r: i128) -> i128 {
            if l < r {
                1
            } else if l > r {
                -1
            } else {
                0
            }
        }

        for i in 0..self.moons.len() {
            for j in i..self.moons.len() {
                for k in 0..Point3D::size() {
                    let adj: i128 =
                        adj(self.moons[i].position.get(k), self.moons[j].position.get(k));
                    let nvi = self.moons[i].velocity.get(k) + adj;
                    let nvj = self.moons[j].velocity.get(k) - adj;
                    self.moons[i].velocity.set(k, nvi);
                    self.moons[j].velocity.set(k, nvj);
                }
            }
        }

        for moon in self.moons.iter_mut() {
            moon.position = moon.position + moon.velocity;
        }

        self.step_count += 1;
    }
}

trait DimensionSlice {
    fn dimension_slice(&self, dimension: usize) -> Vec<i128>;
    fn dimension_equals(&self, dimension: usize, slice: &Vec<i128>) -> bool;
}

impl DimensionSlice for OrbitalSystem {
    fn dimension_slice(self: &Self, dimension: usize) -> Vec<i128> {
        self.moons
            .iter()
            .map(|m| vec![m.position.get(dimension), m.velocity.get(dimension)])
            .flat_map(|v| v.into_iter())
            .collect()
    }

    fn dimension_equals(&self, dimension: usize, slice: &Vec<i128>) -> bool {
        let mut slice_iter = slice.iter();

        self.moons.iter().all(|m| {
            m.position.get(dimension) == *slice_iter.next().unwrap()
                && m.velocity.get(dimension) == *slice_iter.next().unwrap()
        })
    }
}

trait SumTotalEnergy {
    fn sum_total_energy(self: &Self) -> i128;
}

impl SumTotalEnergy for Point3D {
    fn sum_total_energy(self: &Self) -> i128 {
        self.x.abs() as i128 + self.y.abs() as i128 + self.z.abs() as i128
    }
}

impl SumTotalEnergy for Moon {
    fn sum_total_energy(self: &Self) -> i128 {
        self.position.sum_total_energy() * self.velocity.sum_total_energy()
    }
}

impl SumTotalEnergy for OrbitalSystem {
    fn sum_total_energy(self: &Self) -> i128 {
        i128::sum(self.moons.iter().map(|m| m.sum_total_energy()))
    }
}

pub fn part_one(initial_system: &OrbitalSystem, steps: i128) -> i128 {
    let mut live_system = initial_system.clone();
    live_system.take_steps(steps);
    live_system.sum_total_energy()
}

pub fn part_two(initial_system: &OrbitalSystem) -> i128 {
    let mut live_system = initial_system.clone();
    let mut slices = Vec::new();
    for i in 0..Point3D::size() {
        slices.push(initial_system.dimension_slice(i));
    }

    // HINT: use the LCM. Assuming all orbits are periodic, we just need
    // to find the least common multiple of periodicity.
    let mut periods = vec![0; Point3D::size()];
    while periods.iter().any(|p| *p == 0) {
        live_system.take_step();
        for i in 0..Point3D::size() {
            if periods[i] == 0 && live_system.dimension_equals(i, &slices[i]) {
                periods[i] = live_system.step_count;
            }
        }
    }
    lcm(lcm(periods[0], periods[1]), periods[2])
}

pub fn solve() {
    let initial_system = OrbitalSystem::load(INPUT_FILENAME);
    println!(
        "Day twelve, part one: {}, part two: {}",
        part_one(&initial_system, 1000),
        part_two(&initial_system)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_system_load() {
        let os = OrbitalSystem::load("input/day_twelve_sample_one.txt");
        assert_eq!(os.moons[0].position, Point3D { x: -1, y: 0, z: 2 });
        assert_eq!(
            os.moons[1].position,
            Point3D {
                x: 2,
                y: -10,
                z: -7
            }
        );
        assert_eq!(os.moons[2].position, Point3D { x: 4, y: -8, z: 8 });
        assert_eq!(os.moons[3].position, Point3D { x: 3, y: 5, z: -1 });
    }

    #[test]
    fn test_sum_total_energy() {
        let point_a = Point3D::default();
        let point_b = Point3D { x: 1, y: 2, z: 3 };
        let point_c = Point3D { x: 4, y: 5, z: 6 };
        assert_eq!(0, point_a.sum_total_energy());
        assert_eq!(6, point_b.sum_total_energy());
        let moon = Moon {
            position: point_b,
            velocity: point_c,
        };
        assert_eq!(90, moon.sum_total_energy());
        assert_eq!(
            270,
            OrbitalSystem {
                moons: vec![moon, moon, moon],
                step_count: 0
            }
            .sum_total_energy()
        );
    }

    #[test]
    fn test_part_one_sample_one() {
        let os = OrbitalSystem::load("input/day_twelve_sample_one.txt");
        assert_eq!(179, part_one(&os, 10));
    }

    #[test]
    fn test_part_one_sample_two() {
        let os = OrbitalSystem::load("input/day_twelve_sample_two.txt");
        assert_eq!(1940, part_one(&os, 100));
    }

    #[test]
    fn test_part_one() {
        let os = OrbitalSystem::load("input/day_twelve.txt");
        assert_eq!(6423, part_one(&os, 1000));
    }

    #[test]
    fn test_part_two() {
        let os = OrbitalSystem::load("input/day_twelve.txt");
        assert_eq!(327636285682704, part_two(&os));
    }
}
