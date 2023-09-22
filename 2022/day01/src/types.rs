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

use common::parsing::parse_input_lines;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Elf {
    pub(crate) calorie_items: Vec<usize>,
}

impl Elf {
    #[cfg(test)]
    pub(crate) fn new(calorie_items: Vec<usize>) -> Self {
        Elf { calorie_items }
    }

    pub(crate) fn total_calorie_count(&self) -> usize {
        self.calorie_items.iter().sum()
    }
}

impl FromStr for Elf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Elf {
            calorie_items: parse_input_lines(s)?,
        })
    }
}
