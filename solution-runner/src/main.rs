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
    define_solution!(args, 2020, 1, "inputs/2019/day01", day01_2019::Day01);
    define_solution!(args, 2019, 2, "inputs/2019/day02", day02_2019::Day02);
    define_solution!(args, 2019, 3, "inputs/2019/day03", day03_2019::Day03);
    define_solution!(args, 2019, 4, "inputs/2019/day04", day04_2019::Day04);
    define_solution!(args, 2019, 5, "inputs/2019/day05", day05_2019::Day05);
    define_solution!(args, 2019, 6, "inputs/2019/day06", day06_2019::Day06);
    define_solution!(args, 2019, 7, "inputs/2019/day07", day07_2019::Day07);
    define_solution!(args, 2019, 8, "inputs/2019/day08", day08_2019::Day08);
    define_solution!(args, 2019, 9, "inputs/2019/day09", day09_2019::Day09);
    define_solution!(args, 2020, 1, "inputs/2020/day01", day01_2020::Day01);
    define_solution!(args, 2020, 2, "inputs/2020/day02", day02_2020::Day02);
    define_solution!(args, 2020, 3, "inputs/2020/day03", day03_2020::Day03);
    define_solution!(args, 2020, 4, "inputs/2020/day04", day04_2020::Day04);
    define_solution!(args, 2020, 5, "inputs/2020/day05", day05_2020::Day05);
    define_solution!(args, 2020, 6, "inputs/2020/day06", day06_2020::Day06);
    define_solution!(args, 2020, 7, "inputs/2020/day07", day07_2020::Day07);
    define_solution!(args, 2020, 8, "inputs/2020/day08", day08_2020::Day08);
    define_solution!(args, 2020, 9, "inputs/2020/day09", day09_2020::Day09);
    define_solution!(args, 2020, 10, "inputs/2020/day10", day10_2020::Day10);
    define_solution!(args, 2020, 11, "inputs/2020/day11", day11_2020::Day11);
    define_solution!(args, 2020, 12, "inputs/2020/day12", day12_2020::Day12);
    define_solution!(args, 2020, 13, "inputs/2020/day13", day13_2020::Day13);
    define_solution!(args, 2020, 14, "inputs/2020/day14", day14_2020::Day14);
    define_solution!(args, 2020, 15, "inputs/2020/day15", day15_2020::Day15);
    define_solution!(args, 2020, 16, "inputs/2020/day16", day16_2020::Day16);
    define_solution!(args, 2020, 17, "inputs/2020/day17", day17_2020::Day17);
    define_solution!(args, 2020, 18, "inputs/2020/day18", day18_2020::Day18);
    define_solution!(args, 2020, 19, "inputs/2020/day19", day19_2020::Day19);
    define_solution!(args, 2020, 20, "inputs/2020/day20", day20_2020::Day20);
    define_solution!(args, 2020, 21, "inputs/2020/day21", day21_2020::Day21);
    define_solution!(args, 2020, 22, "inputs/2020/day22", day22_2020::Day22);
    define_solution!(args, 2020, 23, "inputs/2020/day23", day23_2020::Day23);
    define_solution!(args, 2020, 24, "inputs/2020/day24", day24_2020::Day24);
    define_solution!(args, 2020, 25, "inputs/2020/day25", day25_2020::Day25);
    define_solution!(args, 2021, 1, "inputs/2021/day01", day01_2021::Day01);
    define_solution!(args, 2021, 2, "inputs/2021/day02", day02_2021::Day02);
    define_solution!(args, 2021, 3, "inputs/2021/day03", day03_2021::Day03);
    define_solution!(args, 2021, 4, "inputs/2021/day04", day04_2021::Day04);
    define_solution!(args, 2021, 5, "inputs/2021/day05", day05_2021::Day05);
    define_solution!(args, 2021, 6, "inputs/2021/day06", day06_2021::Day06);
    define_solution!(args, 2021, 7, "inputs/2021/day07", day07_2021::Day07);
    define_solution!(args, 2021, 8, "inputs/2021/day08", day08_2021::Day08);
    define_solution!(args, 2021, 9, "inputs/2021/day09", day09_2021::Day09);
    define_solution!(args, 2021, 10, "inputs/2021/day10", day10_2021::Day10);
    define_solution!(args, 2021, 11, "inputs/2021/day11", day11_2021::Day11);
    define_solution!(args, 2021, 12, "inputs/2021/day12", day12_2021::Day12);
    define_solution!(args, 2021, 13, "inputs/2021/day13", day13_2021::Day13);
    define_solution!(args, 2021, 14, "inputs/2021/day14", day14_2021::Day14);
    define_solution!(args, 2021, 15, "inputs/2021/day15", day15_2021::Day15);
    define_solution!(args, 2021, 16, "inputs/2021/day16", day16_2021::Day16);
    define_solution!(args, 2021, 17, "inputs/2021/day17", day17_2021::Day17);
    define_solution!(args, 2021, 18, "inputs/2021/day18", day18_2021::Day18);
    define_solution!(args, 2021, 19, "inputs/2021/day19", day19_2021::Day19);
    define_solution!(args, 2021, 20, "inputs/2021/day20", day20_2021::Day20);
    define_solution!(args, 2021, 21, "inputs/2021/day21", day21_2021::Day21);
    define_solution!(args, 2021, 22, "inputs/2021/day22", day22_2021::Day22);
    define_solution!(args, 2021, 24, "inputs/2021/day24", day24_2021::Day24);
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
    define_solution!(args, 2024, 3, "inputs/2024/day03", day03_2024::Day03);
    define_solution!(args, 2024, 4, "inputs/2024/day04", day04_2024::Day04);
    define_solution!(args, 2024, 5, "inputs/2024/day05", day05_2024::Day05);
    define_solution!(args, 2024, 6, "inputs/2024/day06", day06_2024::Day06);
    define_solution!(args, 2024, 7, "inputs/2024/day07", day07_2024::Day07);
    // AUTOGENERATED SOLUTIONS END

    println!("no solution found for year {}, day {}", args.year, args.day);
}