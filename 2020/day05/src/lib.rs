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
pub struct Day05;

const HIGH_BIT_ROW: char = 'B';
const LOW_BIT_ROW: char = 'F';
const HIGH_BIT_COLUMN: char = 'R';
const LOW_BIT_COLUMN: char = 'L';

#[derive(Debug)]
pub struct MalformedSeat;

struct Seat {
    row: u8,
    // restricted from 0 to 127, i.e. 7 bit value
    column: u8, // restricted from 0 to 7, i.e. 3 bit value
}

impl Seat {
    fn id(&self) -> usize {
        self.row as usize * 8 + self.column as usize
    }
}

impl<'a> TryFrom<&'a String> for Seat {
    type Error = MalformedSeat;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        if !value.is_ascii() || value.len() != 10 {
            return Err(MalformedSeat);
        }

        let mut row = 0;
        let mut column = 0;

        let (row_raw, column_raw) = value.split_at(7);
        for (i, row_raw_char) in row_raw.chars().rev().enumerate() {
            if row_raw_char == HIGH_BIT_ROW {
                row |= 1 << i
            } else if row_raw_char != LOW_BIT_ROW {
                return Err(MalformedSeat);
            }
        }

        for (i, column_raw_char) in column_raw.chars().rev().enumerate() {
            if column_raw_char == HIGH_BIT_COLUMN {
                column |= 1 << i
            } else if column_raw_char != LOW_BIT_COLUMN {
                return Err(MalformedSeat);
            }
        }

        Ok(Seat { row, column })
    }
}

pub fn part1(input: Vec<String>) -> Option<usize> {
    input
        .iter()
        .map(Seat::try_from)
        .filter(Result::is_ok)
        .map(|seat| seat.unwrap().id())
        .max()
}

pub fn part2(input: Vec<String>) -> Option<usize> {
    let mut seat_ids: Vec<_> = input
        .iter()
        .map(Seat::try_from)
        .filter(Result::is_ok)
        .map(|seat| seat.unwrap().id())
        .collect();

    seat_ids.sort_unstable();
    let mut gaps = Vec::new();
    for (&prev_seat_id, &next_seat_id) in seat_ids.iter().tuple_windows() {
        if prev_seat_id + 1 != next_seat_id {
            gaps.push(prev_seat_id + 1);
        }
    }

    if gaps.len() != 1 && !gaps.is_empty() {
        eprintln!("found multiple possible seat locations! - {:?}", gaps);
        None
    } else {
        gaps.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_parsing() {
        let seat1 = Seat::try_from(&"BFFFBBFRRR".to_string()).unwrap();
        assert_eq!(seat1.row, 70);
        assert_eq!(seat1.column, 7);
        assert_eq!(seat1.id(), 567);

        let seat2 = Seat::try_from(&"FFFBBBFRRR".to_string()).unwrap();
        assert_eq!(seat2.row, 14);
        assert_eq!(seat2.column, 7);
        assert_eq!(seat2.id(), 119);

        let seat3 = Seat::try_from(&"BBFFBBFRLL".to_string()).unwrap();
        assert_eq!(seat3.row, 102);
        assert_eq!(seat3.column, 4);
        assert_eq!(seat3.id(), 820);
    }

    #[test]
    fn sample_part1_input() {
        let input = vec![
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        let expected = 820;

        assert_eq!(expected, part1(input).unwrap());
    }

    #[test]
    fn sample_part2_input() {
        let input = vec![
            "BFFFBBFRRR".to_string(), // 567
            "BFFFBBFRRL".to_string(), // 566
            "BFFFBBFRLL".to_string(), // 564
        ];

        let expected = 565;

        assert_eq!(expected, part2(input).unwrap());
    }
}
