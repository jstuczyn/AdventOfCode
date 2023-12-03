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

use crate::helpers::{digits, digits_to_number};
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct EngineSchematic {
    inner: HashMap<(usize, usize), SchematicEntry>,
}

impl EngineSchematic {
    pub fn insert<V>(&mut self, pos: (usize, usize), val: V)
    where
        V: Into<SchematicEntry>,
    {
        self.inner.insert(pos, val.into());
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&SchematicEntry> {
        self.inner.get(&pos)
    }

    pub fn num_value(&self, pos: (usize, usize)) -> Option<((usize, usize), u32)> {
        match self.inner.get(&pos)? {
            SchematicEntry::Number(val) => Some((pos, *val)),
            SchematicEntry::Digit { num_start } => self.num_value(*num_start),
            SchematicEntry::Symbol(_) => None,
        }
    }

    pub fn gear_ratio(&self, pos: (usize, usize), val: char) -> Option<u32> {
        if val != '*' {
            return None;
        }

        let (x, y) = pos;
        let mut nums = HashMap::with_capacity(2);

        crate::check_gears!(self, (x, y.saturating_sub(1)), nums);
        crate::check_gears!(self, (x + 1, y.saturating_sub(1)), nums);
        crate::check_gears!(self, (x + 1, y), nums);
        crate::check_gears!(self, (x + 1, y + 1), nums);
        crate::check_gears!(self, (x, y + 1), nums);
        crate::check_gears!(self, (x.saturating_sub(1), y + 1), nums);
        crate::check_gears!(self, (x.saturating_sub(1), y), nums);
        crate::check_gears!(self, (x.saturating_sub(1), y.saturating_sub(1)), nums);

        if nums.len() != 2 {
            None
        } else {
            Some(nums.values().product())
        }
    }

    pub fn part_number(&self, pos: (usize, usize), val: u32) -> Option<u32> {
        let (x, y) = pos;
        let x_end = x + digits(val) - 1;

        for x in x..=x_end {
            crate::check_for_part_number!(self, (x, y.saturating_sub(1)), val)
        }
        crate::check_for_part_number!(self, (x_end + 1, y.saturating_sub(1)), val);
        crate::check_for_part_number!(self, (x_end + 1, y), val);
        crate::check_for_part_number!(self, (x_end + 1, y + 1), val);
        for x in x..=x_end {
            crate::check_for_part_number!(self, (x, y + 1), val);
        }
        crate::check_for_part_number!(self, (x.saturating_sub(1), y + 1), val);
        crate::check_for_part_number!(self, (x.saturating_sub(1), y), val);
        crate::check_for_part_number!(self, (x.saturating_sub(1), y.saturating_sub(1)), val);

        None
    }

    pub fn part_number_sum(&self) -> u32 {
        self.inner
            .iter()
            .filter_map(|(pos, entry)| {
                if let SchematicEntry::Number(val) = entry {
                    self.part_number(*pos, *val)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn gear_ratio_sum(&self) -> u32 {
        self.inner
            .iter()
            .filter_map(|(pos, entry)| {
                if let SchematicEntry::Symbol(val) = entry {
                    self.gear_ratio(*pos, *val)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SchematicEntry {
    Number(u32),
    Digit { num_start: (usize, usize) },
    Symbol(char),
}

impl SchematicEntry {
    pub const fn is_symbol(&self) -> bool {
        matches!(self, SchematicEntry::Symbol(..))
    }
}

impl From<u32> for SchematicEntry {
    fn from(value: u32) -> Self {
        SchematicEntry::Number(value)
    }
}

impl From<char> for SchematicEntry {
    fn from(value: char) -> Self {
        SchematicEntry::Symbol(value)
    }
}

impl FromStr for EngineSchematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = HashMap::new();
        let mut raw_digits = vec![];
        let mut num_start = (0, 0);

        for (y, row) in s.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c.is_ascii_digit() {
                    let pos = (x, y);
                    // this is the first digit
                    if raw_digits.is_empty() {
                        num_start = pos;
                    } else {
                        inner.insert(pos, SchematicEntry::Digit { num_start });
                    }

                    raw_digits.push(c);
                    continue;
                } else if !raw_digits.is_empty() {
                    // convert the digits to a number and insert it to the map
                    let raw = mem::take(&mut raw_digits);
                    let num = digits_to_number(raw);
                    inner.insert(num_start, SchematicEntry::Number(num));
                }

                if c != '.' {
                    inner.insert((x, y), SchematicEntry::Symbol(c));
                }
            }
        }
        Ok(EngineSchematic { inner })
    }
}

#[macro_export]
macro_rules! check_parts {
    ($parts: expr) => {
        if $parts.len() > 2 {
            return None;
        }
    };
}

#[macro_export]
macro_rules! check_for_part_number {
    ($this: expr, $pos: expr, $val: expr) => {
        if let Some(SchematicEntry::Symbol(..)) = $this.get($pos) {
            return Some($val);
        }
    };
}

#[macro_export]
macro_rules! check_gears {
    ($this: expr, $pos: expr, $parts: expr) => {
        if let Some((pos, val)) = $this.num_value($pos) {
            $parts.insert(pos, val);
            if $parts.len() > 2 {
                return None;
            }
        }
    };
}
