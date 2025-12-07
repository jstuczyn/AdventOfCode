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

use aoc_common::helpers::{Digits, digits_to_number};
use aoc_common::parsing::combinators::parse_range_inclusive;
use std::ops::{Deref, RangeInclusive};
use std::str::FromStr;
use winnow::Parser;

#[derive(Clone, Debug)]
pub struct IdRange(RangeInclusive<usize>);

impl IdRange {
    // no point in checking each value individually,
    // we just iterate through all valid prefixes
    pub fn invalid_ids_p1(&self) -> Vec<usize> {
        if self.is_empty() {
            return Vec::new();
        }

        let mut invalid_ids = Vec::new();

        // if our prefix digit count is odd, we start from the smallest next number with even count of digits
        let mut start_digits = self.0.start().to_digits();
        if !start_digits.len().is_multiple_of(2) {
            let start = next_smallest_number_with_more_digits(&start_digits);
            start_digits = start.to_digits();
        }

        let mid_point = start_digits.len() / 2;

        let mut prefix_digits = start_digits[..mid_point].to_vec();
        let mut prefix = digits_to_number(&prefix_digits);
        loop {
            let candidate_id = prefix_digits_to_id(&prefix_digits);
            if self.0.contains(&candidate_id) {
                invalid_ids.push(candidate_id);
            } else if candidate_id > *self.0.start() {
                break;
            }

            prefix += 1;
            prefix_digits = prefix.to_digits();
        }

        invalid_ids
    }

    pub fn invalid_ids_p2(&self) -> Vec<usize> {
        // don't try to be too fancy here,
        // just check every sequence
        // (because I gave up trying to be fancy)
        let mut invalid_ids = Vec::new();

        for candidate in *self.0.start()..=*self.0.end() {
            let digits = candidate.to_digits();
            let digits_len = digits.len();
            if invalid_ids.contains(&candidate) {
                continue;
            }

            for seq_len in 1..=digits_len / 2 {
                // can't possibly be a repeating sequence
                if digits_len % seq_len != 0 {
                    continue;
                }

                let repeats = digits_len / seq_len;
                // task requires at least 2 repeats
                if repeats < 2 {
                    continue;
                }

                let pattern = &digits[..seq_len];
                if pattern.repeat(repeats) == digits {
                    invalid_ids.push(candidate)
                }
            }
        }

        invalid_ids
    }
}

// converts, e.g. [1,2,3] into 123123
fn prefix_digits_to_id(digits: &[usize]) -> usize {
    usize::from_digits(&digits.repeat(2))
}

/// Returns the next smallest number with higher count of digits,
/// e.g. [9,9,9] returns 1000
/// [1,2,3,4] returns 10000
fn next_smallest_number_with_more_digits(digits: &[usize]) -> usize {
    10usize.pow(digits.len() as u32 + 1)
}

impl Deref for IdRange {
    type Target = RangeInclusive<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IdRange(
            parse_range_inclusive
                .parse(s)
                .map_err(|err| anyhow::format_err!("{err}"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_ids_p1() {
        let range = IdRange::from_str("11-44").unwrap();
        assert_eq!(range.invalid_ids_p1(), vec![11, 22, 33, 44]);

        let range = IdRange::from_str("47-126").unwrap();
        assert_eq!(range.invalid_ids_p1(), vec![55, 66, 77, 88, 99]);

        let range = IdRange::from_str("99-1011").unwrap();
        assert_eq!(range.invalid_ids_p1(), vec![99, 1010]);

        let range = IdRange::from_str("999-1011").unwrap();
        assert_eq!(range.invalid_ids_p1(), vec![1010])
    }

    #[test]
    fn invalid_ids_p2() {
        let range = IdRange::from_str("11-22").unwrap();
        assert_eq!(range.invalid_ids_p2(), vec![11, 22]);

        let range = IdRange::from_str("95-115").unwrap();
        assert_eq!(range.invalid_ids_p2(), vec![99, 111]);

        let range = IdRange::from_str("123123122-123123124").unwrap();
        assert_eq!(range.invalid_ids_p2(), vec![123123123]);

        let range = IdRange::from_str("998-1012").unwrap();
        assert_eq!(range.invalid_ids_p2(), vec![999, 1010]);

        let range = IdRange::from_str("824824821-824824827").unwrap();
        assert_eq!(range.invalid_ids_p2(), vec![824824824])
    }
}
