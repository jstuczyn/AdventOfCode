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

use crate::types::{Monkey, State, WorryDecrease};
use common::execution::execute;
use common::parsing::parse_groups;
use common::AocSolution;
use num::integer::lcm;
use std::path::Path;

mod types;

pub struct Day11;

impl AocSolution for Day11 {
    type Input = Vec<Monkey>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input<M: AsRef<str>>(raw: M) -> Result<Self::Input, anyhow::Error> {
        parse_groups(raw.as_ref())
    }

    fn part1(input: Self::Input) -> Result<Self::Part1Output, anyhow::Error> {
        Ok(part1(input))
    }

    fn part2(input: Self::Input) -> Result<Self::Part2Output, anyhow::Error> {
        Ok(part2(input))
    }
}

pub fn solve<P: AsRef<Path>>(input_file: P) {
    execute(input_file, parse_groups, part1, part2)
}

pub fn part1(input: Vec<Monkey>) -> usize {
    let mut state = State::new(input, WorryDecrease::DivByThree);
    state.inspection_rounds(20)
}

pub fn part2(input: Vec<Monkey>) -> usize {
    let lcm = input.iter().map(|m| m.test_value()).fold(1, lcm);
    let mut state = State::new(input, WorryDecrease::GlobalLCM(lcm));
    state.inspection_rounds(10000)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::types::Monkey;
    use common::parsing::parse_groups;

    fn sample_input() -> Vec<Monkey> {
        let raw = r#"Monkey 0:
                Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

            Monkey 1:
                Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

            Monkey 2:
                Starting items: 79, 60, 97
            Operation: new = old * old
            Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

            Monkey 3:
                Starting items: 74
            Operation: new = old + 3
            Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
"#;
        parse_groups(raw).unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 10605;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 2713310158;
        assert_eq!(expected, part2(sample_input()))
    }
}
