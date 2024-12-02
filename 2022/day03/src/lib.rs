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

use crate::types::Rucksack;
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<Rucksack>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day03;

pub fn part1(input: Vec<Rucksack>) -> usize {
    input
        .into_iter()
        .map(|rucksack| rucksack.duplicate_item().priority())
        .sum()
}

pub fn part2(input: Vec<Rucksack>) -> usize {
    input
        .chunks(3)
        .map(|group| group[0].badge(&group[1], &group[2]).priority())
        .sum()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<Rucksack> {
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse().unwrap(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".parse().unwrap(),
            "PmmdzqPrVvPwwTWBwg".parse().unwrap(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".parse().unwrap(),
            "ttgJtRGJQctTZtZT".parse().unwrap(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".parse().unwrap(),
        ]
    }

    #[test]
    fn part1_sample_input() {
        let expected = 157;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 70;
        assert_eq!(expected, part2(sample_input()))
    }
}
