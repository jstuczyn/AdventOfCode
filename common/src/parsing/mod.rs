// Copyright 2022-2023 Jedrzej Stuczynski
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

use anyhow::Result;
use aoc_solution::parser::AocInputParser;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub use impls::*;
pub mod combinators;
pub mod impls;

/// Uses the underlying FromStr impl of T
pub struct FromStrParser<T: FromStr>(PhantomData<T>);

/// Parse input in the form of x=<a>..<b> to `RangeInclusive<isize>`
pub struct AssignedRangeParser;

/// Parses input in the form of:
///
/// value1
/// value2
/// ...
///
/// to Vec<T>
pub struct LineParser<T>(PhantomData<T>);

/// Parses input in the form of:
///
/// value1,value2,...
///
/// to Vec<T>
pub struct CommaSeparatedParser<T>(PhantomData<T>);

/// Parses input in the form of:
///
/// value1 value2 ...
///
/// to Vec<T>
pub struct SpaceSeparatedParser<T>(PhantomData<T>);

/// Splits input in the form of:
///
/// group1_value1
/// group1_value2
/// ...
///
/// group2_value1
/// group2_value2
/// ...
///
/// to Vec<String>
pub struct StringGroupsParser;

/// Parses input in the form of:
///
/// group1_value1
/// group1_value2
/// ...
///
/// group2_value1
/// group2_value2
/// ...
///
/// to Vec<T>
pub struct GroupsParser<T>(PhantomData<T>);

/// Transforms the raw string input into a Vec<char>
pub struct CharVecParser;

impl<T> AocInputParser for FromStrParser<T>
where
    T: FromStr,
    anyhow::Error: From<<T as FromStr>::Err>,
{
    type Output = T;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        raw.parse().map_err(anyhow::Error::from)
    }
}

impl AocInputParser for AssignedRangeParser {
    type Output = RangeInclusive<isize>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        parse_assigned_range(raw)
    }
}

impl<T> AocInputParser for LineParser<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Output = Vec<T>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        parse_input_lines(raw)
    }
}

impl<T> AocInputParser for CommaSeparatedParser<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Output = Vec<T>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        parse_comma_separated_values(raw)
    }
}

impl<T> AocInputParser for SpaceSeparatedParser<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Output = Vec<T>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        parse_space_separated_values(raw)
    }
}

impl AocInputParser for StringGroupsParser {
    type Output = Vec<String>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        Ok(split_to_string_groups(raw))
    }
}

impl<T> AocInputParser for GroupsParser<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    type Output = Vec<T>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        parse_groups(raw)
    }
}

impl AocInputParser for CharVecParser {
    type Output = Vec<char>;

    fn parse_input(raw: &str) -> Result<Self::Output> {
        as_char_vec(raw)
    }
}
