// Copyright 2022-2023 Jedrzej Stuczynski
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

use anyhow::bail;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Item(char);

impl TryFrom<char> for Item {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_alphabetic() {
            bail!("{value} is not a valid rucksack item")
        } else {
            Ok(Item(value))
        }
    }
}

impl Item {
    #[inline]
    fn ascii_value(&self) -> usize {
        debug_assert!(self.0.is_ascii_alphabetic());

        // the casting here is fine as we ensured during parsing that the internal character is a valid ASCII value
        self.0 as usize
    }

    #[inline]
    pub(crate) fn priority(&self) -> usize {
        let ascii = self.ascii_value();
        if ascii < 91 {
            // uppercase
            ascii - 38
        } else {
            // lowercase
            ascii - 96
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rucksack {
    first_compartment: HashSet<Item>,
    second_compartment: HashSet<Item>,
    // is it duplicate data?
    // yes.
    // does it hurt anything?
    // no.
    // and does it make part2 simpler?
    // yes.
    full_content: HashSet<Item>,
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .chars()
            .into_iter()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        if items.len() % 2 != 0 {
            bail!("received odd number of rucksack items")
        }

        let first_compartment = items.iter().copied().take(items.len() / 2).collect();
        let second_compartment = items.iter().copied().skip(items.len() / 2).collect();
        let full_content = items.into_iter().collect();

        Ok(Rucksack {
            first_compartment,
            second_compartment,
            full_content,
        })
    }
}

impl Rucksack {
    pub(crate) fn duplicate_item(&self) -> Item {
        for first_item in &self.first_compartment {
            if self.second_compartment.contains(first_item) {
                return *first_item;
            }
        }

        unreachable!("the puzzle invariant has been broken - no solution exists")
    }

    pub(crate) fn badge(&self, second_elf: &Self, third_elf: &Self) -> Item {
        for candidate in self.full_content.iter() {
            if second_elf.full_content.contains(candidate)
                && third_elf.full_content.contains(candidate)
            {
                return *candidate;
            }
        }

        unreachable!("the puzzle invariant has been broken - no solution exists")
    }
}
