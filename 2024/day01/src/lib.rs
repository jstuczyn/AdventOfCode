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

use crate::common::LocationLists;
use ::common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = LocationLists)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day01;

pub fn part1(input: LocationLists) -> usize {
    input.sorted().into_iter().map(|r| r.difference()).sum()
}

pub fn part2(input: LocationLists) -> usize {
    input.similarity_score()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> LocationLists {
        FromStrParser::parse_input(
            r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 11;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 31;
        assert_eq!(expected, part2(sample_input()))
    }
}
