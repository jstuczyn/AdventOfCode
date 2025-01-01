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

use aoc_common::helpers::{digits_to_number, split_into_digits};
use std::collections::HashMap;
use std::ops::AddAssign;

// this function doesn't have to be generic over an arbitrary large integer, so let's make it a const instead
const fn has_even_number_of_digits(val: usize) -> bool {
    if val < 10 {
        false
    } else if val < 100 {
        true
    } else if val < 1_000 {
        false
    } else if val < 10_000 {
        true
    } else if val < 100_000 {
        false
    } else if val < 1_000_000 {
        true
    } else if val < 10_000_000 {
        false
    } else if val < 100_000_000 {
        true
    } else if val < 1_000_000_000 {
        false
    } else if val < 10_000_000_000 {
        true
    } else if val < 100_000_000_000 {
        false
    } else if val < 1_000_000_000_000 {
        true
    } else if val < 10_000_000_000_000 {
        false
    } else if val < 100_000_000_000_000 {
        true
    } else if val < 1_000_000_000_000_000 {
        false
    } else if val < 10_000_000_000_000_000 {
        true
    } else if val < 100_000_000_000_000_000 {
        false
    } else if val < 1_000_000_000_000_000_000 {
        true
    } else if val < 10_000_000_000_000_000_000 {
        false
    } else {
        // at this point we reached maximum size of an integer on a 64bit system
        true
    }
}

fn split_stone(stone: usize) -> (usize, usize) {
    let digits = split_into_digits(stone);
    let left = digits_to_number(&digits[..digits.len() / 2]);
    let right = digits_to_number(&digits[digits.len() / 2..]);
    (left, right)
}

pub fn blink(stones: Vec<usize>, count: usize) -> usize {
    // we don't about ordering of the stones, only their count
    // SAFETY: sample input as well as actual aoc input has no repeating values
    let mut stones: HashMap<_, _> = stones.into_iter().map(|s| (s, 1)).collect();

    for _ in 0..count {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in stones.into_iter() {
            // apply all rules in order:

            // If the stone is engraved with the number 0,
            // it is replaced by a stone engraved with the number 1.
            if stone == 0 {
                new_stones.entry(1).or_default().add_assign(count);
                continue;
            }

            // If the stone is engraved with a number that has an even number of digits,
            // it is replaced by two stones.
            // The left half of the digits are engraved on the new left stone,
            // and the right half of the digits are engraved on the new right stone.
            // (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
            if has_even_number_of_digits(stone) {
                let (left, right) = split_stone(stone);
                new_stones.entry(left).or_default().add_assign(count);
                new_stones.entry(right).or_default().add_assign(count);
                continue;
            }

            // If none of the other rules apply,
            // the stone is replaced by a new stone;
            // the old stone's number multiplied by 2024 is engraved on the new stone.
            new_stones
                .entry(stone * 2024)
                .or_default()
                .add_assign(count);
        }
        stones = new_stones;
    }

    stones.into_values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_blink() {
        assert_eq!(3, blink(vec![125, 17], 1));
        assert_eq!(4, blink(vec![125, 17], 2));
        assert_eq!(5, blink(vec![125, 17], 3));
        assert_eq!(9, blink(vec![125, 17], 4));
        assert_eq!(13, blink(vec![125, 17], 5));
        assert_eq!(22, blink(vec![125, 17], 6));
    }

    #[test]
    fn splitting_stone() {
        assert_eq!((10, 0), split_stone(1000));
        assert_eq!((12, 34), split_stone(1234));
        assert_eq!((1, 1), split_stone(11));
    }
}
