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

use anyhow::bail;
use std::any::type_name;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::time::{Duration, Instant};

pub trait AocSolution {
    type Input: Clone;
    type Part1Output: Display;
    type Part2Output: Display;

    fn parse_input<M: AsRef<str>>(_raw: M) -> Result<Self::Input, anyhow::Error> {
        bail!("unimplemented")
    }

    fn part1(_input: Self::Input) -> Result<Self::Part1Output, anyhow::Error> {
        bail!("unimplemented")
    }

    fn part2(_input: Self::Input) -> Result<Self::Part2Output, anyhow::Error> {
        bail!("unimplemented")
    }
}

pub trait AocSolutionSolver: AocSolution {
    fn try_solve<M: AsRef<str>>(raw_input: M) {
        match run::<Self, _>(raw_input) {
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
    part1: TimedResult<T::Part1Output>,
    part2: TimedResult<T::Part2Output>,
}

impl<T: AocSolution + ?Sized> Display for DayResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "# TIMING #")?;
        writeln!(f, "PARSING:\t{:?}", self.parsing)?;
        writeln!(f, "PART 1:\t\t{:?}", self.part1.taken)?;
        writeln!(f, "PART 2:\t\t{:?}", self.part2.taken)?;
        writeln!(f)?;
        writeln!(f, "# RESULTS #")?;
        writeln!(f, "PART 1:\t\t{}", self.part1.value)?;
        writeln!(f, "PART 2:\t\t{}", self.part2.value)
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

pub fn run_from_file<T, P>(path: P) -> Result<DayResult<T>, anyhow::Error>
where
    P: AsRef<Path>,
    T: AocSolution + ?Sized,
{
    run(std::fs::read_to_string(path)?)
}

pub fn run<T, M>(input: M) -> Result<DayResult<T>, anyhow::Error>
where
    T: AocSolution + ?Sized,
    M: AsRef<str>,
{
    let parsed_input = timed(T::parse_input, input).transpose()?;

    let part1 = timed(T::part1, parsed_input.value.clone()).transpose()?;
    let part2 = timed(T::part2, parsed_input.value).transpose()?;

    Ok(DayResult {
        parsing: parsed_input.taken,
        part1,
        part2,
    })
}
