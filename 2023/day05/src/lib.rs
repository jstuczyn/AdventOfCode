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

use aoc_solution::Aoc;
use common::parsing::FromStrParser;
use types::Almanac;

mod types;

#[derive(Aoc)]
#[aoc(input = Almanac)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day05;

pub fn part1(input: Almanac) -> usize {
    input.lowest_location()
}

pub fn part2(mut input: Almanac) -> usize {
    input.convert_seeds_to_ranges();
    input.lowest_location()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> Almanac {
        FromStrParser::parse_input(
            r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 35;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 46;
        assert_eq!(expected, part2(sample_input()))
    }
}
