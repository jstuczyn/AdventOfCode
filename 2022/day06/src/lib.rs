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

use aoc_solution::Aoc;
use common::parsing::CharVecParser;

#[derive(Aoc)]
#[aoc(input = Vec<char>)]
#[aoc(parser = CharVecParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day06;

// since our window size is 4 and 14 for part1 and part2 respectively,
// it's more efficient to do full slice lookup as opposed to paying for the instantiation cost of a HashSet
fn solve(input: Vec<char>, window_size: usize) -> usize {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, slice)| !(1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1])))
        .unwrap_or_else(|| panic!("no solution exists for windows size {window_size}"))
        .0
        + window_size
}

pub fn part1(input: Vec<char>) -> usize {
    solve(input, 4)
}

pub fn part2(input: Vec<char>) -> usize {
    solve(input, 14)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect()));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg".chars().collect()));
        assert_eq!(
            10,
            part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect())
        );
        assert_eq!(
            11,
            part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect())
        );
    }

    #[test]
    fn part2_sample_input() {
        assert_eq!(
            19,
            part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect())
        );
        assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect()));
        assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg".chars().collect()));
        assert_eq!(
            29,
            part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect())
        );
        assert_eq!(
            26,
            part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect())
        );
    }
}
