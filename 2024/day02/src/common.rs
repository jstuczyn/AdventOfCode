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

use aoc_common::parsing::combinators::parse_number;
use itertools::Itertools;
use nom::character::complete::multispace1;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Finish;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Report {
    pub(crate) levels: Vec<u8>,
}

fn are_levels_safe(levels: &[u8]) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let expect_increasing = levels[1] > levels[0];

    // using itertools tuple_windows might have been more idiomatic, but meh : )
    for (start_level, end_level) in levels.iter().tuple_windows() {
        // we expect an increase, but it started decreasing
        if expect_increasing && start_level >= end_level {
            return false;
        }

        // we expect a decrease, but it started increasing
        if !expect_increasing && start_level <= end_level {
            return false;
        }

        let diff = start_level.abs_diff(*end_level);
        // the difference is not 1, 2 or 3
        if diff > 3 || diff == 0 {
            return false;
        }
    }

    true
}

impl Report {
    pub fn is_safe(&self) -> bool {
        are_levels_safe(&self.levels)
    }

    pub fn is_dampened_safe(&self) -> bool {
        // if base case is good, no need to check any combinations
        if self.is_safe() {
            return true;
        }

        for idx in 0..self.levels.len() {
            let mut cloned = self.levels.clone();
            cloned.remove(idx);
            if are_levels_safe(&cloned) {
                return true;
            }
        }

        false
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, levels) = separated_list1(multispace1, parse_number)(s)
            .finish()
            .map_err(|err| Error::new(err.input.to_string(), err.code))?;

        Ok(Report { levels })
    }
}
