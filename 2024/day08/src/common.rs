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

use aoc_common::types::{Grid, ParsableGridItem, Position};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use winnow::combinator::alt;
use winnow::stream::AsChar;
use winnow::token::{literal, take_while};
use winnow::{ModalResult, Parser};

#[derive(Debug, Copy, Clone)]
pub enum AntennaGridItem {
    Empty,
    Frequency(char),
}

impl Display for AntennaGridItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AntennaGridItem::Empty => write!(f, "."),
            AntennaGridItem::Frequency(frequency) => write!(f, "{frequency}"),
        }
    }
}

impl ParsableGridItem for AntennaGridItem {
    const PARSER: fn(&mut &str) -> ModalResult<Self> = |input| {
        alt((
            literal('.').value(AntennaGridItem::Empty),
            take_while(1..=1, AsChar::is_alphanum)
                .map(|f: &str| AntennaGridItem::Frequency(char::from(f.as_bytes()[0]))),
        ))
        .parse_next(input)
    };
}

pub type RawCityGrid = Grid<AntennaGridItem>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Ord, PartialOrd, Eq)]
pub struct Antenna {
    frequency: char,
}

#[derive(Debug, Clone)]
pub struct AntennaGrid {
    antennas: HashMap<Antenna, Vec<Position>>,
    width: usize,
    height: usize,
}

impl From<RawCityGrid> for AntennaGrid {
    fn from(value: RawCityGrid) -> Self {
        let height = value.height();
        let width = value.width();
        let mut antennas: HashMap<Antenna, Vec<Position>> = HashMap::new();
        for item in value.into_iter() {
            let AntennaGridItem::Frequency(frequency) = item.item else {
                continue;
            };
            antennas
                .entry(Antenna { frequency })
                .or_default()
                .push(item.position)
        }

        AntennaGrid {
            antennas,
            height,
            width,
        }
    }
}

impl FromStr for AntennaGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RawCityGrid::from_str(s).map(Into::into)
    }
}

#[inline]
const fn determine_antinode_locations(first: Position, second: Position) -> (Position, Position) {
    let sep = antinode_vec(first, second);
    (
        Position {
            x: first.x - sep.0,
            y: first.y - sep.1,
        },
        Position {
            x: second.x + sep.0,
            y: second.y + sep.1,
        },
    )
}

#[inline]
const fn antinode_vec(start: Position, end: Position) -> (isize, isize) {
    (end.x - start.x, end.y - start.y)
}

impl AntennaGrid {
    pub fn on_grid(&self, position: Position) -> bool {
        position.x < self.width as isize
            && position.y < self.height as isize
            && position.is_in_quadrant1()
    }

    pub fn count_basic_antinodes(&self) -> usize {
        let mut antinode_locations = HashSet::new();

        for antennas in self.antennas.values() {
            for antenna_pair in antennas.iter().combinations(2) {
                let (an1, an2) = determine_antinode_locations(*antenna_pair[0], *antenna_pair[1]);
                if self.on_grid(an1) {
                    antinode_locations.insert(an1);
                }
                if self.on_grid(an2) {
                    antinode_locations.insert(an2);
                }
            }
        }

        antinode_locations.len()
    }

    pub fn count_antinodes_with_harmonics(&self) -> usize {
        let mut antinode_locations = HashSet::new();

        for antennas in self.antennas.values() {
            for antenna_pair in antennas.iter().combinations(2) {
                let antinode_vec = antinode_vec(*antenna_pair[0], *antenna_pair[1]);
                let mut neg_antinode = *antenna_pair[0];
                let mut pos_antinode = *antenna_pair[1];

                while self.on_grid(neg_antinode) {
                    antinode_locations.insert(neg_antinode);
                    neg_antinode -= antinode_vec;
                }

                while self.on_grid(pos_antinode) {
                    antinode_locations.insert(pos_antinode);
                    pos_antinode += antinode_vec;
                }
            }
        }

        antinode_locations.len()
    }
}
