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

use crate::common::Map;
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;
use rayon::prelude::*;
use std::collections::HashSet;

mod common;

#[derive(Aoc)]
#[aoc(input = Map)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day06;

pub fn part1(mut input: Map) -> usize {
    let mut visited = HashSet::new();
    visited.insert(input.guard_position());
    loop {
        let next_guard_position = input.move_guard();

        if input.is_outside_map(next_guard_position) {
            return visited.len();
        }

        visited.insert(next_guard_position);
    }
}

pub fn part2(mut input: Map) -> usize {
    let base_loop_tester = input.clone();

    // 1. get all possible spots visited by the guard
    let mut visited = HashSet::new();
    loop {
        let next_guard_position = input.move_guard();

        if input.is_outside_map(next_guard_position) {
            break;
        }
        visited.insert(next_guard_position);
    }

    // 2. for each of them, try to put an obstacle on the way to see if it would result in a loop
    // single-threaded:
    // let mut loops = 0;
    // for possible_obstacle in visited {
    //     if base_loop_tester
    //         .new_with_obstacle(possible_obstacle)
    //         .test_loop()
    //     {
    //         loops += 1
    //     }
    // }

    visited
        .par_iter()
        .map(|pos| {
            if base_loop_tester.new_with_obstacle(*pos).test_loop() {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Map {
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 41;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 6;
        assert_eq!(expected, part2(sample_input()))
    }
}
