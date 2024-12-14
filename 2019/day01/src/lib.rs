// Copyright 2024 Jedrzej Stuczynski
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use aoc_solution::Aoc;

#[derive(Aoc)]
pub struct Day01;

struct FuelCalculator {}

impl FuelCalculator {
    fn required_fuel(mass: u64) -> Option<u64> {
        match (mass as f64 / 3.0).floor() - 2.0 {
            f if f <= 0.0 => None,
            f => Some(f as u64),
        }
    }
}

trait Fuelable {
    fn calculate_base_required_fuel(&self) -> u64;
    fn calculate_total_required_fuel(&self) -> u64;
}

pub struct Module {
    mass: u64,
}

impl Module {
    fn new(mass: u64) -> Self {
        Self { mass }
    }
}

impl Fuelable for Module {
    fn calculate_base_required_fuel(&self) -> u64 {
        FuelCalculator::required_fuel(self.mass).unwrap()
    }

    fn calculate_total_required_fuel(&self) -> u64 {
        let f = FuelCalculator::required_fuel;
        std::iter::successors(f(self.mass), |x| f(*x)).sum()
    }
}

struct FuelUpper {}

impl FuelUpper {
    fn determine_total_required_base_fuel<F: Fuelable>(fuelables: &[F]) -> u64 {
        fuelables
            .iter()
            .map(|f| f.calculate_base_required_fuel())
            .sum()
    }

    fn determine_total_required_fuel<F: Fuelable>(fuelables: &[F]) -> u64 {
        fuelables
            .iter()
            .map(|f| f.calculate_total_required_fuel())
            .sum()
    }
}

pub fn input_to_modules(inputs: Vec<String>) -> Vec<Module> {
    inputs
        .iter()
        .map(|i| i.parse::<u64>().unwrap())
        .map(Module::new)
        .collect()
}

pub fn do_part1(inputs_modules: &[Module]) {
    let required_base_fuel = FuelUpper::determine_total_required_base_fuel(inputs_modules);
    println!("Part 1 answer: {}", required_base_fuel);
}

pub fn do_part2(input_modules: &[Module]) {
    let required_total_fuel = FuelUpper::determine_total_required_fuel(input_modules);
    println!("Part 2 answer: {}", required_total_fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_12() {
        assert_eq!(2, Module::new(12).calculate_base_required_fuel(),);
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_14() {
        assert_eq!(2, Module::new(14).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_1969() {
        assert_eq!(654, Module::new(1969).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_base_fuel_for_mass_of_100756() {
        assert_eq!(33583, Module::new(100_756).calculate_base_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_14() {
        assert_eq!(2, Module::new(14).calculate_total_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_1969() {
        assert_eq!(966, Module::new(1969).calculate_total_required_fuel());
    }

    #[test]
    fn module_calculates_correct_total_fuel_for_mass_of_100756() {
        assert_eq!(50346, Module::new(100_756).calculate_total_required_fuel());
    }
}
