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
pub struct Day09;

fn is_valid(preamble: &[usize], value: usize) -> bool {
    for pair in preamble.iter().tuple_combinations::<(_, _)>() {
        if pair.0 + pair.1 == value {
            return true;
        }
    }

    false
}

pub fn part1(input: Vec<usize>, window_size: usize) -> Option<usize> {
    for (i, val) in input.iter().enumerate().skip(window_size) {
        if !is_valid(&input[i - window_size..i], *val) {
            return Some(*val);
        }
    }

    None
}

fn check_slice(slice: &[usize], target: usize) -> Option<Vec<usize>> {
    let mut running_total = 0;
    let mut used_values = Vec::new();
    for val in slice {
        running_total += *val;
        used_values.push(*val);
        match running_total {
            n if n == target => return Some(used_values),
            n if n > target => return None,
            _ => (),
        }
    }
    None
}

pub fn part2(input: Vec<usize>, window_size: usize) -> Option<usize> {
    let invalid_number = part1(input.clone(), window_size)?;

    // even though it might seem we try every single possible starting index,
    // this loop will never go beyond to where invalid number was found.
    // also we're passing just a pointer to `check_slice` so it's fine to pass it to the whole
    // underlying slice as we will only iterate through tiny part of it each time
    for i in 0..input.len() {
        if let Some(set) = check_slice(&input[i..], invalid_number) {
            debug_assert!(set.len() >= 2);
            let min = set.iter().min()?;
            let max = set.iter().max()?;
            return Some(min + max);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let expected = 127;

        assert_eq!(expected, part1(input, 5).unwrap());
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let expected = 62;

        assert_eq!(expected, part2(input, 5).unwrap());
    }
}
