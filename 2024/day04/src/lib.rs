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

use crate::common::{WordGrid, XmasLetter};
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = WordGrid)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day04;

pub fn part1(input: WordGrid) -> usize {
    let mut xmas_count = 0;
    for (y, row) in input.rows.iter().enumerate() {
        for (x, start) in row.iter().enumerate() {
            if start.is_start() {
                xmas_count += input.find_p1_xmas((x, y).into())
            }
        }
    }
    xmas_count
}

pub fn part2(input: WordGrid) -> usize {
    let mut xmas_count = 0;
    // skip first row and columns because it's impossible to have a valid X-MAS there
    // (technically also last, but then code gets unreadable : ) )
    for (y, row) in input.rows.iter().enumerate().skip(1) {
        for (x, start) in row.iter().enumerate().skip(1) {
            if start == &XmasLetter::A && input.forms_p2_xmas((x, y).into()) {
                xmas_count += 1
            }
        }
    }
    xmas_count
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use aoc_solution::parser::AocInputParser;

    fn sample_input() -> WordGrid {
        FromStrParser::parse_input(
            r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
        )
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 18;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 9;
        assert_eq!(expected, part2(sample_input()))
    }
}
