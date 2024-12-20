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
use std::str::FromStr;
use winnow::ascii::space1;
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

#[derive(Debug, Clone)]
pub struct Equation {
    pub test_value: usize,
    pub operands: Vec<usize>,
}

impl Equation {
    // we don't need to know what particular combination allows us to obtain valid result
    // we just need to know if it's possible
    fn check_subset(&self, index: usize, sub_result: usize) -> bool {
        let operators = [|a, b| a + b, |a, b| a * b];

        if self.operands.len() == index {
            return sub_result == self.test_value;
        }

        for operator in operators {
            let next = self.operands[index];
            if self.check_subset(index + 1, operator(sub_result, next)) {
                return true;
            }
        }

        false
    }

    pub fn is_valid(&self) -> bool {
        self.check_subset(0, 0)
    }
}

fn equation_parser(input: &mut &str) -> PResult<Equation> {
    separated_pair(parse_number, ": ", operands_parser)
        .map(|(test_value, operands)| Equation {
            test_value,
            operands,
        })
        .parse_next(input)
}

fn operands_parser(input: &mut &str) -> PResult<Vec<usize>> {
    separated(1.., parse_number::<usize>, space1).parse_next(input)
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        equation_parser
            .parse(s.trim())
            .map_err(|err| anyhow::format_err!("{err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checking_validity() {
        let equation = Equation {
            test_value: 190,
            operands: vec![10, 19],
        };
        assert!(equation.is_valid());

        let equation = Equation {
            test_value: 292,
            operands: vec![11, 6, 16, 20],
        };
        assert!(equation.is_valid());
    }
}
