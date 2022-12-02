// Copyright 2022 Jedrzej Stuczynski
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
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RoundResult {
    Win,
    Draw,
    Loss,
}

impl FromStr for RoundResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == "X" => Ok(RoundResult::Loss),
            s if s == "Y" => Ok(RoundResult::Draw),
            s if s == "Z" => Ok(RoundResult::Win),
            s => Err(anyhow!("{s} is not a valid round result")),
        }
    }
}

impl RoundResult {
    fn score(&self) -> usize {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }

    fn required_shape(&self, against: &Shape) -> Shape {
        if self == &RoundResult::Draw {
            return *against;
        }

        match against {
            Shape::Rock => {
                if self == &RoundResult::Loss {
                    Shape::Scissors
                } else {
                    Shape::Paper
                }
            }
            Shape::Paper => {
                if self == &RoundResult::Loss {
                    Shape::Rock
                } else {
                    Shape::Scissors
                }
            }
            Shape::Scissors => {
                if self == &RoundResult::Loss {
                    Shape::Paper
                } else {
                    Shape::Rock
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == "A" || s == "X" => Ok(Shape::Rock),
            s if s == "B" || s == "Y" => Ok(Shape::Paper),
            s if s == "C" || s == "Z" => Ok(Shape::Scissors),
            s => Err(anyhow!("{s} is not a valid rock-paper-scissors shape")),
        }
    }
}

impl Shape {
    fn shape_score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_against(&self, other: &Self) -> RoundResult {
        if self == other {
            return RoundResult::Draw;
        }

        match self {
            Shape::Rock => {
                if other == &Shape::Paper {
                    RoundResult::Loss
                } else {
                    RoundResult::Win
                }
            }
            Shape::Paper => {
                if other == &Shape::Scissors {
                    RoundResult::Loss
                } else {
                    RoundResult::Win
                }
            }
            Shape::Scissors => {
                if other == &Shape::Rock {
                    RoundResult::Loss
                } else {
                    RoundResult::Win
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ShapeOrResult {
    shape: Shape,
    result: RoundResult,
}

impl FromStr for ShapeOrResult {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ShapeOrResult {
            shape: s.parse()?,
            result: s.parse()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RPSGame {
    opponent: Shape,
    shape_or_result: ShapeOrResult,
}

impl RPSGame {
    pub(crate) fn play_as_shape(&self) -> usize {
        let shape = self.shape_or_result.shape;
        shape.shape_score() + shape.play_against(&self.opponent).score()
    }

    pub(crate) fn play_as_result(&self) -> usize {
        let result = self.shape_or_result.result;
        let shape = result.required_shape(&self.opponent);

        shape.shape_score() + result.score()
    }
}

impl FromStr for RPSGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();

        let opponent = split
            .next()
            .ok_or_else(|| anyhow!("input did not contain the first rock-paper-scissors item"))?
            .parse()?;
        let shape_or_result = split
            .next()
            .ok_or_else(|| anyhow!("input did not contain the second rock-paper-scissors item"))?
            .parse()?;

        if split.next().is_some() {
            bail!("input seems to be malformed! there are additional shapes present")
        }

        Ok(RPSGame {
            opponent,
            shape_or_result,
        })
    }
}
