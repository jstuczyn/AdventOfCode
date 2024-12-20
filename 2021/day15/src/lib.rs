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

use aoc_common::parsing::FromStrParser;
use aoc_solution::Aoc;
use pathfinding::prelude::dijkstra;
use std::ops::Index;
use std::str::FromStr;

#[derive(Aoc)]
#[aoc(input = RiskLevelMap)]
#[aoc(parser = FromStrParser)]
#[aoc(part1(output = usize, runner = part1))]
#[aoc(part2(output = usize, runner = part2))]
pub struct Day15;

#[derive(Debug, Clone)]
pub struct RiskLevelMap {
    rows: Vec<Vec<usize>>,
}

type Pos = (usize, usize);

impl FromStr for RiskLevelMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<_>> = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Ok(Self { rows })
    }
}

impl Index<Pos> for RiskLevelMap {
    type Output = usize;

    fn index(&self, index: Pos) -> &Self::Output {
        let (x, y) = index;
        &self.rows[y][x]
    }
}

impl RiskLevelMap {
    fn lowest_risk_path_cost(&self) -> usize {
        let start = (0usize, 0usize);
        let end = (self.rows[0].len() - 1, self.rows.len() - 1);
        let (_, cost) = dijkstra(&start, |pos| self.node_successors(pos), |&p| p == end).unwrap();

        cost
    }

    fn node_successors(&self, node: &Pos) -> Vec<(Pos, usize)> {
        let mut successors = Vec::new();
        if node.0 > 0 {
            let left = (node.0 - 1, node.1);
            successors.push((left, self[left]))
        }

        if node.0 < self.rows[0].len() - 1 {
            let right = (node.0 + 1, node.1);
            successors.push((right, self[right]))
        }

        if node.1 > 0 {
            let top = (node.0, node.1 - 1);
            successors.push((top, self[top]))
        }

        if node.1 < self.rows.len() - 1 {
            let bottom = (node.0, node.1 + 1);
            successors.push((bottom, self[bottom]))
        }

        successors
    }

    fn map_value(i: usize, val: usize) -> usize {
        if i == 0 {
            val
        } else {
            let res = val + i;
            if res > 9 {
                res - 9
            } else {
                res
            }
        }
    }

    fn expand_row_five_folds(&mut self, row: usize) {
        let old = std::mem::take(&mut self.rows[row]);
        self.rows[row] = std::iter::repeat(old)
            .take(5)
            .enumerate()
            .flat_map(|(i, vals)| vals.into_iter().map(move |v| Self::map_value(i, v)))
            .collect::<Vec<_>>();
    }

    fn expand_columns_five_folds(&mut self) {
        let rows = self.rows.clone();
        for i in 1..=4 {
            for row in rows.clone() {
                let new_row = row
                    .clone()
                    .into_iter()
                    .map(|v| Self::map_value(i, v))
                    .collect();
                self.rows.push(new_row);
            }
        }
    }

    fn expand_five_folds(&mut self) {
        for i in 0..self.rows.len() {
            self.expand_row_five_folds(i)
        }
        self.expand_columns_five_folds()
    }
}

pub fn part1(risk_map: RiskLevelMap) -> usize {
    risk_map.lowest_risk_path_cost()
}

pub fn part2(mut risk_map: RiskLevelMap) -> usize {
    risk_map.expand_five_folds();
    risk_map.lowest_risk_path_cost()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .parse()
            .unwrap();

        let expected = 40;
        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .parse()
            .unwrap();

        let expected = 315;
        assert_eq!(expected, part2(input))
    }
}
