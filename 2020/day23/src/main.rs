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
// limitations under the License.

use day23_2020::{part1, part2};

#[cfg(not(tarpaulin_include))]
fn main() {
    let input = 364289715;

    let part1_result = part1(input);
    println!("Part 1 result is {}", part1_result);

    let part2_result = part2(input);
    println!("Part 2 result is {}", part2_result);
}
