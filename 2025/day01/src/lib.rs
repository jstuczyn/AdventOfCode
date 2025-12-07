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

use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;
use common::Rotation;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<Rotation>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day01;

pub fn part1(input: Vec<Rotation>) -> usize {
    let mut dial = 50;
    let mut at_zero = 0;
    for rotation in input {
        rotation.apply(&mut dial);
        if dial == 0 {
            at_zero += 1
        }
    }

    at_zero
}

pub fn part2(input: Vec<Rotation>) -> usize {
    let mut dial = 50;
    let mut at_zero = 0;
    for rotation in input {
        at_zero += rotation.apply(&mut dial);
        if dial == 0 {
            at_zero += 1
        }
    }

    at_zero
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Rotation> {
        LineParser::parse_input(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 3;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 6;
        assert_eq!(expected, part2(sample_input()))
    }
}
