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
