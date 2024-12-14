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

use day04_2019::{do_part1, do_part2, PasswordRange};

const DAY4_INPUT: &str = "402328-864247";

fn main() {
    let input = DAY4_INPUT;
    let pwrange = PasswordRange::new(input);
    do_part1(&pwrange);
    do_part2(&pwrange)
}
