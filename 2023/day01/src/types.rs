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

pub fn calculate_calibration_value_sum(raw: Vec<String>, allow_human_digits: bool) -> usize {
    if !allow_human_digits {
        raw.into_iter().map(|s| try_from_ascii_digits(&s)).sum()
    } else {
        raw.into_iter()
            .map(|s| try_from_ascii_and_human_digits(&s))
            .sum()
    }
}

pub fn try_from_ascii_digits(s: &str) -> usize {
    // nice rust solution:
    // let first = s.chars().find(|c| c.is_ascii_digit()).ok_or(anyhow!(
    //     "could not find the first digit of the calibration value"
    // ))?;
    //
    // let last = s.chars().rfind(|c| c.is_ascii_digit()).ok_or(anyhow!(
    //     "could not find the last digit of the calibration value"
    // ))?;
    //
    // // safety: the unwraps here are fine as we've just verified the extracted characters MUST BE ascii digits
    //     // Ok(first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap())
    //
    // but for the first days we care about, as Jeremy Clarkson would say, "SPEEEEEED"
    let bytes = s.as_bytes();

    let mut i = 0;
    let first = loop {
        let c = bytes[i];
        if c.is_ascii_digit() {
            break c - b'0';
        }
        i += 1;
    };

    let mut i = bytes.len() - 1;
    let last = loop {
        let c = bytes[i];
        if c.is_ascii_digit() {
            break c - b'0';
        }
        i -= 1;
    };

    (first * 10 + last) as usize
}

pub fn try_from_ascii_and_human_digits(s: &str) -> usize {
    // SAFETY: we know advent of code inputs are restricted to ascii characters
    let bytes = s.as_bytes();

    let mut i = 0;
    let first = loop {
        let c = bytes[i];
        if c.is_ascii_digit() {
            break c - b'0';
        } else if might_begin_human_readable(c as char) {
            if let Some(digit) = get_start_human_readable_digit(&bytes[i..]) {
                break digit;
            }
        }
        i += 1;
    };

    let mut i = bytes.len() - 1;
    let last = loop {
        let c = bytes[i];
        if c.is_ascii_digit() {
            break c - b'0';
        } else if might_end_human_readable(c as char) {
            if let Some(digit) = get_end_human_readable_digit(&bytes[..=i]) {
                break digit;
            }
        }
        i -= 1;
    };

    (first * 10 + last) as usize
}

const fn might_begin_human_readable(c: char) -> bool {
    // other characters definitely do not indicate a beginning of a human readable digit
    matches!(c, 'z' | 'o' | 't' | 'f' | 's' | 'e' | 'n')
}

const fn might_end_human_readable(c: char) -> bool {
    // other characters definitely do not indicate a end of a human readable digit
    matches!(c, 'o' | 'e' | 'r' | 'n' | 't' | 'x')
}

fn get_start_human_readable_digit(s: &[u8]) -> Option<u8> {
    crate::match_start_human_digit!(s, b"zero", 0);
    crate::match_start_human_digit!(s, b"one", 1);
    crate::match_start_human_digit!(s, b"two", 2);
    crate::match_start_human_digit!(s, b"three", 3);
    crate::match_start_human_digit!(s, b"four", 4);
    crate::match_start_human_digit!(s, b"five", 5);
    crate::match_start_human_digit!(s, b"six", 6);
    crate::match_start_human_digit!(s, b"seven", 7);
    crate::match_start_human_digit!(s, b"eight", 8);
    crate::match_start_human_digit!(s, b"nine", 9);
    None
}

fn get_end_human_readable_digit(s: &[u8]) -> Option<u8> {
    crate::match_end_human_digit!(s, b"zero", 0);
    crate::match_end_human_digit!(s, b"one", 1);
    crate::match_end_human_digit!(s, b"two", 2);
    crate::match_end_human_digit!(s, b"three", 3);
    crate::match_end_human_digit!(s, b"four", 4);
    crate::match_end_human_digit!(s, b"five", 5);
    crate::match_end_human_digit!(s, b"six", 6);
    crate::match_end_human_digit!(s, b"seven", 7);
    crate::match_end_human_digit!(s, b"eight", 8);
    crate::match_end_human_digit!(s, b"nine", 9);
    None
}

#[macro_export]
macro_rules! match_start_human_digit {
    ($s: expr, $expected: literal, $value: expr) => {
        if $s.starts_with($expected) {
            return Some($value);
        }
    };
}

#[macro_export]
macro_rules! match_end_human_digit {
    ($s: expr, $expected: literal, $value: expr) => {
        if $s.ends_with($expected) {
            return Some($value);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_human_ascii() {
        assert_eq!(83, try_from_ascii_and_human_digits("eighthree"));
        assert_eq!(79, try_from_ascii_and_human_digits("sevenine"));
        assert_eq!(94, try_from_ascii_and_human_digits("9sixonefour"));
    }
}
