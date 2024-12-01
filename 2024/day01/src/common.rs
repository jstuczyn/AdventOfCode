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

use common::parsing::parse_input_lines;
use nom::character::complete::{digit1, multispace1};
use nom::combinator::map_res;
use nom::error::Error;
use nom::sequence::separated_pair;
use nom::{Finish, IResult};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct LocationLists {
    pub left: Vec<LocationId>,
    pub right: Vec<LocationId>,
}

impl LocationLists {
    pub fn sorted(&self) -> Self {
        let mut left = self.left.clone();
        let mut right = self.right.clone();

        left.sort();
        right.sort();

        Self { left, right }
    }

    pub fn similarity_score(&self) -> usize {
        let mut frequency_map = HashMap::new();
        for right in &self.right {
            frequency_map
                .entry(right)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut score = 0;
        for left in &self.left {
            let frequency = frequency_map.get(&left).copied().unwrap_or(0);
            score += frequency * left
        }

        score
    }
}

impl FromStr for LocationLists {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Row> = parse_input_lines(s)?;
        Ok(rows.into())
    }
}

pub struct LocationListsIterator {
    location_lists: LocationLists,
}

impl Iterator for LocationListsIterator {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.location_lists.left.pop()?;
        let right = self.location_lists.right.pop()?;
        Some(Row { left, right })
    }
}

impl IntoIterator for LocationLists {
    type Item = Row;
    type IntoIter = LocationListsIterator;

    fn into_iter(self) -> Self::IntoIter {
        LocationListsIterator {
            location_lists: self,
        }
    }
}

impl From<Vec<Row>> for LocationLists {
    fn from(value: Vec<Row>) -> Self {
        let mut left = Vec::with_capacity(value.len());
        let mut right = Vec::with_capacity(value.len());
        for raw in value {
            left.push(raw.left);
            right.push(raw.right);
        }
        LocationLists { left, right }
    }
}

pub type LocationId = usize;

#[derive(Debug, Copy, Clone)]
pub struct Row {
    pub left: LocationId,
    pub right: LocationId,
}

impl Row {
    pub fn difference(&self) -> usize {
        self.left.abs_diff(self.right)
    }
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_number<T: FromStr>(input: &str) -> IResult<&str, T> {
            map_res(digit1, str::parse)(input)
        }

        // TODO: as I start using `nom`, I should probably move common parsers into the common lib
        let (_, (left, right)) = separated_pair(parse_number, multispace1, parse_number)(s)
            .finish()
            .map_err(|err| Error::new(err.input.to_string(), err.code))?;

        Ok(Row { left, right })
    }
}
