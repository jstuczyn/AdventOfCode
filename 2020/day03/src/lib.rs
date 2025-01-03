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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use anyhow::bail;
use aoc_solution::Aoc;
use std::borrow::Borrow;

#[derive(Aoc)]
pub struct Day03;

const EMPTY_STATE_SYMBOL: char = '.';
const TREE_STATE_SYMBOL: char = '#';
const PART1_SLOPE: (usize, usize) = (3, 1);

type Position = (usize, usize);

#[derive(Clone, Copy)]
pub enum Location {
    Empty,
    Tree,
}

impl Location {
    fn is_tree(&self) -> bool {
        matches!(self, Location::Tree)
    }
}

impl TryFrom<char> for Location {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            EMPTY_STATE_SYMBOL => Ok(Location::Empty),
            TREE_STATE_SYMBOL => Ok(Location::Tree),
            _ => bail!("invalid location"),
        }
    }
}

struct Row(Vec<Location>);

impl<'a> TryFrom<&'a String> for Row {
    type Error = anyhow::Error;

    fn try_from(raw: &'a String) -> Result<Self, Self::Error> {
        let chars = raw.chars();
        let size_hint = chars.size_hint();
        let mut row = Vec::with_capacity(size_hint.1.unwrap_or(size_hint.0));
        for location in raw.chars().map(|char| char.try_into()) {
            row.push(location?);
        }

        Ok(Row(row))
    }
}

struct Grid(Vec<Row>);

impl<'a> TryFrom<&'a [String]> for Grid {
    type Error = anyhow::Error;

    fn try_from(raw: &'a [String]) -> Result<Self, Self::Error> {
        let mut rows = Vec::with_capacity(raw.len());
        for raw_row in raw {
            rows.push(raw_row.try_into()?);
        }

        Ok(Grid(rows))
    }
}

impl std::ops::Index<Position> for Grid {
    type Output = Location;

    fn index(&self, index: Position) -> &Self::Output {
        let (x, y) = index;

        self.0[y].0[x].borrow()
    }
}

impl Grid {
    // TODO: what's the idiomatic way of creating a parameterized iterator?
    fn into_iterator(self, slope: (usize, usize)) -> GridIntoIterator {
        GridIntoIterator {
            slope,
            position: (0, 0),
            grid: self,
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn row_len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].0.len()
        }
    }
}

struct GridIntoIterator {
    slope: (usize, usize),
    position: Position,
    grid: Grid,
}

// as per part1 specs
impl Iterator for GridIntoIterator {
    type Item = Location;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.1 >= self.grid.len() {
            None
        } else {
            let location = self.grid[self.position];
            self.position = (
                (self.position.0 + self.slope.0) % self.grid.row_len(),
                self.position.1 + self.slope.1,
            );
            Some(location)
        }
    }
}

pub fn part1(input: Vec<String>) -> Option<usize> {
    Some(
        Grid::try_from(input.as_slice())
            .ok()?
            .into_iterator(PART1_SLOPE)
            .filter(|location| location.is_tree())
            .count(),
    )
}

pub fn part2(input: Vec<String>) -> Option<usize> {
    let slopes = vec![(1usize, 1usize), (3, 1), (5, 1), (7, 1), (1, 2)];
    // can't do it with a single iterator due to trying to catch errors with `?` : (
    let mut running_total = 1;
    for slope in slopes {
        running_total *= Grid::try_from(input.as_slice())
            .ok()?
            .into_iterator(slope)
            .filter(|location| location.is_tree())
            .count();
    }

    Some(running_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        let expected = 7;

        assert_eq!(expected, part1(input).unwrap())
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        let expected = 336;

        assert_eq!(expected, part2(input).unwrap())
    }

    #[test]
    fn fails_to_parse_invalid_location() {
        assert!(Location::try_from('a').is_err())
    }

    #[test]
    fn grid_returns_0_row_if_empty() {
        assert_eq!(0, Grid(Vec::new()).row_len())
    }
}
