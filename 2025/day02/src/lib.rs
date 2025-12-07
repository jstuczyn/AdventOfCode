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

use crate::common::IdRange;
use aoc_common::parsing::CommaSeparatedParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<IdRange>)]
#[aoc(parser = CommaSeparatedParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day02;

pub fn part1(input: Vec<IdRange>) -> usize {
    input
        .into_iter()
        .flat_map(|range| range.invalid_ids_p1().into_iter())
        .sum()
}

pub fn part2(input: Vec<IdRange>) -> usize {
    input
        .into_iter()
        .flat_map(|range| range.invalid_ids_p2().into_iter())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<IdRange> {
        CommaSeparatedParser::parse_input(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        ).unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 1227775554;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 4174379265;
        assert_eq!(expected, part2(sample_input()))
    }
}
