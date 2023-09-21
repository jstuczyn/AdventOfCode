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

use crate::types::RPSGame;
use common::execution::execute;
use common::parsing::parse_input_lines;
use common::AocSolution;
use std::path::Path;

mod types;

pub struct Day02;

impl AocSolution for Day02 {
    type Input = Vec<RPSGame>;
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

pub fn part1(input: Vec<RPSGame>) -> usize {
    input.into_iter().map(|p| p.play_as_shape()).sum()
}

pub fn part2(input: Vec<RPSGame>) -> usize {
    input.into_iter().map(|p| p.play_as_result()).sum()
}

pub fn solve<P: AsRef<Path>>(input_file: P) {
    execute(input_file, parse_input_lines, part1, part2)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<RPSGame> {
        vec![
            "A Y".parse().unwrap(),
            "B X".parse().unwrap(),
            "C Z".parse().unwrap(),
        ]
    }

    #[test]
    fn part1_sample_input() {
        let expected = 15;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 12;
        assert_eq!(expected, part2(sample_input()))
    }
}
