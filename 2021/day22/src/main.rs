// Copyright 2021-2022 Jedrzej Stuczynski
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

use aoc_common::legacy::execute_vec;
use aoc_common::legacy::input_read::read_parsed_line_input;
use day22_2021::{part1, part2};

#[cfg(not(tarpaulin_include))]
fn main() {
    execute_vec("inputs/2021/day22", read_parsed_line_input, part1, part2)
}
