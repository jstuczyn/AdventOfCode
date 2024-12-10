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

use aoc_common::types::Position;
use std::str::FromStr;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, repeat, separated};
use winnow::token::literal;
use winnow::{PResult, Parser};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum XmasLetter {
    X,
    M,
    A,
    S,
}

impl XmasLetter {
    pub fn is_start(&self) -> bool {
        matches!(self, XmasLetter::X)
    }

    pub fn next(&self) -> Option<XmasLetter> {
        match self {
            XmasLetter::X => Some(XmasLetter::M),
            XmasLetter::M => Some(XmasLetter::A),
            XmasLetter::A => Some(XmasLetter::S),
            XmasLetter::S => None,
        }
    }

    pub fn can_form_p2_xmas(&self, other: Self) -> bool {
        use XmasLetter::*;
        matches!((self, other), (M, S) | (S, M))
    }
}

fn xmas_letter_parser(input: &mut &str) -> PResult<XmasLetter> {
    alt((
        literal('X').value(XmasLetter::X),
        literal('M').value(XmasLetter::M),
        literal('A').value(XmasLetter::A),
        literal('S').value(XmasLetter::S),
    ))
    .parse_next(input)
}

fn row_parser(input: &mut &str) -> PResult<Vec<XmasLetter>> {
    repeat(1.., xmas_letter_parser).parse_next(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct WordGrid {
    pub(crate) rows: Vec<Vec<XmasLetter>>,
}

impl FromStr for WordGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = separated(1.., row_parser, line_ending)
            .parse(s.trim())
            .map_err(|err| anyhow::format_err!("{err}"))?;

        Ok(WordGrid { rows })
    }
}

impl WordGrid {
    pub fn letter(&self, position: Position) -> Option<XmasLetter> {
        let pos_x = if position.x >= 0 {
            position.x as usize
        } else {
            return None;
        };

        let pos_y = if position.y >= 0 {
            position.y as usize
        } else {
            return None;
        };

        let row = self.rows.get(pos_y)?;
        row.get(pos_x).copied()
    }

    pub fn on_grid(&self, position: Position) -> bool {
        self.letter(position).is_some()
    }

    #[inline]
    fn check_xmas_direction(
        &self,
        position: Position,
        expected: XmasLetter,
        translation: (isize, isize),
    ) -> bool {
        let translated = position + translation;
        let Some(next_letter) = self.letter(translated) else {
            return false;
        };

        if next_letter != expected {
            return false;
        }

        let Some(following_one) = next_letter.next() else {
            return true;
        };

        self.check_xmas_direction(translated, following_one, translation)
    }

    pub fn find_p1_xmas(&self, start: Position) -> usize {
        debug_assert_eq!(self.letter(start), Some(XmasLetter::X));

        let translations = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        let mut word_count = 0;
        for translation in translations {
            if self.check_xmas_direction(start, XmasLetter::M, translation) {
                word_count += 1
            }
        }

        word_count
    }

    pub fn forms_p2_xmas(&self, start: Position) -> bool {
        debug_assert_eq!(self.letter(start), Some(XmasLetter::A));

        // for a valid X-MAS, there must be two M-S pairs diagonal of each other
        let Some(q1) = self.letter(start + (1, 1)) else {
            return false;
        };
        let Some(q2) = self.letter(start + (-1, 1)) else {
            return false;
        };
        let Some(q3) = self.letter(start + (-1, -1)) else {
            return false;
        };
        let Some(q4) = self.letter(start + (1, -1)) else {
            return false;
        };

        q1.can_form_p2_xmas(q3) && q2.can_form_p2_xmas(q4)
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::XmasLetter::{A, M, S, X};

    #[test]
    fn word_grid_parser() {
        let simple_grid: WordGrid = r#"XMS
ASS
AMX"#
            .parse()
            .unwrap();

        assert_eq!(
            WordGrid {
                rows: vec![vec![X, M, S], vec![A, S, S], vec![A, M, X],]
            },
            simple_grid
        )
    }
}
