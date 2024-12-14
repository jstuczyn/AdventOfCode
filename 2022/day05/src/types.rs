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

use anyhow::{anyhow, bail};
use aoc_common::parsing::{parse_input_lines, split_to_string_groups};
use std::collections::VecDeque;
use std::str::FromStr;

#[inline]
fn stack_index(stack: usize) -> usize {
    stack - 1
}

#[derive(Debug, Clone)]
pub struct Supplies {
    crates: Vec<CrateStack>,
    rearrangement_procedure: VecDeque<RearrangementStep>,
}

impl Supplies {
    pub fn apply_next_step(&mut self, all_at_once: bool) {
        if let Some(next_step) = self.rearrangement_procedure.pop_front() {
            let source = next_step.source_stack;
            let target = next_step.target_stack;
            let amount = next_step.number_of_crates;

            let mut popped = self.crates[stack_index(source)].pop_items(amount);
            if !all_at_once {
                popped.reverse();
            }
            self.crates[stack_index(target)].push_items(popped);
        } else {
            panic!("we run out of steps to apply!")
        }
    }

    pub fn complete_rearrangement_procedure(&mut self, all_at_once: bool) {
        let num_steps = self.rearrangement_procedure.len();
        for _ in 0..num_steps {
            self.apply_next_step(all_at_once);
        }
    }

    pub fn top_message(&self) -> String {
        self.crates
            .iter()
            .filter_map(|crate_stack| crate_stack.peek_top())
            .map(|c| c.0)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct CrateStack {
    items: Vec<Crate>,
}

impl CrateStack {
    fn new(items: Vec<Crate>) -> Self {
        CrateStack { items }
    }

    fn pop_items(&mut self, amount: usize) -> Vec<Crate> {
        let size = self.items.len();
        let new_len = size - amount;
        self.items.split_off(new_len)
    }

    fn push_items(&mut self, mut items: Vec<Crate>) {
        self.items.append(&mut items)
    }

    fn peek_top(&self) -> Option<Crate> {
        self.items.last().copied()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Crate(char);

impl TryFrom<char> for Crate {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_uppercase() {
            bail!("'{value}' is not a valid crate!")
        } else {
            Ok(Crate(value))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RearrangementStep {
    number_of_crates: usize,
    source_stack: usize,
    target_stack: usize,
}

impl FromStr for RearrangementStep {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let without_prefix = s
            .strip_prefix("move ")
            .ok_or_else(|| anyhow!("rearrangement step does not have 'move ' directive!"))?;

        let mut amount_stacks = without_prefix.split(" from ");
        let amount = amount_stacks
            .next()
            .ok_or_else(|| anyhow!("rearrangement step does not have ' from ' directive"))?;

        let mut source_target = amount_stacks
            .next()
            .ok_or_else(|| anyhow!("rearrangement step does not have ' from ' directive"))?
            .split(" to ");

        let source = source_target
            .next()
            .ok_or_else(|| anyhow!("rearrangement step does not have source information"))?;

        let target = source_target
            .next()
            .ok_or_else(|| anyhow!("rearrangement step does not have target information"))?;

        Ok(RearrangementStep {
            number_of_crates: amount.parse()?,
            source_stack: source.parse()?,
            target_stack: target.parse()?,
        })
    }
}

impl FromStr for Supplies {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn extract_row(raw: &str, num_stacks: usize) -> Result<Vec<Option<Crate>>, anyhow::Error> {
            let row_characters = raw.chars().collect::<Vec<_>>();

            // the input has the following format:
            // [<CRATE>]<SPACE>[<CRATE>]...
            // so crate can only appear at positions:
            // 1, 5, 9, ... 4n - 3
            // therefore, if we have n crates, the input MUST be exactly 4n - 1 long
            let expected_len = 4 * num_stacks - 1;
            if row_characters.len() != expected_len {
                bail!(
                    "our row has unexpected len. Expected: {expected_len}, got: {}",
                    row_characters.len()
                );
            }

            (1..=num_stacks)
                .map(|maybe_crate_idx| {
                    let raw_crate = row_characters[4 * maybe_crate_idx - 3];
                    if raw_crate.is_ascii_whitespace() {
                        Ok(None)
                    } else {
                        Some(Crate::try_from(raw_crate)).transpose()
                    }
                })
                .collect()
        }

        let groups = split_to_string_groups(s);
        if groups.len() != 2 {
            bail!("the provided input does not have exactly one stack of crates and one set of procedures")
        }

        let stack_group = groups[0].lines().collect::<Vec<_>>();

        // figure out the number of stacks
        let Some((raw_indices, raw_crates)) = stack_group.split_last() else {
            bail!("the crate stack is empty!")
        };

        let indices = raw_indices.split_ascii_whitespace().collect::<Vec<_>>();
        let Some(raw_last_stack_index) = indices.last() else {
            bail!("the crate stack has no indices!")
        };

        let highest_stack = raw_crates.len();
        let last_stack_index: usize = raw_last_stack_index.parse()?;
        let mut reversed_crates = Vec::with_capacity(last_stack_index);

        // initialise stacks
        for _ in 0..last_stack_index {
            reversed_crates.push(Vec::with_capacity(highest_stack))
        }

        for raw_crate_row in raw_crates {
            let crates = extract_row(raw_crate_row, last_stack_index)?;
            for (index, parsed_crate) in crates.into_iter().enumerate() {
                if let Some(parsed_crate) = parsed_crate {
                    reversed_crates[index].push(parsed_crate)
                }
            }
        }

        Ok(Supplies {
            crates: reversed_crates
                .into_iter()
                .map(|mut reversed_stack| {
                    reversed_stack.reverse();
                    CrateStack::new(reversed_stack)
                })
                .collect(),
            rearrangement_procedure: parse_input_lines(&groups[1])?.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popping_items() {
        let mut dummy = CrateStack {
            items: vec![Crate('A'), Crate('B'), Crate('C'), Crate('D'), Crate('E')],
        };

        let popped = dummy.pop_items(3);
        assert_eq!(vec![Crate('C'), Crate('D'), Crate('E')], popped);
        assert_eq!(vec![Crate('A'), Crate('B')], dummy.items)
    }
}
