use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &str = "./src/day_one_input.txt";

type Mass = u32;
type Fuel = u32;

fn get_fuel_for_module(mass: Mass) -> Fuel {
    ((mass as f32 / 3.0) - 2.0).trunc() as Fuel
}

pub fn solve() -> String {
    let file = File::open(INPUT_FILENAME).expect("Invalid filename");
    let reader = BufReader::new(file);

    let mut sum: Fuel = 0;
    for line in reader.lines() {
      let unparsed_mass: String = line.expect("Invalid file contents");
      let module_mass: Mass = unparsed_mass.parse::<Mass>()
          .expect("Invalid file contents");
      sum += get_fuel_for_module(module_mass);
    }
    sum.to_string()
}
