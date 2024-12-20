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

use crate::common::Report;
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<Report>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day02;

pub fn part1(input: Vec<Report>) -> usize {
    input.into_iter().filter(|r| r.is_safe()).count()
}

pub fn part2(input: Vec<Report>) -> usize {
    input.into_iter().filter(|r| r.is_dampened_safe()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Report> {
        LineParser::parse_input(
            r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 2;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 4;
        assert_eq!(expected, part2(sample_input()))
    }
}
