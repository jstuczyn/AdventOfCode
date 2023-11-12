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

pub use crate::types::AssignmentPair;
use aoc_solution::Aoc;
use common::parsing::LineParser;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<AssignmentPair>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day04;

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
