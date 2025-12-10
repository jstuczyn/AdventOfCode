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

use crate::common::BatteryBank;
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<BatteryBank>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day03;

pub fn part1(input: Vec<BatteryBank>) -> usize {
    input
        .into_iter()
        .map(|b| b.maximum_joltage_with_two())
        .sum()
}

pub fn part2(input: Vec<BatteryBank>) -> usize {
    input
        .into_iter()
        .map(|b| b.maximum_joltage_with_twelve())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<BatteryBank> {
        LineParser::parse_input(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 357;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 3121910778619;
        assert_eq!(expected, part2(sample_input()))
    }
}
