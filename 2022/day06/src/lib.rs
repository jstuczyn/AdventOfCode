// Copyright 2022 Jedrzej Stuczynski
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

use common::execution::execute;
use common::parsing::as_char_vec;
use std::path::Path;

pub use crate::solution::{part1, part2};

mod solution;

pub fn solve<P: AsRef<Path>>(input_file: P) {
    execute(input_file, as_char_vec, part1, part2)
}
