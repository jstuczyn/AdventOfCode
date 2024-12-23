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

use crate::common::DiskMap;
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = DiskMap)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day09;

pub fn part1(mut input: DiskMap) -> usize {
    input.defragment();
    input.checksum()
}

pub fn part2(input: DiskMap) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::DiskMap;

    fn sample_input() -> DiskMap {
        "2333133121414131402".parse().unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 1928;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 0;
        assert_eq!(expected, part2(sample_input()))
    }
}
