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

use aoc_solution::Aoc;
use std::fmt::{self, Display, Formatter};
use std::mem;
use std::ops::{Index, IndexMut};

#[derive(Aoc)]
pub struct Day11;

const EMPTY_SEAT: char = 'L';
const OCCUPIED_SEAT: char = '#';
const FLOOR: char = '.';

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(value: char) -> Self {
        match value {
            EMPTY_SEAT => Seat::Empty,
            OCCUPIED_SEAT => Seat::Occupied,
            FLOOR => Seat::Floor,
            v => panic!("invalid seat state {}", v),
        }
    }
}

impl From<Seat> for char {
    fn from(seat: Seat) -> Self {
        match seat {
            Seat::Empty => EMPTY_SEAT,
            Seat::Occupied => OCCUPIED_SEAT,
            Seat::Floor => FLOOR,
        }
    }
}

impl Seat {
    fn swap(&mut self) {
        mem::swap(
            self,
            &mut match self {
                Seat::Empty => Seat::Occupied,
                Seat::Occupied => Seat::Empty,
                Seat::Floor => Seat::Floor,
            },
        );
    }

    fn is_floor(&self) -> bool {
        matches!(self, Seat::Floor)
    }

    fn is_empty(&self) -> bool {
        matches!(self, Seat::Empty)
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Seat::Occupied)
    }
}

type SeatRow = Vec<Seat>;
type SeatPosition = (usize, usize);

#[derive(PartialEq)]
pub struct SeatGrid {
    rows: Vec<SeatRow>,
}

impl Index<SeatPosition> for SeatGrid {
    type Output = Seat;

    fn index(&self, index: SeatPosition) -> &Self::Output {
        &self.rows[index.1][index.0]
    }
}

impl IndexMut<SeatPosition> for SeatGrid {
    fn index_mut(&mut self, index: SeatPosition) -> &mut Self::Output {
        &mut self.rows[index.1][index.0]
    }
}

impl Display for SeatGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            let row_string: String = row
                .iter()
                .map(|&seat| {
                    let char: char = seat.into();
                    char
                })
                .collect();
            writeln!(f, "{}", row_string)?;
        }
        Ok(())
    }
}

impl From<&[String]> for SeatGrid {
    fn from(raw_rows: &[String]) -> Self {
        Self {
            rows: raw_rows
                .iter()
                .map(|row| row.chars().map(Seat::from).collect())
                .collect(),
        }
    }
}

impl SeatGrid {
    fn immediately_adjacent(&self, position: SeatPosition) -> Vec<Seat> {
        let mut adjacent = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(seat) = self.attempt_seat_lookup(position, (i, j)) {
                    adjacent.push(seat)
                }
            }
        }

        adjacent
    }

    fn visibly_adjacent(&self, position: SeatPosition) -> Vec<Seat> {
        let mut adjacent = Vec::new();

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut translation = (i, j);
                while let Some(seat) = self.attempt_seat_lookup(position, translation) {
                    if !seat.is_floor() {
                        adjacent.push(seat);
                        break;
                    } else {
                        translation.0 += i;
                        translation.1 += j;
                    }
                }
            }
        }

        adjacent
    }

    fn attempt_seat_lookup(
        &self,
        position: SeatPosition,
        translation: (isize, isize),
    ) -> Option<Seat> {
        let (x, y) = position;
        let (dx, dy) = translation;
        let translated = (x as isize + dx, y as isize + dy);

        if translated.0 < 0
            || translated.0 >= self.rows[0].len() as isize
            || translated.1 < 0
            || translated.1 >= self.rows.len() as isize
        {
            None
        } else {
            // based on previous checks we know we can safely cast it
            let new_position = (translated.0 as usize, translated.1 as usize);

            Some(self[new_position])
        }
    }

    fn simulate_step<F, C>(&self, adjacent_seats: F, seat_checker: C) -> Self
    where
        F: Fn(&SeatGrid, SeatPosition) -> Vec<Seat>,
        C: Fn(&Seat, &[Seat]) -> bool,
    {
        let mut new_grid = SeatGrid {
            rows: self.rows.clone(),
        };

        self.rows.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, seat)| {
                if !seat.is_floor() {
                    let adjacent = adjacent_seats(self, (x, y));

                    if seat_checker(seat, &adjacent) {
                        new_grid[(x, y)].swap();
                    }
                }
            })
        });

        new_grid
    }

    fn occupied_count(&self) -> usize {
        self.rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|seat| seat.is_occupied())
            .count()
    }
}

pub fn part1(input: Vec<String>) -> usize {
    let seat_checker = |seat: &Seat, adjacent: &[Seat]| {
        // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        if seat.is_empty() && !adjacent.iter().any(|adj| adj.is_occupied()) {
            return true;
        }
        // If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
        if seat.is_occupied() && adjacent.iter().filter(|seat| seat.is_occupied()).count() >= 4 {
            return true;
        }
        false
    };

    let mut grid = SeatGrid::from(input.as_slice());
    loop {
        let next_grid = grid.simulate_step(SeatGrid::immediately_adjacent, seat_checker);
        if next_grid == grid {
            break;
        }
        grid = next_grid;
    }
    grid.occupied_count()
}

pub fn part2(input: Vec<String>) -> usize {
    let seat_checker = |seat: &Seat, adjacent: &[Seat]| {
        // If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
        if seat.is_empty() && !adjacent.iter().any(|adj| adj.is_occupied()) {
            return true;
        }
        // it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules)
        if seat.is_occupied() && adjacent.iter().filter(|seat| seat.is_occupied()).count() >= 5 {
            return true;
        }
        false
    };

    let mut grid = SeatGrid::from(input.as_slice());
    loop {
        let next_grid = grid.simulate_step(SeatGrid::visibly_adjacent, seat_checker);
        if next_grid == grid {
            break;
        }
        grid = next_grid;
    }
    grid.occupied_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ];

        let expected = 37;

        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ];

        let expected = 26;

        assert_eq!(expected, part2(input))
    }

    #[test]
    fn display_works_as_expected() {
        let input = vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ];
        let grid = SeatGrid::from(&*input);

        let mut expected = input.join("\n");
        expected.push('\n');

        assert_eq!(expected, format!("{}", grid));
    }
}
