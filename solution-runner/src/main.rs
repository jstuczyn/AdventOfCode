// Copyright 2022 Jedrzej Stuczynski
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
    ($args: ident, $year: literal, $day: literal, $input: literal, $solver: expr) => {
        if $args.year == $year && $args.day == $day {
            return $solver($input);
        }
    };
}

fn main() {
    let args = Args::parse();

    define_solution!(args, 2022, 1, "2022/day01/input", day01_2022::solve);
    define_solution!(args, 2022, 2, "2022/day02/input", day02_2022::solve);
    define_solution!(args, 2022, 3, "2022/day03/input", day03_2022::solve);
    define_solution!(args, 2022, 4, "2022/day04/input", day04_2022::solve);
    define_solution!(args, 2022, 5, "2022/day05/input", day05_2022::solve);
    define_solution!(args, 2022, 6, "2022/day06/input", day06_2022::solve);
    define_solution!(args, 2022, 7, "2022/day07/input", day07_2022::solve);
    define_solution!(args, 2022, 8, "2022/day08/input", day08_2022::solve);
    define_solution!(args, 2022, 11, "2022/day11/input", day11_2022::solve);

    println!("no solution found for year {}, day {}", args.year, args.day);
}
