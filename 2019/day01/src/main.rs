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

use day01_2019::{do_part1, do_part2, input_to_modules};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut inputs = vec![];
    for line in reader.lines() {
        inputs.push(line.unwrap());
    }

    inputs
}

fn main() {
    let day1_input = read_input_file("inputs/2019/day01");
    let modules = input_to_modules(day1_input);
    do_part1(&modules);
    do_part2(&modules);
}
