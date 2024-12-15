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

use crate::constants::{EMPTY_PIXEL, FILLED_PIXEL};
use anyhow::bail;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Pixel {
    Active,
    #[default]
    Inactive,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        char::from(*self).fmt(f)
    }
}

impl Pixel {
    pub fn unchecked_from_char(c: char) -> Self {
        c.try_into().unwrap()
    }

    pub fn to_readable(&self) -> char {
        match self {
            // use those unicode characters instead of the original ones
            // for way better readability
            Pixel::Active => FILLED_PIXEL,
            Pixel::Inactive => EMPTY_PIXEL,
        }
    }
}

impl TryFrom<char> for Pixel {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Pixel::Active),
            '.' => Ok(Pixel::Inactive),
            other => bail!("{other} is not a valid pixel value"),
        }
    }
}

impl From<Pixel> for char {
    fn from(pixel: Pixel) -> Self {
        match pixel {
            Pixel::Active => '#',
            Pixel::Inactive => '.',
        }
    }
}

impl Pixel {
    pub fn is_active(&self) -> bool {
        matches!(self, Pixel::Active)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(isize, isize)> for Position {
    type Output = Position;

    fn add(self, (dx, dy): (isize, isize)) -> Self::Output {
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl AddAssign<(isize, isize)> for Position {
    fn add_assign(&mut self, (dx, dy): (isize, isize)) {
        self.x += dx;
        self.y += dy;
    }
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Position {
            x: x as isize,
            y: y as isize,
        }
    }
}

impl From<Position> for (isize, isize) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

impl Position {
    pub const fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub const fn next_horizontal(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub const fn previous_horizontal(&self) -> Position {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub const fn next_vertical(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub const fn previous_vertical(&self) -> Position {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }

    #[inline]
    pub const fn adjacent(&self) -> [Position; 8] {
        [
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }

    #[inline]
    pub const fn is_in_quadrant1(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    // for tasks operating in Q1
    pub fn quadrant1_adjacent(&self) -> impl Iterator<Item = Position> {
        self.adjacent().into_iter().filter(|a| a.is_in_quadrant1())
    }
}
