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

#[macro_export]
macro_rules! define_aoc_benchmark {
    ($input: literal, $typ: ty) => {
        use ::aoc_solution::AocSolution;

        use aoc_common::helpers::root_path;
        use aoc_common::input_read::read_input;
        use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
        use std::fs;

        fn get_input() -> <$typ as AocSolution>::Input {
            read_input(root_path($input), <$typ as AocSolution>::parse_input).unwrap()
        }

        fn input_parse_benchmark(c: &mut Criterion) {
            let input = fs::read_to_string(root_path($input)).unwrap();
            let bench_name = format!("{}_input_parse", env!("CARGO_PKG_NAME"));
            c.bench_function(&bench_name, move |b| {
                b.iter(|| {
                    let _ = <$typ as AocSolution>::parse_input(&input).unwrap();
                })
            });
        }

        fn part1_benchmark(c: &mut Criterion) {
            let input = get_input();
            let bench_name = format!("{}_part1", env!("CARGO_PKG_NAME"));
            c.bench_function(&bench_name, move |b| {
                b.iter_batched(
                    || input.clone(),
                    |input| <$typ as AocSolution>::part1((black_box(input))),
                    BatchSize::SmallInput,
                )
            });
        }

        fn part2_benchmark(c: &mut Criterion) {
            let input = get_input();
            let bench_name = format!("{}_part2", env!("CARGO_PKG_NAME"));
            c.bench_function(&bench_name, move |b| {
                b.iter_batched(
                    || input.clone(),
                    |input| <$typ as AocSolution>::part2((black_box(input))),
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
    };
}
