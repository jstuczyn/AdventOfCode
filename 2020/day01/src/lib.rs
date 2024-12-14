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

use aoc_solution::Aoc;
use itertools::Itertools;

#[derive(Aoc)]
pub struct Day01;

pub fn part1(input: Vec<usize>) -> Option<usize> {
    // if you really want to be fancy about it, you could sort the whole thing first,
    // then be smart about choosing second value, like if v1 + v2 > 2020, don't bother
    // checking anything above v2. But current approach works well enough
    // and cutting edge performance is not a requirement.

    for pair in input.iter().tuple_combinations::<(_, _)>() {
        if pair.0 + pair.1 == 2020 {
            return Some(pair.0 * pair.1);
        }
    }

    None
}

pub fn part2(input: Vec<usize>) -> Option<usize> {
    for triplet in input.iter().tuple_combinations::<(_, _, _)>() {
        if triplet.0 + triplet.1 + triplet.2 == 2020 {
            return Some(triplet.0 * triplet.1 * triplet.2);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 514579;

        assert_eq!(expected, part1(input).unwrap())
    }

    #[test]
    fn part1_fails_on_invalid_input() {
        assert!(part1(vec![1, 2, 3]).is_none())
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let expected = 241861950;

        assert_eq!(expected, part2(input).unwrap())
    }

    #[test]
    fn part2_fails_on_invalid_input() {
        assert!(part2(vec![1, 2, 3]).is_none())
    }
}
