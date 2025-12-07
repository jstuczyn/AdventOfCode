// Copyright 2023 Jedrzej Stuczynski
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

use std::path::{Path, PathBuf};

pub const COMMON_ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Creates a path relative to the project root
pub fn root_path<P: AsRef<Path>>(segment: P) -> PathBuf {
    // hehe, that's a disgusting hack, but hey, it works
    PathBuf::from(COMMON_ROOT).join("..").join(segment)
}

pub trait Digits {
    fn to_digits(&self) -> Vec<usize>;

    fn to_digits_reversed(&self) -> Vec<usize>;

    fn from_digits(digits: &[usize]) -> Self;
}

impl Digits for usize {
    fn to_digits(&self) -> Vec<usize> {
        split_into_digits(*self)
    }

    fn to_digits_reversed(&self) -> Vec<usize> {
        split_into_digits_reversed(*self)
    }

    fn from_digits(digits: &[usize]) -> Self {
        digits_to_number(digits)
    }
}

#[inline]
pub fn split_into_digits(number: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut n = number;
    while n > 9 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

#[inline]
pub fn split_into_digits_reversed(mut input: usize) -> Vec<usize> {
    let mut digits = Vec::new();

    if input == 0 {
        digits.push(0);
        return digits;
    }

    while input > 0 {
        digits.push(input % 10);
        input /= 10;
    }

    digits
}

#[inline]
pub fn digits_to_number(digits: &[usize]) -> usize {
    digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, digit)| acc + 10usize.pow(idx as u32) * digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_to_number() {
        assert_eq!(super::digits_to_number(&[1, 2, 3]), 123);
        assert_eq!(super::digits_to_number(&[1]), 1);
        assert_eq!(super::digits_to_number(&[0, 1]), 1);
        assert_eq!(super::digits_to_number(&[0, 1, 2]), 12);
        assert_eq!(super::digits_to_number(&[1, 2, 0]), 120);
    }

    #[test]
    fn number_to_digits() {
        assert_eq!(split_into_digits(1), vec![1]);
        assert_eq!(split_into_digits(12), vec![1, 2]);
        assert_eq!(split_into_digits(123), vec![1, 2, 3]);
        assert_eq!(split_into_digits(1234), vec![1, 2, 3, 4]);
        assert_eq!(split_into_digits(12345), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn number_to_digits_reversed() {
        assert_eq!(split_into_digits_reversed(1), vec![1]);
        assert_eq!(split_into_digits_reversed(12), vec![2, 1]);
        assert_eq!(split_into_digits_reversed(123), vec![3, 2, 1]);
        assert_eq!(split_into_digits_reversed(1234), vec![4, 3, 2, 1]);
        assert_eq!(split_into_digits_reversed(12345), vec![5, 4, 3, 2, 1]);
    }
}
