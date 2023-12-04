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

use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Scratchcard {
    pub id: usize,

    // the length of the vector is small enough for it to be faster than the overhead of using hashset
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

#[inline]
pub fn score(matches: u32) -> usize {
    if matches == 0 {
        0
    } else {
        2usize.pow(matches - 1)
    }
}

impl Scratchcard {
    #[inline]
    pub fn matches(&self) -> u32 {
        let mut matches = 0;

        for num in &self.card_numbers {
            if self.winning_numbers.contains(num) {
                matches += 1;
            }
        }

        matches
    }

    #[inline]
    pub fn score(&self) -> usize {
        score(self.matches())
    }
}

impl FromStr for Scratchcard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, numbers) = s
            .split_once(": ")
            .ok_or(anyhow!("the scratchcard didn't have colon separator"))?;
        let id = id
            .strip_prefix("Card ")
            .ok_or(anyhow!("the scratchcard didn't have 'Card' prefix"))?;

        let (winning_numbers, card_numbers) = numbers
            .split_once(" | ")
            .ok_or(anyhow!("the scratchcard didn't have | separator"))?;

        let winning_numbers = winning_numbers
            .split_ascii_whitespace()
            .map(|num| num.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;

        let card_numbers = card_numbers
            .split_ascii_whitespace()
            .map(|num| num.trim().parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Scratchcard {
            id: id.trim().parse()?,
            winning_numbers,
            card_numbers,
        })
    }
}
