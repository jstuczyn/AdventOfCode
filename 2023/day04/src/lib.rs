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

use crate::types::{score, Scratchcard};
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<Scratchcard>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day04;

pub fn part1(input: Vec<Scratchcard>) -> usize {
    input.into_iter().map(|card| card.score()).sum()
}

pub fn part2(input: Vec<Scratchcard>) -> usize {
    // optimise for performance, not memory
    let mut scores = [0; 201];
    let mut instances = [0; 201];

    let mut total = 0;
    for card in &input {
        let matches = card.matches();
        let score = score(matches);

        // note the -1 offset since cards start at 1
        scores[card.id - 1] = score;
        instances[card.id - 1] += 1;

        let self_instances = instances[card.id - 1];

        if matches != 0 {
            for id in card.id + 1..=(card.id + matches as usize) {
                instances[id - 1] += self_instances;
            }
        }

        total += self_instances;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Scratchcard> {
        LineParser::parse_input(
            r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 13;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 30;
        assert_eq!(expected, part2(sample_input()))
    }
}
