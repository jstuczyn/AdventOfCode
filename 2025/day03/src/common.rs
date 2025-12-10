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

use anyhow::Context;
use aoc_common::helpers::digits_to_number;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct BatteryBank {
    pub batteries: Vec<usize>,
}

impl BatteryBank {
    fn maximum_joltage(&self, num_digits: usize) -> usize {
        let mut digits = Vec::new();
        let mut remaining_to_choose = num_digits;

        let len = self.batteries.len();
        if len < num_digits {
            return 0;
        }
        if len == num_digits {
            return digits_to_number(&self.batteries);
        }

        let mut window_start = 0;
        for _ in 0..num_digits {
            let mut chosen = 0;

            // our unused window must always be sufficiently big for the remaining digits
            let selection_window = &self.batteries[window_start..len - remaining_to_choose + 1];

            // check if we simply run out of choices
            if self.batteries[window_start..].len() == remaining_to_choose {
                digits.extend_from_slice(&self.batteries[window_start..]);
                break;
            }

            let mut found_at_index = 0;
            for (i, value) in selection_window.iter().enumerate() {
                if *value > chosen {
                    chosen = *value;
                    found_at_index = i;
                }
                if *value == 9 {
                    break;
                }
            }
            window_start += found_at_index + 1;
            remaining_to_choose -= 1;

            digits.push(chosen);
        }

        digits_to_number(&digits)
    }

    pub fn maximum_joltage_with_two(&self) -> usize {
        self.maximum_joltage(2)
    }

    pub fn maximum_joltage_with_twelve(&self) -> usize {
        self.maximum_joltage(12)
    }
}

impl FromStr for BatteryBank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as usize)
                    .context("invalid battery value")
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(BatteryBank { batteries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_joltage_with_two() {
        let bank = BatteryBank::from_str("987654321111111").unwrap();
        assert_eq!(bank.maximum_joltage_with_two(), 98);

        let bank = BatteryBank::from_str("811111111111119").unwrap();
        assert_eq!(bank.maximum_joltage_with_two(), 89);

        let bank = BatteryBank::from_str("234234234234278").unwrap();
        assert_eq!(bank.maximum_joltage_with_two(), 78);

        let bank = BatteryBank::from_str("818181911112111").unwrap();
        assert_eq!(bank.maximum_joltage_with_two(), 92);
    }

    #[test]
    fn maximum_joltage_with_twelve() {
        let bank = BatteryBank::from_str("987654321111111").unwrap();
        assert_eq!(bank.maximum_joltage_with_twelve(), 987654321111);

        let bank = BatteryBank::from_str("811111111111119").unwrap();
        assert_eq!(bank.maximum_joltage_with_twelve(), 811111111119);

        let bank = BatteryBank::from_str("234234234234278").unwrap();
        assert_eq!(bank.maximum_joltage_with_twelve(), 434234234278);

        let bank = BatteryBank::from_str("818181911112111").unwrap();
        assert_eq!(bank.maximum_joltage_with_twelve(), 888911112111);
    }
}
