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

use aoc_common::helpers::root_path;
use aoc_solution::AocSolutionSolver;
use clap::Parser;

/// Simple solution runner for Advent of Code puzzles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specifies the event year
    #[arg(short, long, required = true)]
    year: u16,

    /// Specifies the event day
    #[arg(short, long, required = true)]
    day: u8,
    // not implemented yet
    // #[arg(long, conflicts_with = "custom_input")]
    // custom_input_filepath: Option<PathBuf>,

    // not implemented yet
    // #[arg(long, conflicts_with = "custom_input_filepath")]
    // custom_input: Option<String>,
}

macro_rules! define_solution {
    ($args: ident, $year: literal, $day: literal, $input: literal, $solver: ty) => {
        if $args.year == $year && $args.day == $day {
            return <$solver as AocSolutionSolver>::try_solve_from_file(root_path($input));
        }
    };
}

fn main() {
    let args = Args::parse();

    // AUTOGENERATED SOLUTIONS START
    define_solution!(args, 2022, 1, "inputs/2022/day01", day01_2022::Day01);
    define_solution!(args, 2022, 2, "inputs/2022/day02", day02_2022::Day02);
    define_solution!(args, 2022, 3, "inputs/2022/day03", day03_2022::Day03);
    define_solution!(args, 2022, 4, "inputs/2022/day04", day04_2022::Day04);
    define_solution!(args, 2022, 5, "inputs/2022/day05", day05_2022::Day05);
    define_solution!(args, 2022, 6, "inputs/2022/day06", day06_2022::Day06);
    define_solution!(args, 2022, 7, "inputs/2022/day07", day07_2022::Day07);
    define_solution!(args, 2022, 8, "inputs/2022/day08", day08_2022::Day08);
    define_solution!(args, 2022, 11, "inputs/2022/day11", day11_2022::Day11);
    define_solution!(args, 2022, 10, "inputs/2022/day10", day10_2022::Day10);
    define_solution!(args, 2023, 1, "inputs/2023/day01", day01_2023::Day01);
    define_solution!(args, 2023, 2, "inputs/2023/day02", day02_2023::Day02);
    define_solution!(args, 2023, 3, "inputs/2023/day03", day03_2023::Day03);
    define_solution!(args, 2023, 4, "inputs/2023/day04", day04_2023::Day04);
    define_solution!(args, 2023, 5, "inputs/2023/day05", day05_2023::Day05);
    define_solution!(args, 2024, 1, "inputs/2024/day01", day01_2024::Day01);
    define_solution!(args, 2024, 2, "inputs/2024/day02", day02_2024::Day02);
    // AUTOGENERATED SOLUTIONS END

    println!("no solution found for year {}, day {}", args.year, args.day);
}
