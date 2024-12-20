// Copyright 2020 Jedrzej Stuczynski
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

use aoc_solution::Aoc;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign};

#[derive(Aoc)]
pub struct Day17;

// Point contains list of values for each dimension
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point(pub Vec<isize>);

impl Point {
    pub fn base(dims: usize) -> Point {
        Point(vec![0; dims])
    }

    pub fn dimension_adjacent(&self, dim: usize) -> Vec<Point> {
        let mut positively_adjacent = self.clone();
        positively_adjacent.0[dim - 1] += 1;

        let mut negatively_adjacent = self.clone();
        negatively_adjacent.0[dim - 1] -= 1;

        vec![positively_adjacent, negatively_adjacent]
    }

    pub fn adjacent_points(&self) -> Vec<Point> {
        let mut adjacent = Vec::with_capacity(3usize.pow(self.0.len() as u32 - 1));

        for dim in 1..=self.0.len() {
            let mut dim_adjacent = Vec::new();
            for adj in adjacent.iter().chain(std::iter::once(self)) {
                dim_adjacent.append(&mut adj.dimension_adjacent(dim))
            }
            adjacent.extend(dim_adjacent.into_iter().filter(|p| p != self))
        }

        adjacent
    }
}

impl Add<(isize, isize, isize)> for &Point {
    type Output = Point;

    fn add(self, rhs: (isize, isize, isize)) -> Self::Output {
        assert_eq!(3, self.0.len());

        Point(vec![
            self.0[0] + rhs.0,
            self.0[1] + rhs.1,
            self.0[2] + rhs.2,
        ])
    }
}

impl AddAssign<(isize, isize, isize)> for Point {
    fn add_assign(&mut self, rhs: (isize, isize, isize)) {
        assert_eq!(3, self.0.len());

        self.0[0] += rhs.0;
        self.0[1] += rhs.1;
        self.0[2] += rhs.2;
    }
}

const ACTIVE_CUBE: char = '#';
const NUM_CYCLES: usize = 6;

fn parse_initial_data(input: &[String], dims: usize) -> HashSet<Point> {
    let mut active_cubes = HashSet::new();
    for (y, raw_row) in input.iter().enumerate() {
        raw_row.chars().enumerate().for_each(|(x, char)| {
            if char == ACTIVE_CUBE {
                let mut coords = vec![0; dims];
                coords[0] = x as isize;
                coords[1] = y as isize;
                active_cubes.insert(Point(coords));
            }
        })
    }

    active_cubes
}

struct SimulatedPoint {
    point: Point,
    neighbours: Vec<Point>,
    should_deactivate: bool,
}

fn simulate_step_par(active_points: &mut HashSet<Point>) {
    let simulated_points: Vec<_> = active_points
        .par_iter()
        .map(|active_point| {
            let neighbours = active_point.adjacent_points();
            let active_neighbours = neighbours
                .par_iter()
                .map(|neighbour| active_points.contains(neighbour))
                .filter(|is_active| *is_active)
                .count();

            SimulatedPoint {
                point: active_point.clone(),
                neighbours,
                should_deactivate: active_neighbours != 2 && active_neighbours != 3,
            }
        })
        .collect();

    let mut all_adjacents = HashMap::new();
    for simulated_point in simulated_points {
        if simulated_point.should_deactivate {
            active_points.remove(&simulated_point.point);
        }
        for neighbour in simulated_point.neighbours {
            *all_adjacents.entry(neighbour).or_insert(0) += 1;
        }
    }

    for (adjacent, count) in all_adjacents.into_iter() {
        if count == 3 {
            active_points.insert(adjacent);
        }
    }
}

fn simulate_step(active_points: &mut HashSet<Point>) {
    let mut adjacent_points = HashMap::new();

    let mut points_to_deactivate = Vec::new();

    for active_point in active_points.iter() {
        let adjacents = active_point.adjacent_points();
        let mut active = 0;
        for adjacent in adjacents {
            if active_points.contains(&adjacent) {
                active += 1;
            }
            let entry = adjacent_points.entry(adjacent).or_insert(0);
            *entry += 1;
        }
        if active != 2 && active != 3 {
            points_to_deactivate.push(active_point.clone());
        }
    }

    for (adjacent, count) in adjacent_points.into_iter() {
        if count == 3 {
            active_points.insert(adjacent);
        }
    }

    for deactivate in points_to_deactivate.into_iter() {
        active_points.remove(&deactivate);
    }
}

pub fn part1(input: Vec<String>) -> usize {
    let mut active_points = parse_initial_data(&input, 3);

    for _ in 0..NUM_CYCLES {
        simulate_step(&mut active_points);
    }

    active_points.len()
}

pub fn part2(input: Vec<String>) -> usize {
    let mut active_points = parse_initial_data(&input, 4);

    for _ in 0..NUM_CYCLES {
        simulate_step_par(&mut active_points);
    }

    active_points.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];

        let expected = 112;

        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];

        let expected = 848;

        assert_eq!(expected, part2(input))
    }
}
