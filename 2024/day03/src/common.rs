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
use winnow::combinator::{alt, delimited, iterator, separated_pair};
use winnow::token::any;
use winnow::{ModalResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Mul(MulInstruction),
    Do,
    Dont,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MulInstruction(usize, usize);

impl MulInstruction {
    pub fn execute(&self) -> usize {
        self.0 * self.1
    }
}

fn mul_parser(input: &mut &str) -> ModalResult<MulInstruction> {
    delimited("mul(", separated_pair(parse_number, ",", parse_number), ")")
        .map(|(lhs, rhs)| MulInstruction(lhs, rhs))
        .parse_next(input)
}

fn instruction_parser(input: &mut &str) -> ModalResult<Instruction> {
    alt((
        mul_parser.map(Instruction::Mul),
        "don't".value(Instruction::Dont),
        "do".value(Instruction::Do),
    ))
    .parse_next(input)
}

pub(crate) struct InstructionsParser;

impl AocInputParser for InstructionsParser {
    type Output = Vec<Instruction>;

    fn parse_input(raw: &str) -> anyhow::Result<Self::Output> {
        let mut it = iterator(raw, alt((instruction_parser.map(Some), any.value(None))));
        let parsed = it.flatten().collect::<Vec<_>>();
        it.finish().map_err(|err| anyhow::format_err!("{err}"))?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_parsing() {
        assert_eq!(mul_parser(&mut "mul(5,5)").unwrap(), MulInstruction(5, 5));

        assert_eq!(
            mul_parser(&mut "mul(42,69)").unwrap(),
            MulInstruction(42, 69)
        );
    }
}
