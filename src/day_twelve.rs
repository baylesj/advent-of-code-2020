use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::Sum;

#[path = "loadable.rs"]
mod loadable;
use loadable::LoadableFromFile;

const INPUT_FILENAME: &'static str = "input/day_twelve.txt";

#[derive(Debug, PartialEq, Eq, Default, Hash, Copy, Clone)]
pub struct Point3D {
  pub x: i64,
  pub y: i64,
  pub z: i64
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Moon {
  pub position: Point3D,
  pub velocity: Point3D
}

#[derive(Debug, Default, Clone)]
pub struct OrbitalSystem {
  pub moons: Vec<Moon>
}

impl LoadableFromFile for OrbitalSystem {
  fn load(filename: &str) -> OrbitalSystem {
    let file = File::open(filename).expect("orbital system file open");
    let reader = BufReader::new(file);

    let mut orbital_system = OrbitalSystem::default();
    for line in reader.lines() {
      const CHARS_TO_TRIM: &[char] = &['<', '>', 'x', 'y', 'z', '=', ' '];
      let fields: Vec<i64> = line.unwrap().split(',').
      map(|m| {
        m.trim_matches(|c: char| {CHARS_TO_TRIM.contains(&c)}).parse::<i64>()
        .expect("invalid point")
      }).collect();
      let mut new_moon = Moon::default();
      new_moon.position = Point3D{ x: fields[0], y: fields[1], z: fields[2]};
      orbital_system.moons.push(new_moon);
    }
    orbital_system
  }
}

trait Stepping {
  fn take_steps(self: &mut Self, step_count: i64);
  fn step(self: &mut Self);
}

impl Stepping for OrbitalSystem {
  fn take_steps(self: &mut Self, step_count: i64) {
    for _ in 0..step_count {
      self.step();
    }
  }

  fn step(self: &mut Self) {

  }
}

trait EnergySummation {
  fn sum_total_energy(self: &Self) -> i64;
}

impl EnergySummation for Point3D {
  fn sum_total_energy(self: &Self) -> i64 {
    self.x.abs() + self.y.abs() + self.z.abs()
  }
}

impl EnergySummation for Moon {
  fn sum_total_energy(self: &Self) -> i64 {
    self.position.sum_total_energy() + self.velocity.sum_total_energy()
  }
}

impl EnergySummation for OrbitalSystem {
  fn sum_total_energy(self: &Self) -> i64 {
    i64::sum(self.moons.iter().map(|m| m.sum_total_energy()))
  }
}

pub fn part_one(initial_system: &OrbitalSystem, steps: i64) -> i64 {
    let mut live_system = initial_system.clone();
    live_system.take_steps(steps);
    live_system.sum_total_energy()
}

pub fn solve() {
    let initial_system = OrbitalSystem::load(INPUT_FILENAME);
    println!(
        "Day twelve, part one: {}, part two: {}",
        part_one(&initial_system, 1000), 0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_system_load() {
      let os = OrbitalSystem::load("input/day_twelve_sample_one.txt");
      assert_eq!(os.moons[0].position, Point3D{x:-1, y:0, z:2});
      assert_eq!(os.moons[1].position, Point3D{x:2, y:-10, z:-7});
      assert_eq!(os.moons[2].position, Point3D{x:4, y:-8, z:8});
      assert_eq!(os.moons[3].position, Point3D{x:3, y:5, z:-1});
    }

    #[test]
    fn test_sum_total_energy() {
      let point_a = Point3D::default();
      let point_b = Point3D{x: 1, y: 2, z: 3};
      let point_c = Point3D{x: 4, y: 5, z: 6};
      assert_eq!(0, point_a.sum_total_energy());
      assert_eq!(6, point_b.sum_total_energy());
      let moon = Moon{position: point_b, velocity: point_c};
      assert_eq!(21, moon.sum_total_energy());
      assert_eq!(63, OrbitalSystem{ moons: vec![moon, moon, moon]}.sum_total_energy());
    }

    #[test]
    fn test_part_one_sample_one() {
      let _os = OrbitalSystem::load("input/day_twelve_sample_one.txt");
    }
}
