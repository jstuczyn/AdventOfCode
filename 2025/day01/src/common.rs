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
use std::str::FromStr;
use winnow::combinator::alt;
use winnow::{ModalResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Direction {
    Left,
    Right,
}

fn direction_parser(input: &mut &str) -> ModalResult<Direction> {
    alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))).parse_next(input)
}

fn rotation_parser(input: &mut &str) -> ModalResult<Rotation> {
    let (direction, magnitude) = (direction_parser, parse_number).parse_next(input)?;
    Ok(Rotation {
        direction,
        magnitude,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rotation {
    pub direction: Direction,
    pub magnitude: u16,
}

impl Rotation {
    pub fn apply(&self, dial_position: &mut u16) -> usize {
        let mut at_zero = 0;

        // 1. normalise rotation, i.e. remove full rotations
        // and count number of times it would have passed over the origin
        at_zero += (self.magnitude / 100) as usize;
        let normalised = self.magnitude % 100;
        let position = *dial_position;

        // 2. apply the rotation
        let new_dial = match self.direction {
            Direction::Left => {
                // will pass over the 0 position
                if normalised > position {
                    // don't count the cases where we're starting from 0
                    // those have already been accounted for
                    if position != 0 {
                        at_zero += 1;
                    }

                    let diff = normalised - position;
                    100 - diff
                } else {
                    position - normalised
                }
            }
            Direction::Right => {
                if position + normalised > 100 {
                    at_zero += 1;
                }
                (position + normalised) % 100
            }
        };

        *dial_position = new_dial;
        at_zero
    }
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rotation_parser
            .parse(s)
            .map_err(|err| anyhow::format_err!("{err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_parser() {
        assert_eq!(
            Rotation {
                direction: Direction::Right,
                magnitude: 3,
            },
            "R3".parse().unwrap()
        );

        assert_eq!(
            Rotation {
                direction: Direction::Left,
                magnitude: 69,
            },
            "L69".parse().unwrap()
        );
    }

    #[test]
    fn sample_left() {
        let mut dial = 14;
        Rotation {
            direction: Direction::Left,
            magnitude: 82,
        }
        .apply(&mut dial);
        assert_eq!(dial, 32);
    }

    #[test]
    fn multiple_left_rot() {
        let mut dial = 0;
        Rotation {
            direction: Direction::Left,
            magnitude: 1,
        }
        .apply(&mut dial);
        assert_eq!(dial, 99);

        Rotation {
            direction: Direction::Left,
            magnitude: 99,
        }
        .apply(&mut dial);
        assert_eq!(dial, 0);

        Rotation {
            direction: Direction::Left,
            magnitude: 100,
        }
        .apply(&mut dial);
        assert_eq!(dial, 0);

        Rotation {
            direction: Direction::Left,
            magnitude: 200,
        }
        .apply(&mut dial);
        assert_eq!(dial, 0);

        Rotation {
            direction: Direction::Left,
            magnitude: 201,
        }
        .apply(&mut dial);
        assert_eq!(dial, 99);
    }

    fn rot(raw: &str) -> Rotation {
        raw.parse().unwrap()
    }

    #[test]
    fn passing_over_zero() {
        let mut dial = 50;
        assert_eq!(1, rot("L68").apply(&mut dial));
        assert_eq!(0, rot("L30").apply(&mut dial));
        assert_eq!(0, rot("R48").apply(&mut dial));
        assert_eq!(dial, 0);
        assert_eq!(0, rot("L5").apply(&mut dial));
        assert_eq!(1, rot("R60").apply(&mut dial));
        assert_eq!(0, rot("L55").apply(&mut dial));
        assert_eq!(dial, 0);
        assert_eq!(0, rot("L1").apply(&mut dial));
        assert_eq!(0, rot("L99").apply(&mut dial));
        assert_eq!(dial, 0);
        assert_eq!(0, rot("R14").apply(&mut dial));
        assert_eq!(1, rot("L82").apply(&mut dial));
    }
}
