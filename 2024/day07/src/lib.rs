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

use crate::common::Equation;
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<Equation>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day07;

pub fn part1(input: Vec<Equation>) -> usize {
    input
        .into_iter()
        .filter(|e| e.is_valid())
        .map(|e| e.test_value)
        .sum()
}

pub fn part2(input: Vec<Equation>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Equation> {
        LineParser::parse_input(
            r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 3749;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 0;
        assert_eq!(expected, part2(sample_input()))
    }
}
