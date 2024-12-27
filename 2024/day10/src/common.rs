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

use aoc_common::parsing::combinators::parse_digit;
use aoc_common::types::{Grid, ParsableGridItem, Position};
use std::collections::HashMap;
use std::str::FromStr;
use winnow::PResult;

#[derive(Debug, Copy, Clone)]
pub struct Hill {
    height: usize,
}

impl ParsableGridItem for Hill {
    const PARSER: fn(&mut &str) -> PResult<Self> =
        |input| parse_digit(input).map(|height| Hill { height });
}

#[derive(Clone, Debug)]
pub struct TopographicMap {
    inner: Grid<Hill>,
}

impl FromStr for TopographicMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TopographicMap { inner: s.parse()? })
    }
}

impl TopographicMap {
    fn find_trailhead_starts(&self) -> impl Iterator<Item = (Position, Hill)> + '_ {
        self.inner
            .iter()
            .filter(|item| item.height == 0)
            .map(|item| (item.position, *item.item))
    }

    // given the number of trails, overhead of hashset is bigger than linear lookup of a vector
    fn check_subtrail(
        &self,
        (position, hill): (Position, Hill),
        consider_alternative_paths: bool,
        reachable_trails: &mut HashMap<Position, Vec<Position>>,
    ) -> Vec<Position> {
        if let Some(cached) = reachable_trails.get(&position) {
            return cached.clone();
        }

        let mut trail_ends = Vec::new();
        for adj in position.cardinal_adjacent() {
            if let Some(&adj_hill) = self.inner.get(adj) {
                if adj_hill.height == hill.height + 1 {
                    if adj_hill.height == 9 {
                        trail_ends.push(adj);
                        continue;
                    }
                    let reachable = self.check_subtrail(
                        (adj, adj_hill),
                        consider_alternative_paths,
                        reachable_trails,
                    );
                    for end in reachable {
                        if consider_alternative_paths || !trail_ends.contains(&end) {
                            trail_ends.push(end);
                        }
                    }
                }
            }
        }

        reachable_trails.insert(position, trail_ends.clone());
        trail_ends
    }

    pub fn trailheads_score(&self) -> usize {
        let mut trail_map = HashMap::new();
        self.find_trailhead_starts()
            .map(|start| self.check_subtrail(start, false, &mut trail_map).len())
            .sum()
    }

    pub fn trailheads_rating(&self) -> usize {
        let mut trail_map = HashMap::new();
        self.find_trailhead_starts()
            .map(|start| self.check_subtrail(start, true, &mut trail_map).len())
            .sum()
    }
}
