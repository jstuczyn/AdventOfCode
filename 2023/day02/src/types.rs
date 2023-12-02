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
use std::cmp::max;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub id: usize,
    sets: Vec<CubeSet>,
}

impl Game {
    pub fn is_possible(&self, content: CubeSet) -> bool {
        self.sets.iter().all(|s| s.could_contain(&content))
    }

    pub fn minimal_set(self) -> CubeSet {
        self.sets
            .into_iter()
            .fold(CubeSet::default(), |acc, next| acc.reduce(next))
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    pub const fn new(red: usize, green: usize, blue: usize) -> Self {
        CubeSet { red, green, blue }
    }

    pub const fn could_contain(&self, other: &Self) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    pub fn power(&self) -> usize {
        self.red * self.blue * self.green
    }

    pub fn reduce(mut self, other: Self) -> Self {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);

        self
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_with_id, raw_sets) = s
            .split_once(": ")
            .ok_or(anyhow!("the game input is malformed"))?;
        let id = game_with_id
            .strip_prefix("Game ")
            .ok_or(anyhow!("the game input is malformed - no 'Game ' prefix"))?
            .parse()?;

        let sets = raw_sets
            .split("; ")
            .map(CubeSet::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Game { id, sets })
    }
}

impl FromStr for CubeSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for colour_value in s.split(", ") {
            let (value, colour) = colour_value
                .split_once(' ')
                .ok_or(anyhow!("{colour_value} is not a valid color value"))?;
            match colour {
                "red" => red = value.parse()?,
                "green" => green = value.parse()?,
                "blue" => blue = value.parse()?,
                other => bail!("{other} is not a valid colour"),
            }
        }

        Ok(CubeSet { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_parsing() {
        let set1 = CubeSet {
            red: 6,
            green: 0,
            blue: 0,
        };
        assert_eq!(set1, "6 red".parse().unwrap());

        let set2 = CubeSet {
            red: 6,
            green: 0,
            blue: 1,
        };
        assert_eq!(set2, "6 red, 1 blue".parse().unwrap());

        let set3 = CubeSet {
            red: 1,
            green: 4,
            blue: 0,
        };
        assert_eq!(set3, "4 green, 1 red".parse().unwrap());
    }

    #[test]
    fn game_parsing() {
        let expected = Game {
            id: 1,
            sets: vec![
                CubeSet {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                CubeSet {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                CubeSet {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };
        assert_eq!(
            expected,
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                .parse()
                .unwrap()
        )
    }
}
