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

fn main() {
    let args = Args::parse();

    // TODO: this match is going to get incredibly nasty with more years
    // some macro could help here.
    match args.year {
        y if y == 2022 => match args.day {
            d if d == 1 => {
                day01_2022::solve("2022/day01/input");
            }
            d if d == 2 => {
                day02_2022::solve("2022/day02/input");
            }
            d if d > 25 || d == 0 => {
                println!("no puzzles exist for day {d}!")
            }
            d => println!("no solutions found for {y} day {d} : ("),
        },
        y if y == 2021 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2020 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2019 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2018 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2017 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2016 => {
            println!("no solutions found for {y} : (");
        }
        y if y == 2015 => {
            println!("no solutions found for {y} : (");
        }
        y => {
            println!("Advent of code hasn't published any puzzles for {y}! (alternatively I haven't updated my code yet)")
        }
    }
}
