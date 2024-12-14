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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;
use itertools::Itertools;

#[derive(Aoc)]
#[aoc(input = Vec<usize>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day01;

pub fn part1(input: Vec<usize>) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

pub fn part2(input: Vec<usize>) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 7;
        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let expected = 5;

        assert_eq!(expected, part2(input))
    }
}
