// Copyright 2022 Jedrzej Stuczynski
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
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct AssignmentPair {
    pub first_range: RangeInclusive<usize>,
    pub second_range: RangeInclusive<usize>,
}

impl FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(raw: &str) -> anyhow::Result<RangeInclusive<usize>> {
            let mut vals = raw.split('-');
            let start = vals
                .next()
                .ok_or_else(|| anyhow!("range does not contain starting value"))?
                .parse()?;
            let end = vals
                .next()
                .ok_or_else(|| anyhow!("range does not contain ending value"))?
                .parse()?;

            Ok(RangeInclusive::new(start, end))
        }

        let mut pairs = s.split(',');
        let raw_first_range = pairs
            .next()
            .ok_or_else(|| anyhow!("input does not contain the first range"))?;
        let raw_second_range = pairs
            .next()
            .ok_or_else(|| anyhow!("input does not contain the second range"))?;

        Ok(AssignmentPair {
            first_range: parse_range(raw_first_range)?,
            second_range: parse_range(raw_second_range)?,
        })
    }
}

impl AssignmentPair {
    pub fn has_full_overlap(&self) -> bool {
        // I find this if chain more readable than the match alternative
        #[allow(clippy::comparison_chain)]
        if self.first_range.start() < self.second_range.start() {
            // second range must be contained within the first range
            self.first_range.end() >= self.second_range.end()
        } else if self.first_range.start() > self.second_range.start() {
            // first range must be contained within the second range
            self.first_range.end() <= self.second_range.end()
        } else {
            // if both ranges start at the same point, they must be fully contained
            true
        }
    }

    pub fn has_any_overlap(&self) -> bool {
        !(self.first_range.start() > self.second_range.end()
            || self.second_range.start() > self.first_range.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_overlap_edge_cases() {
        let edge_case: AssignmentPair = "5-5,5-94".parse().unwrap();
        assert!(edge_case.has_full_overlap());

        let edge_case: AssignmentPair = "34-87,34-88".parse().unwrap();
        assert!(edge_case.has_full_overlap())
    }
}
