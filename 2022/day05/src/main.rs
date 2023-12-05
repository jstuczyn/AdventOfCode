// Copyright 2022-2023 Jedrzej Stuczynski
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

use aoc_solution::AocSolutionSolver;
use common::helpers::root_path;
use day05_2022::Day05;

#[cfg(not(tarpaulin))]
fn main() {
    Day05::try_solve_from_file(root_path("inputs/2022/day05"))
}
