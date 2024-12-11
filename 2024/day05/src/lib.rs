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

use crate::common::PrintingRules;
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = PrintingRules)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day05;

pub fn part1(input: PrintingRules) -> usize {
    input
        .updates()
        .iter()
        .filter(|u| input.is_update_valid(u))
        .map(|u| u.middle_value())
        .sum()
}

pub fn part2(input: PrintingRules) -> usize {
    input
        .updates()
        .iter()
        .filter(|u| !input.is_update_valid(u))
        .map(|u| input.fix_update(u).middle_value())
        .sum()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> PrintingRules {
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 143;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 123;
        assert_eq!(expected, part2(sample_input()))
    }
}
