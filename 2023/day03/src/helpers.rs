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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

pub(crate) fn digits(num: u32) -> usize {
    // yes, yes, we could have used a more generic solution with a log10 or iterators, like with
    // `iterate(n, |&n| n / 10).take_while(|&n| n > 0).count().max(1)`
    // but the naive solution is faster : P
    if num < 10 {
        1
    } else if num < 100 {
        2
    } else if num < 1000 {
        3
    } else {
        // manually inspecting the input, this case is impossible
        unreachable!("input contained a 4+ digit number")
    }
}

pub(crate) fn digits_to_number(digits: Vec<char>) -> u32 {
    const RADIX: u32 = 10;
    // SAFETY: 10 is a valid radix value
    digits
        .into_iter()
        .map(|c| c.to_digit(RADIX).unwrap())
        .fold(0, |acc, d| acc * 10 + d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_to_number_test() {
        assert_eq!(4, digits_to_number(vec!['4']));
        assert_eq!(42, digits_to_number(vec!['4', '2']));
        assert_eq!(432, digits_to_number(vec!['4', '3', '2']));
        assert_eq!(4321, digits_to_number(vec!['4', '3', '2', '1']));
    }
}
