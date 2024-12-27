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

use crate::common::TopographicMap;
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = TopographicMap)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day10;

pub fn part1(input: TopographicMap) -> usize {
    input.trailheads_score()
}

pub fn part2(input: TopographicMap) -> usize {
    input.trailheads_rating()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> TopographicMap {
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
            .parse()
            .unwrap()
    }

    fn small_sample_input() -> TopographicMap {
        r#"2220222
4441444
1112111
6543456
7111117
8111118
9111119"#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_small_sample_input() {
        let expected = 2;
        assert_eq!(expected, part1(small_sample_input()))
    }

    fn small_sample_input2() -> TopographicMap {
        r#"4490449
4441498
8882117
6543456
7651987
8761111
9871111"#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_small_sample_input2() {
        let expected = 4;
        assert_eq!(expected, part1(small_sample_input2()))
    }

    #[test]
    fn part1_sample_input() {
        let expected = 36;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 81;
        assert_eq!(expected, part2(sample_input()))
    }
}
