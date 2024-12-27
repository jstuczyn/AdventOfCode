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
use std::ops::{Add, AddAssign, Deref, Index, IndexMut, Sub, SubAssign};
use std::str::FromStr;
use std::vec::IntoIter;
use winnow::ascii::line_ending;
use winnow::combinator::{repeat, separated};
use winnow::error::ParserError;
use winnow::stream::{Compare, Stream, StreamIsPartial};
use winnow::{PResult, Parser};

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

impl Sub<(isize, isize)> for Position {
    type Output = Position;

    fn sub(self, (dx, dy): (isize, isize)) -> Self::Output {
        Position {
            x: self.x - dx,
            y: self.y - dy,
        }
    }
}

impl SubAssign<(isize, isize)> for Position {
    fn sub_assign(&mut self, (dx, dy): (isize, isize)) {
        self.x -= dx;
        self.y -= dy;
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

impl From<(isize, isize)> for Position {
    fn from((x, y): (isize, isize)) -> Self {
        Position { x, y }
    }
}

impl From<Position> for (isize, isize) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

impl Position {
    pub const fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    pub const fn origin() -> Self {
        Position { x: 0, y: 0 }
    }

    pub const fn unchecked_into_q1(&self) -> (usize, usize) {
        self.as_q1().expect("invalid q1 cast")
    }

    pub const fn as_q1(&self) -> Option<(usize, usize)> {
        if !self.is_in_quadrant1() {
            None
        } else {
            Some((self.x as usize, self.y as usize))
        }
    }

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
    pub const fn cardinal_adjacent(&self) -> [Position; 4] {
        [
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
        ]
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

#[derive(Debug, Clone, PartialEq)]
pub struct PositionedItem<T> {
    pub position: Position,
    pub item: T,
}

impl<T> Deref for PositionedItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn iter(&self) -> GridIterator<T> {
        GridIterator {
            next: Default::default(),
            grid: self,
        }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        let (x, y) = position.as_q1()?;
        self.rows.get(y).and_then(|row| row.get(x))
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        let (x, y) = position.as_q1()?;
        self.rows.get_mut(y).and_then(|row| row.get_mut(x))
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        self.index(index.unchecked_into_q1())
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.index_mut(index.unchecked_into_q1())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.rows[y][x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.rows[y][x]
    }
}

pub trait ParsableGridItem: Sized {
    const PARSER: fn(&mut &str) -> PResult<Self>;
}

impl<T> FromStr for Grid<T>
where
    T: ParsableGridItem,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        grid_parser(T::PARSER)
            .parse(s.trim())
            .map_err(|err| anyhow::format_err!("{err}"))
    }
}

pub fn grid_parser<T, Input, Error, P>(item_parser: P) -> impl Parser<Input, Grid<T>, Error>
where
    Input: StreamIsPartial + Stream + Compare<&'static str>,
    P: Parser<Input, T, Error>,
    Error: ParserError<Input>,
{
    separated(1.., repeat(1.., item_parser), line_ending).map(|rows| Grid::<T> { rows })
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for item in row {
                item.fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = PositionedItem<T>;
    type IntoIter = GridIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut rows_iterator = self.rows.into_iter();

        let Some(first_row) = rows_iterator.next() else {
            return GridIntoIterator {
                next: Default::default(),

                current_row_iter: Default::default(),
                remaining_rows: Default::default(),
            };
        };

        GridIntoIterator {
            next: Default::default(),
            current_row_iter: first_row.into_iter(),
            remaining_rows: rows_iterator,
        }
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = PositionedItem<&'a T>;
    type IntoIter = GridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct GridIterator<'a, T> {
    next: Position,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = PositionedItem<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.grid.get(self.next)?;

        let next_item = PositionedItem {
            position: self.next,
            item,
        };

        // since we got at least one item, it means our width is non-zero
        if self.next.x < self.grid.width() as isize - 1 {
            self.next.x += 1
        } else {
            self.next.x = 0;
            self.next.y += 1;
        }

        Some(next_item)
    }
}

pub struct GridIntoIterator<T> {
    next: Position,
    current_row_iter: IntoIter<T>,
    remaining_rows: IntoIter<Vec<T>>,
}

impl<T> Iterator for GridIntoIterator<T> {
    type Item = PositionedItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.current_row_iter.next() {
            let position = self.next;
            self.next.x += 1;
            return Some(PositionedItem { position, item });
        };

        // our current iterator is exhausted
        if let Some(next_row) = self.remaining_rows.next() {
            self.current_row_iter = next_row.into_iter();
            self.next.y += 1;
            self.next.x = 0;
            return self.next();
        }

        // we're done!
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::tests::DotOrPound::{Dot, Pound};
    use winnow::combinator::alt;
    use winnow::token::literal;
    use winnow::PResult;

    #[derive(Debug, Copy, Clone, PartialEq)]
    enum DotOrPound {
        Dot,
        Pound,
    }

    impl ParsableGridItem for DotOrPound {
        const PARSER: fn(&mut &str) -> PResult<Self> = dot_or_pound_parser;
    }

    fn dot_or_pound_parser(input: &mut &str) -> PResult<DotOrPound> {
        alt((literal('.').value(Dot), literal('#').value(Pound))).parse_next(input)
    }

    #[test]
    fn grid_parsing() {
        let dummy = r#"..#
.#.
###"#;
        let mut parser = grid_parser(dot_or_pound_parser);
        let res: Grid<DotOrPound> = parser.parse(dummy).unwrap();
        assert_eq!(
            res,
            Grid {
                rows: vec![
                    vec![Dot, Dot, Pound],
                    vec![Dot, Pound, Dot],
                    vec![Pound, Pound, Pound]
                ],
            }
        )
    }

    #[test]
    fn from_str_grid() {
        let dummy = r#"..#
.#.
###"#;

        let res: Grid<DotOrPound> = dummy.parse().unwrap();
        assert_eq!(
            res,
            Grid {
                rows: vec![
                    vec![Dot, Dot, Pound],
                    vec![Dot, Pound, Dot],
                    vec![Pound, Pound, Pound]
                ],
            }
        )
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Dummy(usize);

    fn dummy_grid() -> Grid<Dummy> {
        Grid {
            rows: vec![
                vec![Dummy(0), Dummy(1), Dummy(2)],
                vec![Dummy(3), Dummy(4), Dummy(5)],
                vec![Dummy(6), Dummy(7), Dummy(8)],
                vec![Dummy(9), Dummy(10), Dummy(11)],
            ],
        }
    }

    #[test]
    fn grid_iterator() {
        let grid = dummy_grid();
        for (i, item) in grid.iter().enumerate() {
            assert_eq!(&Dummy(i), item.item);
        }

        let grid = dummy_grid();
        let mut iter = grid.iter();
        assert_eq!(Position::new(0, 0), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 0), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 0), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 1), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 1), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 1), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 2), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 2), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 2), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 3), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 3), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 3), iter.next().unwrap().position);

        assert!(iter.next().is_none())
    }

    #[test]
    fn into_grid_iterator() {
        let grid = dummy_grid();
        for (i, item) in grid.into_iter().enumerate() {
            assert_eq!(Dummy(i), item.item);
        }

        let grid = dummy_grid();
        let mut iter = grid.into_iter();
        assert_eq!(Position::new(0, 0), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 0), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 0), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 1), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 1), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 1), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 2), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 2), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 2), iter.next().unwrap().position);

        assert_eq!(Position::new(0, 3), iter.next().unwrap().position);
        assert_eq!(Position::new(1, 3), iter.next().unwrap().position);
        assert_eq!(Position::new(2, 3), iter.next().unwrap().position);

        assert!(iter.next().is_none())
    }
}
