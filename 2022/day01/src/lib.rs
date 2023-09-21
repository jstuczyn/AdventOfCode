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

use crate::types::Elf;
use common::parsing::parse_groups;
use common::AocSolution;

mod types;

pub struct Day01;

impl AocSolution for Day01 {
    type Input = Vec<Elf>;
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

pub fn part1(input: Vec<Elf>) -> usize {
    input
        .iter()
        .map(|elf| elf.total_calorie_count())
        .max()
        .unwrap_or_default()
}

pub fn part2(input: Vec<Elf>) -> usize {
    // sorting the input and getting 3 last values is O(nlogn),
    // meanwhile keeping track of 3 maximum values requires single O(n) iteration
    let mut max = usize::MIN;
    let mut max2 = usize::MIN;
    let mut max3 = usize::MIN;

    for total_count in input.iter().map(|elf| elf.total_calorie_count()) {
        if total_count > max {
            max3 = max2;
            max2 = max;
            max = total_count
        } else if total_count > max2 {
            max3 = max2;
            max2 = total_count;
        } else if total_count > max3 {
            max3 = total_count
        }
    }

    max + max2 + max3
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    fn sample_input() -> Vec<Elf> {
        vec![
            Elf::new(vec![1000, 2000, 3000]),
            Elf::new(vec![4000]),
            Elf::new(vec![5000, 6000]),
            Elf::new(vec![7000, 8000, 9000]),
            Elf::new(vec![10000]),
        ]
    }

    #[test]
    fn part1_sample_input() {
        let expected = 24000;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 45000;
        assert_eq!(expected, part2(sample_input()))
    }
}
