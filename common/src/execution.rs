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

use crate::input_read::read_input;
use std::fmt::Display;
use std::path::Path;
use std::time::{Duration, Instant};

pub fn execute_with_timing<F, T, U>(func: F, args: T) -> (U, Duration)
where
    F: Fn(T) -> U,
{
    let start = Instant::now();
    let res = func(args);
    let time_taken = start.elapsed();
    (res, time_taken)
}

pub fn execute<P, T, F, G, H, U, S>(input_file: P, input_parser: F, part1_fn: G, part2_fn: H)
where
    P: AsRef<Path>,
    F: Fn(&str) -> anyhow::Result<T>,
    T: Clone,
    G: Fn(T) -> U,
    H: Fn(T) -> S,
    U: Display,
    S: Display,
{
    let parsing_start = Instant::now();
    let input = read_input(input_file, input_parser).expect("failed to read input file");
    let parsing_time_taken = parsing_start.elapsed();

    let (part1_result, part1_time_taken) = execute_with_timing(part1_fn, input.clone());
    let (part2_result, part2_time_taken) = execute_with_timing(part2_fn, input);

    println!("It took {parsing_time_taken:?} to parse the input");
    println!();
    println!("Part 1 result is {part1_result}\nIt took {part1_time_taken:?} to compute",);
    println!();
    println!("Part 2 result is {part2_result}\nIt took {part2_time_taken:?} to compute",);
}
