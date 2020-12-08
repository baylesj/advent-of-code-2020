use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const INPUT_FILENAME: &'static str = "input/day_one.txt";

type Mass = u32;
type Fuel = u32;

fn get_fuel_for_module(mass: Mass) -> Fuel {
    let mut total_fuel: Fuel = 0;

    let mut current_step_mass: Mass = mass;
    while current_step_mass != 0 {
        current_step_mass = get_fuel_for_module_step(current_step_mass);
        total_fuel += current_step_mass as Fuel;
    }

    total_fuel
}

fn get_fuel_for_module_step(mass: Mass) -> Fuel {
    let one_third: Fuel = (mass / 3) as Fuel;
    if one_third > 2 {
        one_third - 2
    } else {
        0
    }
}

pub fn solve() -> String {
    let file = File::open(INPUT_FILENAME).expect("Invalid filename");
    let reader = BufReader::new(file);

    let mut part_one_sum: Fuel = 0;
    let mut part_two_sum: Fuel = 0;
    for line in reader.lines() {
        let unparsed_mass: String = line.expect("Invalid file contents");
        let module_mass: Mass = unparsed_mass
            .parse::<Mass>()
            .expect("Invalid file contents");
        part_one_sum += get_fuel_for_module_step(module_mass);
        part_two_sum += get_fuel_for_module(module_mass);
    }

    format!("part one: {}, part two: {}", part_one_sum, part_two_sum)
}
