// Copyright 2020 Jedrzej Stuczynski
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
// legacy code

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use aoc_common::legacy::input_read;
use day09_2020::{part1, part2};

const PART1_WINDOW_SIZE: usize = 25;

#[cfg(not(tarpaulin_include))]
fn main() {
    let input =
        input_read::read_line_input("inputs/2020/day09").expect("failed to read input file");

    let part1_result = part1(input.clone(), PART1_WINDOW_SIZE).expect("failed to solve part1");
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(input, PART1_WINDOW_SIZE).expect("failed to solve part2");
    println!("Part 2 result is {}", part2_result);
}
