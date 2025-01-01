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

use anyhow::{Error, Result};
use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::str::FromStr;

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
/// value1<token>value2<token>...
///
/// to Vec<T>
pub fn parse_token_separated_values<T>(raw: &str, token: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    raw.trim()
        .split(token)
        .map(|split| split.parse())
        .collect::<Result<Vec<T>, _>>()
        .map_err(|err| {
            Error::msg(format!(
                "input could not be parsed into desired type: {err:?}"
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
    parse_token_separated_values(raw, ",")
}

/// Parses input in the form of:
///
/// value1 value2 ...
///
/// to Vec<T>
pub fn parse_space_separated_values<T>(raw: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_token_separated_values(raw, " ")
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
