// Copyright 2019-2024 Jedrzej Stuczynski
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

use day05_2019::utils::read_input_file;
use day05_2019::{IntcodeMachine, Tape};

fn run_machine(tape: Tape) {
    // answer will be printed (as per specs) to output (here STDOUT)
    // part1 requires input of 1, part2 of 5
    println!("When asked for input, provide '1' when executing part1 and '5' when executing part2");
    IntcodeMachine::new(tape).run().unwrap();
}

fn main() {
    let tape = Tape::new(read_input_file("inputs/2019/day05"));
    run_machine(tape);
}
