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

use crate::common::blink;
use aoc_common::parsing::SpaceSeparatedParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = Vec<usize>)]
#[aoc(parser = SpaceSeparatedParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day11;

pub fn part1(input: Vec<usize>) -> usize {
    blink(input, 25)
}

pub fn part2(input: Vec<usize>) -> usize {
    blink(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<usize> {
        vec![125, 17]
    }

    #[test]
    fn part1_sample_input() {
        let expected = 55312;
        assert_eq!(expected, part1(sample_input()))
    }
}
