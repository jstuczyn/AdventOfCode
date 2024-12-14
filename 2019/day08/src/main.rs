// Copyright 2019-2024 Jedrzej Stuczynski
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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use day08_2019::{do_part1, do_part2, Image};
use std::fs;

fn read_input_file(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn main() {
    let mut image = Image::new(6, 25);

    let input = read_input_file("inputs/2019/day08");
    let layer_chunks = input.chunks(image.area());

    for layer_data in layer_chunks {
        image.add_layer(layer_data.to_vec());
    }

    do_part1(image.clone());
    do_part2(image);
}
