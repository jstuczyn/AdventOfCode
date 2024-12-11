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
use std::collections::HashMap;
use std::str::FromStr;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, separated, separated_pair};
use winnow::{PResult, Parser};

#[derive(Clone, Debug)]
pub struct PrintingRules {
    // map of rules where the key value must come before any of the specified values
    ordering_rules: HashMap<usize, Vec<usize>>,
    printing_updates: Vec<PrintingUpdate>,
}

impl PrintingRules {
    fn can_be_printed_before(&self, page: usize, after: &[usize]) -> bool {
        if after.is_empty() {
            return true;
        }

        let Some(rules) = self.ordering_rules.get(&page) else {
            return false;
        };

        for next in after {
            if next == &page {
                continue;
            }

            if !rules.contains(next) {
                return false;
            }
        }

        true
    }

    pub fn is_update_valid(&self, update: &PrintingUpdate) -> bool {
        for (i, page) in update.pages_to_produce.iter().enumerate() {
            if !self.can_be_printed_before(*page, &update.pages_to_produce[i + 1..]) {
                return false;
            }
        }
        true
    }

    pub fn fix_update(&self, update: &PrintingUpdate) -> PrintingUpdate {
        let mut fixed = Vec::with_capacity(update.pages_to_produce.len());

        let mut pages_to_insert = update.pages_to_produce.clone();
        // first page is the one that comes before **all** remaining ones (there can only be one)
        // second page includes all but the first, etc...
        // is this the most efficient? lol, no, insertion sort is one of the worst ones,
        // but given the size of the input, it's a perfectly valid solution to this problem
        while !pages_to_insert.is_empty() {
            if pages_to_insert.len() == 1 {
                fixed.push(pages_to_insert[0]);
                break;
            }
            for (i, page) in pages_to_insert.iter().enumerate() {
                if self.can_be_printed_before(*page, &pages_to_insert) {
                    fixed.push(*page);
                    pages_to_insert.remove(i);
                    break;
                }
            }
        }

        PrintingUpdate {
            pages_to_produce: fixed,
        }
    }

    pub fn updates(&self) -> &[PrintingUpdate] {
        &self.printing_updates
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RawOrderingRule {
    first: usize,
    second: usize,
}

#[derive(Clone, Debug)]
pub struct PrintingUpdate {
    pages_to_produce: Vec<usize>,
}

impl PrintingUpdate {
    pub fn middle_value(&self) -> usize {
        self.pages_to_produce[self.pages_to_produce.len() / 2]
    }
}

fn ordering_rule_parser(input: &mut &str) -> PResult<RawOrderingRule> {
    separated_pair(parse_number, '|', parse_number)
        .map(|(first, second)| RawOrderingRule { first, second })
        .parse_next(input)
}

fn raw_ordering_rules_parser(input: &mut &str) -> PResult<Vec<RawOrderingRule>> {
    separated(1.., ordering_rule_parser, line_ending).parse_next(input)
}

fn printing_update_parser(input: &mut &str) -> PResult<PrintingUpdate> {
    separated(1.., parse_number::<usize>, ',')
        .map(|pages_to_produce| PrintingUpdate { pages_to_produce })
        .parse_next(input)
}

impl FromStr for PrintingRules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_ordering_rules, printing_updates) = separated_pair(
            raw_ordering_rules_parser,
            alt(("\n\n", "\r\n\r\n")),
            separated(1.., printing_update_parser, line_ending),
        )
        .parse(s.trim())
        .map_err(|err| anyhow::format_err!("{err}"))?;

        let mut ordering_rules = HashMap::new();
        for raw in raw_ordering_rules {
            ordering_rules
                .entry(raw.first)
                .or_insert_with(Vec::new)
                .push(raw.second);
        }

        Ok(PrintingRules {
            ordering_rules,
            printing_updates,
        })
    }
}
