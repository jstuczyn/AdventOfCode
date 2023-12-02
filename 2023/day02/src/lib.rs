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

use crate::types::{CubeSet, Game};
use aoc_solution::Aoc;
use common::parsing::LineParser;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<Game>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day02;

pub fn part1(input: Vec<Game>) -> usize {
    const BAG_CONTENT: CubeSet = CubeSet::new(12, 13, 14);

    input
        .into_iter()
        .filter(|g| g.is_possible(BAG_CONTENT))
        .map(|g| g.id)
        .sum()
}

pub fn part2(input: Vec<Game>) -> usize {
    input.into_iter().map(|g| g.minimal_set().power()).sum()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Vec<Game> {
        LineParser::parse_input(
            r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 8;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 2286;
        assert_eq!(expected, part2(sample_input()))
    }
}
