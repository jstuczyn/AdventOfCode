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
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum WorryLevelChange {
    Add(usize),
    Multiply(usize),
    Square,
}

impl WorryLevelChange {
    #[inline]
    fn apply_change(&self, item: usize) -> usize {
        match self {
            WorryLevelChange::Add(val) => item + val,
            WorryLevelChange::Multiply(val) => item * val,
            WorryLevelChange::Square => item * item,
        }
    }
}

impl FromStr for WorryLevelChange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s
            .trim()
            .strip_prefix("Operation: new = old ")
            .ok_or_else(|| anyhow!("monkey does not contain valid operation"))?;
        if let Some(addition_arg) = stripped.strip_prefix("+ ") {
            Ok(WorryLevelChange::Add(addition_arg.parse()?))
        } else {
            let arg = stripped
                .strip_prefix("* ")
                .ok_or_else(|| anyhow!("monkey does not contain valid operation"))?;
            if arg == "old" {
                Ok(WorryLevelChange::Square)
            } else {
                Ok(WorryLevelChange::Multiply(arg.parse()?))
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum WorryDecrease {
    DivByThree,
    GlobalLCM(usize),
}

impl WorryDecrease {
    #[inline]
    fn apply_strategy(&self, val: usize) -> usize {
        match self {
            WorryDecrease::DivByThree => val / 3,
            WorryDecrease::GlobalLCM(max) => {
                if val > *max {
                    val % max
                } else {
                    val
                }
            }
        }
    }
}

pub struct State {
    monkeys: Vec<Monkey>,
    worry_decrease_strategy: WorryDecrease,
}

impl State {
    pub fn new(monkeys: Vec<Monkey>, worry_decrease_strategy: WorryDecrease) -> Self {
        State {
            monkeys,
            worry_decrease_strategy,
        }
    }

    #[inline]
    pub fn inspection_round(&mut self) {
        for monkey_id in 0..self.monkeys.len() {
            for throw_result in
                self.monkeys[monkey_id].inspect_all_items(self.worry_decrease_strategy)
            {
                self.monkeys[throw_result.target_monkey].catch_item(throw_result.worry_level)
            }
        }
    }

    pub fn inspection_rounds(&mut self, rounds: usize) -> usize {
        for _ in 0..rounds {
            self.inspection_round();
        }

        let mut top1 = 0;
        let mut top2 = 0;
        for monkey in &self.monkeys {
            if monkey.inspected_items > top1 {
                top2 = top1;
                top1 = monkey.inspected_items
            } else if monkey.inspected_items > top2 {
                top2 = monkey.inspected_items
            }
        }

        top1 * top2
    }
}

#[derive(Clone, Debug)]
pub struct Monkey {
    // ordering doesn't matter
    items: Vec<usize>,
    op: WorryLevelChange,
    test_value: usize,
    throw_on_true: usize,
    throw_on_false: usize,

    inspected_items: usize,
}

#[derive(Copy, Clone)]
struct ThrowResult {
    target_monkey: usize,
    worry_level: usize,
}

impl Monkey {
    pub(crate) fn test_value(&self) -> usize {
        self.test_value
    }

    #[inline]
    fn inspect_item(&self, worry_level: usize, worry_strat: WorryDecrease) -> ThrowResult {
        let worry_level = worry_strat.apply_strategy(self.op.apply_change(worry_level));

        let target_monkey = if worry_level % self.test_value == 0 {
            self.throw_on_true
        } else {
            self.throw_on_false
        };

        ThrowResult {
            target_monkey,
            worry_level,
        }
    }

    fn inspect_all_items(&mut self, worry_strat: WorryDecrease) -> Vec<ThrowResult> {
        self.inspected_items += self.items.len();

        let items = std::mem::take(&mut self.items);
        items
            .into_iter()
            .map(|worry_level| self.inspect_item(worry_level, worry_strat))
            .collect()
    }

    fn catch_item(&mut self, worry_level: usize) {
        self.items.push(worry_level)
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _id_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain id"))?;
        let starting_items_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain starting items"))?;
        let operation_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain operation"))?;
        let test_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain test information"))?;
        let true_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain on true information"))?;
        let false_line = lines
            .next()
            .ok_or_else(|| anyhow!("monkey does not contain on false information"))?;

        let starting_items = starting_items_line
            .trim()
            .strip_prefix("Starting items: ")
            .and_then(|raw_items| {
                raw_items
                    .split(", ")
                    .map(|raw| raw.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
            })
            .ok_or_else(|| anyhow!("monkey does not contain valid starting items"))?;
        let operation = operation_line.parse()?;

        let test_value = test_line
            .trim()
            .strip_prefix("Test: divisible by ")
            .ok_or_else(|| anyhow!("test line is malformed"))?
            .parse()?;
        let throw_on_true = true_line
            .trim()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .ok_or_else(|| anyhow!("if true line is malformed"))?
            .parse()?;
        let throw_on_false = false_line
            .trim()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .ok_or_else(|| anyhow!("if false line is malformed"))?
            .parse()?;

        Ok(Monkey {
            items: starting_items,
            op: operation,
            test_value,
            throw_on_true,
            throw_on_false,
            inspected_items: 0,
        })
    }
}
