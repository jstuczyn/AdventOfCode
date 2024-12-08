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

use crate::common::Instruction;
use crate::common::InstructionsParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<Instruction>)]
#[aoc(parser = InstructionsParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day03;

pub fn part1(input: Vec<Instruction>) -> usize {
    input
        .into_iter()
        .filter_map(|i| {
            if let Instruction::Mul(mul) = i {
                Some(mul.execute())
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: Vec<Instruction>) -> usize {
    let mut active = true;
    let mut sum = 0;
    for instruction in input {
        match instruction {
            Instruction::Mul(mul) => {
                if active {
                    sum += mul.execute()
                }
            }
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
        }
    }
    sum
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::common::Instruction;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Instruction> {
        InstructionsParser::parse_input(
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 161;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 48;
        assert_eq!(expected, part2(sample_input()))
    }
}
