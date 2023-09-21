// Copyright 2022-2023 Jedrzej Stuczynski
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

#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use crate::types::AssignmentPair;
use common::parsing::parse_input_lines;
use common::AocSolution;

mod types;

pub struct Day04;

impl AocSolution for Day04 {
    type Input = Vec<AssignmentPair>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input<M: AsRef<str>>(raw: M) -> Result<Self::Input, anyhow::Error> {
        parse_input_lines(raw.as_ref())
    }
    fn part1(input: Self::Input) -> Result<Self::Part1Output, anyhow::Error> {
        Ok(part1(input))
    }

    fn part2(input: Self::Input) -> Result<Self::Part2Output, anyhow::Error> {
        Ok(part2(input))
    }
}

pub fn part1(input: Vec<AssignmentPair>) -> usize {
    input
        .into_iter()
        .filter(|pair| pair.has_full_overlap())
        .count()
}

pub fn part2(input: Vec<AssignmentPair>) -> usize {
    input
        .into_iter()
        .filter(|pair| pair.has_any_overlap())
        .count()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<AssignmentPair> {
        vec![
            "2-4,6-8".parse().unwrap(),
            "2-3,4-5".parse().unwrap(),
            "5-7,7-9".parse().unwrap(),
            "2-8,3-7".parse().unwrap(),
            "6-6,4-6".parse().unwrap(),
            "2-6,4-8".parse().unwrap(),
        ]
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
