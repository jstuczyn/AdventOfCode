// Copyright 2025 Jedrzej Stuczynski
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

use aoc_common::types::{Grid, ParsableGridItem, PositionedItem};
use std::fmt::Display;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::token::literal;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PaperGridItem {
    Paper,
    // Forklift,
    Empty,
}

impl PaperGridItem {
    pub fn is_paper(&self) -> bool {
        *self == PaperGridItem::Paper
    }

    pub fn set_to_empty(&mut self) {
        *self = PaperGridItem::Empty
    }
}

impl Display for PaperGridItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperGridItem::Paper => '@'.fmt(f),
            // PaperGridItem::Forklift => 'x'.fmt(f),
            PaperGridItem::Empty => '.'.fmt(f),
        }
    }
}

impl ParsableGridItem for PaperGridItem {
    const PARSER: fn(&mut &str) -> ModalResult<Self> = |input| {
        alt((
            literal('.').value(PaperGridItem::Empty),
            literal('@').value(PaperGridItem::Paper),
        ))
        .parse_next(input)
    };
}

pub type PaperGrid = Grid<PaperGridItem>;

pub trait PaperGridExt {
    fn accessible_count(&self) -> usize;

    fn remove_accessible(&mut self) -> usize;

    fn can_be_accessed<T>(&self, item: &PositionedItem<T>) -> bool
    where
        T: std::ops::Deref<Target = PaperGridItem>;
}

impl PaperGridExt for PaperGrid {
    fn accessible_count(&self) -> usize {
        self.iter()
            .filter(|item| self.can_be_accessed(item))
            .count()
    }

    fn remove_accessible(&mut self) -> usize {
        let mut to_remove = Vec::new();
        for item in self.iter() {
            if self.can_be_accessed(&item) {
                to_remove.push(item.position);
            }
        }
        let count = to_remove.len();
        for item in to_remove {
            // SAFETY: the item MUST exist as we just retrieved it through the iterator
            #[allow(clippy::unwrap_used)]
            self.get_mut(item).unwrap().set_to_empty()
        }

        count
    }

    fn can_be_accessed<T>(&self, item: &PositionedItem<T>) -> bool
    where
        T: std::ops::Deref<Target = PaperGridItem>,
    {
        item.is_paper()
            && self
                .adjacent(item)
                .iter()
                .filter(|item| item.is_paper())
                .count()
                < 4
    }
}
