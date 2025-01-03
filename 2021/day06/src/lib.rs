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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use aoc_common::parsing::CommaSeparatedParser;
use aoc_solution::Aoc;

#[derive(Aoc)]
#[aoc(input = Vec<usize>)]
#[aoc(parser = CommaSeparatedParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day06;

fn naive_simulation(cycle_timers: &[usize], days: usize) -> usize {
    let mut timers: [usize; 9] = Default::default();
    for timer in cycle_timers {
        timers[*timer] += 1;
    }

    for _ in 0..days {
        let t_0 = timers[0];
        timers[0] = timers[1];
        timers[1] = timers[2];
        timers[2] = timers[3];
        timers[3] = timers[4];
        timers[4] = timers[5];
        timers[5] = timers[6];
        timers[6] = timers[7] + t_0;
        timers[7] = timers[8];
        timers[8] = t_0;
    }

    timers.iter().sum()
}

pub fn part1(input: Vec<usize>) -> usize {
    naive_simulation(&input, 80)
}

pub fn part2(input: Vec<usize>) -> usize {
    naive_simulation(&input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![3, 4, 3, 1, 2];

        let expected = 5934;

        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![3, 4, 3, 1, 2];

        let expected = 26984457539;

        assert_eq!(expected, part2(input))
    }
}
