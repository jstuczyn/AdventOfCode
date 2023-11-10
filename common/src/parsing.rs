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

use anyhow::{Error, Result};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
use std::str::FromStr;

// we need separate trait, i.e. we can't just use `FromStr`,
// because of all the custom rules for say `Vec<T>`
// TODO: or maybe we should just create wrapper containers instead?
pub trait AocInputParser {
    type Output;

    fn parse_input(raw: &str) -> Result<Self::Output>;
}

pub trait AocParseExt {
    fn parse_aoc_input<F: AocInputParser>(&self) -> Result<F::Output>;
}

impl AocParseExt for str {
    fn parse_aoc_input<F: AocInputParser>(&self) -> Result<F::Output> {
        <F as AocInputParser>::parse_input(self)
    }
}

pub struct GroupsParser<T>(*const PhantomData<T>);

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

/// Parse input in the form of x=<a>..<b> to `RangeInclusive<isize>`
pub fn parse_raw_range(raw: &str) -> Result<RangeInclusive<isize>> {
    let mut bounds = raw.split('=');
    let _axis = bounds
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?;
    let mut values = bounds
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .split("..");

    let lower_bound = values
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .parse()?;
    let upper_bound = values
        .next()
        .ok_or_else(|| Error::msg("incomplete range"))?
        .parse()?;

    Ok(RangeInclusive::new(lower_bound, upper_bound))
}

/// Parses input in the form of:
///
/// value1
/// value2
/// ...
///
/// to Vec<T>
pub fn parse_input_lines<T>(raw: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    raw.lines()
        .map(|line| line.parse::<T>())
        .collect::<std::result::Result<Vec<T>, _>>()
        .map_err(|err| {
            Error::msg(format!(
                "input could not be parsed into desired type - {err:?}"
            ))
        })
}

/// Parses input in the form of:
///
/// value1,value2,...
///
/// to Vec<T>
pub fn parse_comma_separated_values<T>(raw: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    raw.split(',')
        .map(|split| split.parse())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| {
            Error::msg(format!(
                "input could not be parsed into desired type - {err:?}"
            ))
        })
}

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
pub fn split_to_string_groups(raw: &str) -> Vec<String> {
    let split = raw
        .replace("\r\n", "\n") // Windows fix
        .split("\n\n")
        .map(|split| split.to_owned())
        .collect();

    split
}

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
pub fn parse_groups<T>(raw: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    split_to_string_groups(raw)
        .into_iter()
        .map(|line| line.parse::<T>())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| {
            Error::msg(format!(
                "input could not be parsed into desired type - {err:?}"
            ))
        })
}

/// Transforms the raw string input into a Vec<char>
pub fn as_char_vec(raw: &str) -> Result<Vec<char>> {
    Ok(raw.chars().collect())
}
