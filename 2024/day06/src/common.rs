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

use anyhow::bail;
use aoc_common::types::Position;
use std::collections::HashSet;
use std::str::FromStr;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, repeat, separated};
use winnow::token::literal;
use winnow::{PResult, Parser};

#[derive(Debug, Copy, Clone)]
enum MapFeature {
    Empty,
    Obstruction,
    UpFacingGuard,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GuardDirection {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Guard {
    position: Position,
    direction: GuardDirection,
}

impl Guard {
    fn rotate(&mut self) {
        self.direction = match self.direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Right => GuardDirection::Down,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
        }
    }

    fn move_to(&mut self, new_position: Position) {
        self.position = new_position;
    }

    fn in_front(&self) -> Position {
        let (dx, dy) = match self.direction {
            GuardDirection::Up => (0isize, -1isize),
            GuardDirection::Right => (1, 0),
            GuardDirection::Down => (0, 1),
            GuardDirection::Left => (-1, 0),
        };
        self.position + (dx, dy)
    }
}

fn map_feature_parser(input: &mut &str) -> PResult<MapFeature> {
    alt((
        literal('.').value(MapFeature::Empty),
        literal('#').value(MapFeature::Obstruction),
        literal('^').value(MapFeature::UpFacingGuard),
    ))
    .parse_next(input)
}

fn row_parser(input: &mut &str) -> PResult<Vec<MapFeature>> {
    repeat(1.., map_feature_parser).parse_next(input)
}

#[derive(Debug, Clone)]
struct RawMap {
    rows: Vec<Vec<MapFeature>>,
}

impl FromStr for RawMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = separated(1.., row_parser, line_ending)
            .parse(s.trim())
            .map_err(|err| anyhow::format_err!("{err}"))?;

        Ok(RawMap { rows })
    }
}

impl TryFrom<RawMap> for Map {
    type Error = anyhow::Error;

    fn try_from(value: RawMap) -> Result<Self, Self::Error> {
        let mut obstacles = HashSet::new();
        let mut guard = Guard::default();

        let height = value.rows.len();
        let width = value.rows[0].len();
        for (y, row) in value.rows.into_iter().enumerate() {
            for (x, feature) in row.into_iter().enumerate() {
                let pos = Position::from((x, y));
                match feature {
                    MapFeature::Empty => {}
                    MapFeature::Obstruction => {
                        obstacles.insert(pos);
                    }
                    MapFeature::UpFacingGuard => {
                        guard.position = pos;
                    }
                }
            }
        }

        if guard.position.is_origin() {
            bail!("failed to find initial guard position")
        }

        Ok(Map {
            guard,
            obstacles,
            dimensions: Rect { width, height },
        })
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RawMap::from_str(s)?.try_into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug)]
pub struct Map {
    guard: Guard,
    obstacles: HashSet<Position>,
    pub dimensions: Rect,
}

impl Map {
    pub fn is_outside_map(&self, pos: Position) -> bool {
        pos.x < 0
            || pos.y < 0
            || pos.x >= self.dimensions.width as isize
            || pos.y >= self.dimensions.height as isize
    }

    pub fn has_obstacle(&self, position: Position) -> bool {
        self.obstacles.contains(&position)
    }

    pub fn move_guard(&mut self) -> Position {
        loop {
            let front = self.guard.in_front();
            if self.has_obstacle(front) {
                self.guard.rotate()
            } else {
                self.guard.move_to(front);
                return front;
            }
        }
    }

    pub fn guard_position(&self) -> Position {
        self.guard.position
    }

    pub fn test_loop(mut self) -> bool {
        let mut visited = HashSet::new();
        visited.insert(self.guard);

        loop {
            let next_guard_position = self.move_guard();
            if !visited.insert(self.guard) {
                return true;
            }
            if self.is_outside_map(next_guard_position) {
                return false;
            }
        }
    }

    pub fn new_with_obstacle(&self, obstacle: Position) -> Self {
        let mut tester = self.clone();
        tester.obstacles.insert(obstacle);
        tester
    }
}
