// Copyright 2023 Jedrzej Stuczynski
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

use crate::types::calculate_calibration_value_sum;
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<String>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day01;

pub fn part1(input: Vec<String>) -> usize {
    calculate_calibration_value_sum(input, false)
}

pub fn part2(input: Vec<String>) -> usize {
    calculate_calibration_value_sum(input, true)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<String> {
        LineParser::parse_input(
            r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#,
        )
        .unwrap()
    }

    fn sample_input2() -> Vec<String> {
        LineParser::parse_input(
            r#"    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 142;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 281;
        assert_eq!(expected, part2(sample_input2()))
    }
}
