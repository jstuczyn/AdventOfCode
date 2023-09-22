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

use crate::types::Supplies;
use common::AocSolution;

mod types;

pub struct Day05;

impl AocSolution for Day05 {
    type Input = Supplies;
    type Part1Output = String;
    type Part2Output = String;

    fn parse_input<M: AsRef<str>>(raw: M) -> Result<Self::Input, anyhow::Error> {
        raw.as_ref().parse()
    }

    fn part1(input: Self::Input) -> Result<Self::Part1Output, anyhow::Error> {
        Ok(part1(input))
    }

    fn part2(input: Self::Input) -> Result<Self::Part2Output, anyhow::Error> {
        Ok(part2(input))
    }
}

pub fn part1(mut input: Supplies) -> String {
    input.complete_rearrangement_procedure(false);
    input.top_message()
}

pub fn part2(mut input: Supplies) -> String {
    input.complete_rearrangement_procedure(true);
    input.top_message()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> Supplies {
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
        .parse()
        .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = "CMZ";
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = "MCD";
        assert_eq!(expected, part2(sample_input()))
    }
}
