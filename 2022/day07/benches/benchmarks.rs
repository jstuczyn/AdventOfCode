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

use common::input_read::read_input;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day07_2022::solution::{part1, part2};
use day07_2022::types::FileSystem;
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn get_input() -> FileSystem {
    read_input("input", FromStr::from_str).unwrap()
}

fn input_parse_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input").unwrap();
    c.bench_function("input parse", move |b| {
        b.iter(|| FileSystem::from_str(&input).unwrap())
    });
}

fn part1_benchmark(c: &mut Criterion) {
    let input = get_input();
    c.bench_function("part1", move |b| {
        b.iter_batched(
            || input.clone(),
            |input| part1(black_box(input)),
            BatchSize::SmallInput,
        )
    });
}

fn part2_benchmark(c: &mut Criterion) {
    let input = get_input();
    c.bench_function("part2", move |b| {
        b.iter_batched(
            || input.clone(),
            |input| part2(black_box(input)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    input_parse_benchmark,
    part1_benchmark,
    part2_benchmark
);
criterion_main!(benches);
