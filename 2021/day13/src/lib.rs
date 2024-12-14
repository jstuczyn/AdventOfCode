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

use anyhow::{bail, Context};
use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;
use std::collections::{BTreeSet, VecDeque};
use std::str::FromStr;

#[derive(Aoc)]
#[aoc(input = Manual)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = String, runner = part2))]
pub struct Day13;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().context("malformed point")?.parse()?;
        let y = split.next().context("malformed point")?.parse()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone)]
pub struct Fold {
    axis: Axis,
    at: usize,
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = s.strip_prefix("fold along ").context("malformed fold")?;
        let mut split = stripped.split('=');
        let axis = match split.next().context("malformed fold")? {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => bail!("malformed fold"),
        };
        let at = split.next().context("malformed fold")?.parse()?;

        Ok(Fold { axis, at })
    }
}

#[derive(Debug, Clone)]
pub struct Manual {
    points: BTreeSet<Point>,
    folds: VecDeque<Fold>,
}

impl FromStr for Manual {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .replace("\r\n", "\n") // Windows fix
            .split("\n\n")
            .map(|split| split.to_owned())
            .collect::<Vec<_>>();

        let points = lines[0].lines().map(|s| s.parse().unwrap()).collect();
        let folds = lines[1].lines().map(|s| s.parse().unwrap()).collect();

        Ok(Manual { points, folds })
    }
}

impl Manual {
    #[cfg(test)]
    fn from_raw(raw: &[String]) -> Manual {
        let points = raw[0].lines().map(|s| s.parse().unwrap()).collect();
        let folds = raw[1].lines().map(|s| s.parse().unwrap()).collect();

        Manual { points, folds }
    }

    fn fold_at_y_axis(&mut self, at: usize) {
        let mut new_points: BTreeSet<Point> = self
            .points
            .iter()
            .filter(|point| point.y < at)
            .copied()
            .collect();
        for point in &self.points {
            if point.y > at {
                new_points.insert(Point {
                    x: point.x,
                    y: 2 * at - point.y,
                });
            }
        }

        self.points = new_points
    }

    fn fold_at_x_axis(&mut self, at: usize) {
        let mut new_points: BTreeSet<Point> = self
            .points
            .iter()
            .filter(|point| point.x < at)
            .copied()
            .collect();
        for point in &self.points {
            if point.x > at {
                new_points.insert(Point {
                    x: 2 * at - point.x,
                    y: point.y,
                });
            }
        }

        self.points = new_points
    }

    fn fold(&mut self) -> bool {
        if let Some(fold) = self.folds.pop_front() {
            if fold.axis == Axis::Y {
                self.fold_at_y_axis(fold.at)
            } else {
                self.fold_at_x_axis(fold.at)
            }
            true
        } else {
            false
        }
    }

    fn final_manual(&self) -> String {
        let max_x = self.points.iter().max_by_key(|point| point.x).unwrap().x;
        let max_y = self.points.iter().max_by_key(|point| point.y).unwrap().y;
        let mut out = vec![String::new()];
        for y in 0..=max_y {
            let mut row = Vec::with_capacity(max_x);
            for x in 0..=max_x {
                if self.points.contains(&Point { x, y }) {
                    row.push('█');
                } else {
                    row.push('⠀')
                }
            }
            out.push(row.into_iter().collect::<String>())
        }
        out.join("\n")
    }
}

pub fn part1(mut manual: Manual) -> usize {
    manual.fold();
    manual.points.len()
}

pub fn part2(mut manual: Manual) -> String {
    while manual.fold() {}
    manual.final_manual()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"
            .to_string(),
            "fold along y=7
fold along x=5"
                .to_string(),
        ];

        let manual = Manual::from_raw(&input);
        let expected = 17;

        assert_eq!(expected, part1(manual))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"
            .to_string(),
            "fold along y=7
fold along x=5"
                .to_string(),
        ];

        let manual = Manual::from_raw(&input);
        let expected = r#"
█████
█⠀⠀⠀█
█⠀⠀⠀█
█⠀⠀⠀█
█████"#;

        assert_eq!(expected, part2(manual))
    }
}
