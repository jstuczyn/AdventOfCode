// Copyright 2025 Jedrzej Stuczynski
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

use crate::common::{PaperGrid, PaperGridExt};
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;

mod common;

#[derive(Aoc)]
#[aoc(input = PaperGrid)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day04;

pub fn part1(grid: PaperGrid) -> usize {
    grid.accessible_count()
}

pub fn part2(mut grid: PaperGrid) -> usize {
    let mut removed = 0;

    loop {
        let iteration_removed = grid.remove_accessible();
        removed += iteration_removed;
        if iteration_removed == 0 {
            break;
        }
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> PaperGrid {
        r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 13;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 43;
        assert_eq!(expected, part2(sample_input()))
    }
}
