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

use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;
use types::EngineSchematic;

pub(crate) mod helpers;
mod types;

#[derive(Aoc)]
#[aoc(input = EngineSchematic)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = u32, runner = part1))]
#[aoc(part2(output = u32, runner = part2))]
pub struct Day03;

pub fn part1(input: EngineSchematic) -> u32 {
    input.part_number_sum()
}

pub fn part2(input: EngineSchematic) -> u32 {
    input.gear_ratio_sum()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> EngineSchematic {
        FromStrParser::parse_input(
            r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 4361;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 467835;
        assert_eq!(expected, part2(sample_input()))
    }
}
