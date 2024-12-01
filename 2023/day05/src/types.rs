// Copyright 2023 Jedrzej Stuczynski
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

use anyhow::{anyhow, bail};
use common::parsing::split_to_string_groups;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Seeds {
    Individual(Vec<usize>),
    Ranges(Vec<Range<usize>>),
    Invalid,
}

#[derive(Debug, Clone)]
pub struct Almanac {
    pub seeds: Seeds,
    seed_soil_map: Map,
    soil_fertilizer_map: Map,
    fertilizer_water_map: Map,
    water_light_map: Map,
    light_temperature_map: Map,
    temperature_humidity_map: Map,
    humidity_location: Map,
}

#[derive(Debug, Clone)]
pub struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    // assumption: entries don't overlap each-other
    pub fn map_to_destination(&self, source: usize) -> usize {
        self.entries
            .iter()
            .find(|e| e.source_range.contains(&source))
            .map(|entry| entry.map_to_destination(source))
            .unwrap_or(source)
    }
}

#[derive(Debug, Clone)]
pub struct MapEntry {
    source_range: Range<usize>,
    destination_range_start: usize,
}

impl MapEntry {
    pub fn map_to_destination(&self, value: usize) -> usize {
        debug_assert!(self.source_range.contains(&value));
        let offset = value - self.source_range.start;

        self.destination_range_start + offset
    }
}

impl Almanac {
    pub fn convert_seeds_to_ranges(&mut self) {
        let seeds = match std::mem::replace(&mut self.seeds, Seeds::Invalid) {
            Seeds::Individual(seeds) => seeds,
            _ => unreachable!(),
        };

        let ranges = seeds
            .into_iter()
            .tuples()
            .map(|(start, length)| start..start + length)
            .collect();
        self.seeds = Seeds::Ranges(ranges);
    }

    pub fn lowest_location(&self) -> usize {
        match &self.seeds {
            Seeds::Individual(seeds) => seeds
                .iter()
                .map(|&s| self.seed_to_location(s))
                .min()
                .unwrap_or(usize::MAX),
            Seeds::Ranges(ranges) => {
                let ranges = ranges.clone();

                // if we can't create a new runtime, then we have a problem and have to abort
                #[allow(clippy::expect_used)]
                let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
                let this = Arc::new(self.clone());

                rt.block_on(async {
                    let mut tasks = ranges
                        .into_iter()
                        .map(|range| {
                            let that = Arc::clone(&this);
                            tokio::spawn(async move {
                                range
                                    .into_par_iter()
                                    .map(|s| that.seed_to_location(s))
                                    .min()
                                    .unwrap_or(usize::MAX)
                            })
                        })
                        .collect::<FuturesUnordered<_>>();

                    // wait for all futures to finish
                    let mut min = usize::MAX;
                    while let Some(completed) = tasks.next().await {
                        // if we fail to execute the future, we have reached some degenerate case
                        // and have to abort as we can't trust our answer
                        #[allow(clippy::expect_used)]
                        let result = completed.expect("execution failure");
                        if result < min {
                            min = result
                        }
                    }

                    min
                })
            }
            Seeds::Invalid => unreachable!(),
        }
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_soil_map.map_to_destination(seed);
        let fertilizer = self.soil_fertilizer_map.map_to_destination(soil);
        let water = self.fertilizer_water_map.map_to_destination(fertilizer);
        let light = self.water_light_map.map_to_destination(water);
        let temperature = self.light_temperature_map.map_to_destination(light);
        let humidity = self
            .temperature_humidity_map
            .map_to_destination(temperature);
        self.humidity_location.map_to_destination(humidity)
    }
}

impl FromStr for MapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raw = s.split_ascii_whitespace();
        let destination_range_start = raw
            .next()
            .ok_or(anyhow!("no source range start"))?
            .parse()?;
        let source_start = raw
            .next()
            .ok_or(anyhow!("no destination range start"))?
            .parse()?;
        let range_length: usize = raw.next().ok_or(anyhow!("no range length"))?.parse()?;

        Ok(MapEntry {
            source_range: source_start..source_start + range_length,
            destination_range_start,
        })
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = s
            .lines()
            .skip(1)
            .map(MapEntry::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        entries.sort_unstable_by_key(|e| e.source_range.start);

        Ok(Map { entries })
    }
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = split_to_string_groups(s);
        if groups.len() != 8 {
            bail!("malformed input - we don't have 8 string groups")
        }

        let seeds = groups[0]
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("no seeds"))?
            .split_ascii_whitespace()
            .map(FromStr::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Almanac {
            seeds: Seeds::Individual(seeds),
            seed_soil_map: groups[1].parse()?,
            soil_fertilizer_map: groups[2].parse()?,
            fertilizer_water_map: groups[3].parse()?,
            water_light_map: groups[4].parse()?,
            light_temperature_map: groups[5].parse()?,
            temperature_humidity_map: groups[6].parse()?,
            humidity_location: groups[7].parse()?,
        })
    }
}
