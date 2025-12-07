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

use std::ops::RangeInclusive;
use std::str::FromStr;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::digit1;
use winnow::combinator::separated_pair;
use winnow::stream::AsChar;
use winnow::token::take_while;

pub fn parse_number<T: FromStr>(input: &mut &str) -> ModalResult<T> {
    digit1.parse_to().parse_next(input)
}

pub fn parse_digit<T: FromStr>(input: &mut &str) -> ModalResult<T> {
    take_while(1, AsChar::is_dec_digit)
        .parse_to()
        .parse_next(input)
}

pub fn parse_range_inclusive<T: FromStr>(input: &mut &str) -> ModalResult<RangeInclusive<T>> {
    separated_pair(parse_number, "-", parse_number)
        .map(|(start, end)| RangeInclusive::new(start, end))
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_inclusive_parser() {
        assert_eq!(
            RangeInclusive::new(11usize, 22),
            parse_range_inclusive::<usize>.parse("11-22").unwrap()
        );
        assert_eq!(
            RangeInclusive::new(1188511880usize, 1188511890),
            parse_range_inclusive::<usize>
                .parse("1188511880-1188511890")
                .unwrap()
        );
        assert_eq!(
            RangeInclusive::new(998usize, 1012),
            parse_range_inclusive::<usize>.parse("998-1012").unwrap()
        );
    }
}
