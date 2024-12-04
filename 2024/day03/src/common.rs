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
use aoc_solution::parser::AocInputParser;
use std::str::FromStr;
use winnow::combinator::{alt, delimited, iterator, separated_pair};
use winnow::token::any;
use winnow::{PResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MulInstruction(usize, usize);

impl MulInstruction {
    pub fn execute(&self) -> usize {
        self.0 * self.1
    }
}

fn mul_parser(input: &mut &str) -> PResult<MulInstruction> {
    delimited("mul(", separated_pair(parse_number, ",", parse_number), ")")
        .map(|(lhs, rhs)| MulInstruction(lhs, rhs))
        .parse_next(input)
}

impl FromStr for MulInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mul = mul_parser
            .parse(s)
            .map_err(|err| anyhow::format_err!("{err}"))?;
        Ok(mul)
    }
}

pub(crate) struct MulInstructionParser;

impl AocInputParser for MulInstructionParser {
    type Output = Vec<MulInstruction>;

    fn parse_input(raw: &str) -> anyhow::Result<Self::Output> {
        let mut it = iterator(raw, alt((mul_parser.map(Some), any.value(None))));
        let parsed = it.flatten().collect::<Vec<_>>();
        it.finish().map_err(|err| anyhow::format_err!("{err}"))?;
        Ok(parsed)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn mul_parsing() {
        assert_eq!(
            "mul(5,5)".parse::<MulInstruction>().unwrap(),
            MulInstruction(5, 5)
        );

        assert_eq!(
            "mul(42,69)".parse::<MulInstruction>().unwrap(),
            MulInstruction(42, 69)
        );
    }
}
