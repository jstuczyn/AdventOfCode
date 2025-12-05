# AdventOfCode

Keeping track of solutions to various puzzles from https://adventofcode.com/

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=for-the-badge&logo=appveyo)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://img.shields.io/github/actions/workflow/status/jstuczyn/AdventOfCode/build.yml?style=for-the-badge)](https://github.com/jstuczyn/AdventOfCode/actions?query=branch%3Amaster)
[![Coverage](https://img.shields.io/codecov/c/github/jstuczyn/AdventOfCode2021?token=MB5EB16E2Y&style=for-the-badge&logo=codecov?token=EEAVX8J62K)](https://codecov.io/gh/jstuczyn/AdventOfCode2021)

## About

> Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that
> can be solved in any programming language you like. People use them as a speed contest, interview prep, company
> training, university coursework, practice problems, or to challenge each other.
>
> You don't need a computer science background to participate - just a little programming knowledge and some problem
> solving skills will get you pretty far. Nor do you need a fancy computer; every problem has a solution that completes
> in
> at most 15 seconds on ten-year-old hardware.
>
> - www.adventofcode.com

## Latest puzzle

[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2025.json)](https://adventofcode.com/2025/about)

## Previous years

[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2024.json)](https://adventofcode.com/2024/about)
[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2023.json)](https://adventofcode.com/2023/about)
[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2022.json)](https://adventofcode.com/2022/about)
[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2021.json)](https://adventofcode.com/2021/about)
[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2020.json)](https://adventofcode.com/2020/about)
[![Completion Status](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/jstuczyn/AdventOfCode/master/.github/badges/completion2019.json)](https://adventofcode.com/2019/about)

## Running the code

There are two ways of running particular solution. One can either go to the directory associated with the given day, for
example:

```shell
cd 2022/day01
```

and run it from there:

```shell
cargo run --release
```

Alternatively, there's a dedicated `solution-runner` binary that's can run any sub-solution based on arguments provided.
For example

```shell
./solution-runner --year 2022 --day 1
```

### Note:

solutions from 2019, 2020 and 2021 are not guaranteed to run correctly,
as they got imported from old repositories and have not been written with the current runner framework in mind.

## Adding new day

Run the following command to generate the template:

```shell
cargo run -p aoc-init -- --year $YEAR --day $DAY
```

[//]: # (It further has optional flags `custom-input-filepath` and `custom-input` for providing non-default inputs.)
