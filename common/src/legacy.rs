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

pub use execution::*;
pub use input_read::*;
pub use parsing::*;

pub mod execution {

    use std::fmt::Display;
    use std::io;
    use std::path::Path;
    use std::time::{Duration, Instant};

    pub fn execute_slice_with_timing<F, T, U>(func: F, args: &[T]) -> (U, Duration)
    where
        F: Fn(&[T]) -> U,
    {
        let start = Instant::now();
        let res = func(args);
        let time_taken = start.elapsed();
        (res, time_taken)
    }

    pub fn execute_vec_with_timing<F, T, U>(func: F, args: Vec<T>) -> (U, Duration)
    where
        F: Fn(Vec<T>) -> U,
    {
        let start = Instant::now();
        let res = func(args);
        let time_taken = start.elapsed();
        (res, time_taken)
    }

    pub fn execute_struct_with_timing<F, T, U>(func: F, args: T) -> (U, Duration)
    where
        F: Fn(T) -> U,
    {
        let start = Instant::now();
        let res = func(args);
        let time_taken = start.elapsed();
        (res, time_taken)
    }

    pub fn execute_vec<P, T, F, G, H, U, S>(
        input_file: P,
        input_parser: F,
        part1_fn: G,
        part2_fn: H,
    ) where
        P: AsRef<Path>,
        F: Fn(P) -> io::Result<Vec<T>>,
        G: Fn(Vec<T>) -> U,
        H: Fn(Vec<T>) -> S,
        U: Display,
        S: Display,
        T: Clone,
    {
        let parsing_start = Instant::now();
        let input = input_parser(input_file).expect("failed to read input file");
        let parsing_time_taken = parsing_start.elapsed();

        let (part1_result, part1_time_taken) = execute_vec_with_timing(part1_fn, input.clone());
        let (part2_result, part2_time_taken) = execute_vec_with_timing(part2_fn, input);

        println!("It took {parsing_time_taken:?} to parse the input");
        println!();
        println!(
            "Part 1 result is {}\nIt took {:?} to compute",
            part1_result, part1_time_taken
        );
        println!();
        println!(
            "Part 2 result is {}\nIt took {:?} to compute",
            part2_result, part2_time_taken
        );
    }

    // We'll see how it evolves with variety of inputs we get
    pub fn execute_slice<P, T, F, G, H, U, S>(
        input_file: P,
        input_parser: F,
        part1_fn: G,
        part2_fn: H,
    ) where
        P: AsRef<Path>,
        F: Fn(P) -> io::Result<Vec<T>>,
        G: Fn(&[T]) -> U,
        H: Fn(&[T]) -> S,
        U: Display,
        S: Display,
    {
        let parsing_start = Instant::now();
        let input = input_parser(input_file).expect("failed to read input file");
        let parsing_time_taken = parsing_start.elapsed();

        let (part1_result, part1_time_taken) = execute_slice_with_timing(part1_fn, &input);
        let (part2_result, part2_time_taken) = execute_slice_with_timing(part2_fn, &input);

        println!("It took {parsing_time_taken:?} to parse the input");
        println!();
        println!(
            "Part 1 result is {}\nIt took {:?} to compute",
            part1_result, part1_time_taken
        );
        println!();
        println!(
            "Part 2 result is {}\nIt took {:?} to compute",
            part2_result, part2_time_taken
        );
    }

    pub fn execute_struct<P, T, F, G, H, U, S>(
        input_file: P,
        input_parser: F,
        part1_fn: G,
        part2_fn: H,
    ) where
        P: AsRef<Path>,
        F: Fn(P) -> io::Result<T>,
        G: Fn(T) -> U,
        H: Fn(T) -> S,
        U: Display,
        S: Display,
        T: Clone,
    {
        let parsing_start = Instant::now();
        let input = input_parser(input_file).expect("failed to read input file");
        let parsing_time_taken = parsing_start.elapsed();

        let (part1_result, part1_time_taken) = execute_struct_with_timing(part1_fn, input.clone());
        let (part2_result, part2_time_taken) = execute_struct_with_timing(part2_fn, input);

        println!("It took {parsing_time_taken:?} to parse the input");
        println!();
        println!(
            "Part 1 result is {}\nIt took {:?} to compute",
            part1_result, part1_time_taken
        );
        println!();
        println!(
            "Part 2 result is {}\nIt took {:?} to compute",
            part2_result, part2_time_taken
        );
    }
}

pub mod input_read {

    use std::fmt::Debug;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::str::FromStr;

    pub fn read_input_lines<P>(path: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        io::BufReader::new(file).lines().collect()
    }

    pub fn read_input_lines_with_parser<T, F, P>(path: P, parser: F) -> io::Result<Vec<T>>
    where
        P: AsRef<Path>,
        F: Fn(String) -> io::Result<T>,
    {
        read_input_lines(path)?
            .into_iter()
            .map(parser)
            .collect::<Result<Vec<T>, _>>()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    /// Reads the file as lines, parsing each of them into desired type.
    pub fn read_parsed_line_input<T, P>(path: P) -> io::Result<Vec<T>>
    where
        P: AsRef<Path>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        read_input_lines(path)?
            .into_iter()
            .map(|line| line.parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("input could not be parsed into desired type - {err:?}"),
                )
            })
    }

    /// Reads the file and outputs String groups that were originally separated by an empty line
    pub fn read_into_string_groups<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
        fs::read_to_string(path).map(|string| {
            string
                .replace("\r\n", "\n") // Windows fix
                .split("\n\n")
                .map(|split| split.to_owned())
                .collect()
        })
    }

    pub fn read_parsed_groups<T, P>(path: P) -> io::Result<Vec<T>>
    where
        P: AsRef<Path>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        read_into_string_groups(path)?
            .into_iter()
            .map(|line| line.parse::<T>())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("input could not be parsed into desired type - {err:?}"),
                )
            })
    }

    /// Reads the file as a string and parses comma-separated types
    pub fn read_parsed_comma_separated_values<T, P>(path: P) -> io::Result<Vec<T>>
    where
        P: AsRef<Path>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        fs::read_to_string(path)?
            .split(',')
            .map(|split| split.parse())
            .collect::<Result<Vec<T>, _>>()
            .map_err(|err| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("input could not be parsed into desired type - {err:?}"),
                )
            })
    }

    pub fn read_parsed<T, P>(path: P) -> io::Result<T>
    where
        P: AsRef<Path>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        fs::read_to_string(path).map(|s| s.parse())?.map_err(|err| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("input could not be parsed into desired type - {err:?}"),
            )
        })
    }
}

pub mod parsing {
    use anyhow::{Error, Result};
    use std::ops::RangeInclusive;

    // parses something in the form of x=<a>..<b>
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
}
