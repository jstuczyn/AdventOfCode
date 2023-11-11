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

use humantime::format_duration;
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::time::{Duration, Instant};

pub mod parser;

extern crate aoc_derive;

pub use aoc_derive::Aoc;

pub trait AocSolution {
    type Input: Clone;
    type Error: Display;
    type Part1Output: Display;
    type Part2Output: Display;

    fn parse_input(_raw: &str) -> Result<Self::Input, Self::Error>;
    fn part1(_input: Self::Input) -> Result<Self::Part1Output, Self::Error>;
    fn part2(_input: Self::Input) -> Result<Self::Part2Output, Self::Error>;
}

pub trait AocSolutionSolver: AocSolution {
    fn try_solve(raw_input: &str) {
        match run::<Self>(raw_input) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("failed to solve aoc for '{}': {err}", type_name::<Self>()),
        }
    }

    fn try_solve_from_file<P>(path: P)
    where
        P: AsRef<Path>,
    {
        match run_from_file::<Self, _>(path) {
            Ok(result) => println!("{result}"),
            Err(err) => eprintln!("failed to solve aoc for '{}': {err}", type_name::<Self>()),
        }
    }
}

impl<T> AocSolutionSolver for T where T: AocSolution {}

pub struct DayResult<T: AocSolution + ?Sized> {
    parsing: Duration,
    part1: TimedResult<Result<T::Part1Output, T::Error>>,
    part2: TimedResult<Result<T::Part2Output, T::Error>>,
}

impl<T: AocSolution + ?Sized> Display for DayResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# TIMING #")?;
        writeln!(f, "PARSING:\t{}", format_duration(self.parsing))?;
        writeln!(f, "PART 1:\t\t{}", format_duration(self.part1.taken))?;
        writeln!(f, "PART 2:\t\t{}", format_duration(self.part2.taken))?;
        writeln!(f)?;
        writeln!(f, "# RESULTS #")?;
        let display_p1 = match &self.part1.value {
            Ok(res) => res.to_string(),
            Err(err) => format!("failed to solve: {err}"),
        };

        let display_p2 = match &self.part2.value {
            Ok(res) => res.to_string(),
            Err(err) => format!("failed to solve: {err}"),
        };

        writeln!(f, "PART 1:\t\t{display_p1}")?;
        writeln!(f, "PART 2:\t\t{display_p2}")
    }
}

struct TimedResult<T> {
    taken: Duration,
    value: T,
}

impl<T, E> TimedResult<Result<T, E>> {
    pub fn transpose(self) -> Result<TimedResult<T>, E> {
        match self.value {
            Ok(x) => Ok(TimedResult {
                taken: self.taken,
                value: x,
            }),
            Err(err) => Err(err),
        }
    }
}

fn timed<T, U, F: FnOnce(T) -> U>(f: F, input: T) -> TimedResult<U> {
    let start = Instant::now();
    let output = f(input);
    let taken = start.elapsed();
    TimedResult {
        taken,
        value: output,
    }
}

pub fn run_from_file<T, P>(path: P) -> Result<DayResult<T>, T::Error>
where
    P: AsRef<Path>,
    T: AocSolution + ?Sized,
{
    let read_input = std::fs::read_to_string(path).expect("failed to read the file input");
    run(&read_input)
}

pub fn run<T>(input: &str) -> Result<DayResult<T>, T::Error>
where
    T: AocSolution + ?Sized,
{
    let parsed_input = timed(T::parse_input, input).transpose()?;

    let part1 = timed(T::part1, parsed_input.value.clone());
    let part2 = timed(T::part2, parsed_input.value);

    Ok(DayResult {
        parsing: parsed_input.taken,
        part1,
        part2,
    })
}
