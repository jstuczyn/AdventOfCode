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

use crate::intcode_machine::{IntcodeMachine, Tape};
use aoc_solution::Aoc;

pub mod intcode_machine;
pub mod utils;

#[derive(Aoc)]
pub struct Day09;

pub fn do_part1(tape: Tape) {
    let fake_input = b"1";
    let mut output = Vec::new();

    IntcodeMachine::new(tape, &fake_input[..], &mut output)
        .run()
        .unwrap();

    let parsed_output = utils::parse_multiple_utf8_num_repr_lns(&output)
        .last()
        .unwrap()
        .to_owned();

    println!("{:?}", parsed_output);
}

pub fn do_part2(tape: Tape) {
    let fake_input = b"2";
    let mut output = Vec::new();

    IntcodeMachine::new(tape, &fake_input[..], &mut output)
        .run()
        .unwrap();

    let parsed_output = utils::parse_multiple_utf8_num_repr_lns(&output)
        .last()
        .unwrap()
        .to_owned();

    println!("{:?}", parsed_output);
}
