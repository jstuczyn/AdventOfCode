// Copyright 2024 Jedrzej Stuczynski
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

use aoc_solution::Aoc;

#[derive(Aoc)]
pub struct Day13;

struct Bus {
    id: usize,
}

impl Bus {
    fn new(raw_id: &str) -> Option<Self> {
        match raw_id.parse() {
            Ok(id) => Some(Bus { id }),
            _ => None,
        }
    }

    // used for part1
    fn earliest_departure_from(&self, timestamp: usize) -> usize {
        // assume id < timestamp
        let quo = timestamp / self.id;
        let rem = timestamp % self.id;

        let mut n = quo;
        if rem != 0 {
            n += 1;
        }

        self.id * n
    }
}

fn split_into_timestamp_and_buses(input: &str) -> (usize, Vec<Option<Bus>>) {
    let split: Vec<_> = input.split_ascii_whitespace().collect();
    assert_eq!(2, split.len(), "invalid input");

    let timestamp = split[0].parse().expect("failed to parse timestamp");
    let buses = split[1].split(',').map(Bus::new).collect();

    (timestamp, buses)
}

pub fn part1(input: &str) -> usize {
    let (timestamp, buses) = split_into_timestamp_and_buses(input);
    let (id, departure) = buses
        .into_iter()
        .flatten()
        .map(|bus| (bus.id, bus.earliest_departure_from(timestamp)))
        .min_by(|(_, timestamp1), (_, timestamp2)| timestamp1.cmp(timestamp2))
        .unwrap();
    id * (departure - timestamp)
}

#[allow(clippy::many_single_char_names)]
// code was originally adapted from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// code was originally adapted from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn mod_inv(x: isize, n: isize) -> Option<isize> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// code was originally adapted from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn crt(residues: &[isize], modulii: &[isize]) -> Option<isize> {
    let prod = modulii.iter().product::<isize>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part2(input: &str) -> usize {
    let (_, buses) = split_into_timestamp_and_buses(input);
    let (modulii, residues): (Vec<_>, Vec<_>) = buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, bus)| {
            bus.map(|bus| {
                (
                    bus.id as isize,
                    (bus.id as isize - i as isize) % bus.id as isize,
                )
            })
        })
        .unzip();

    crt(&residues, &modulii).expect("failed to apply CRT") as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = r#"939
7,13,x,x,59,x,31,19"#;

        let expected = 295;

        assert_eq!(expected, part1(input));
    }

    #[test]
    fn part2_sample_input1() {
        let input = r#"939
    7,13,x,x,59,x,31,19"#;

        let expected = 1068781;

        assert_eq!(expected, part2(input));
    }

    #[test]
    fn part2_sample_input2() {
        let input = r#"42
    17,x,13,19"#;

        let expected = 3417;

        assert_eq!(expected, part2(input));
    }

    #[test]
    fn part2_sample_input3() {
        let input = r#"42
67,7,59,61"#;

        let expected = 754018;

        assert_eq!(expected, part2(input));
    }

    #[test]
    fn part2_sample_input4() {
        let input = r#"42
    67,x,7,59,61"#;

        let expected = 779210;

        assert_eq!(expected, part2(input));
    }

    #[test]
    fn part2_sample_input5() {
        let input = r#"42
    67,7,x,59,61"#;

        let expected = 1261476;

        assert_eq!(expected, part2(input));
    }

    #[test]
    fn part2_sample_input6() {
        let input = r#"42
    1789,37,47,1889"#;

        let expected = 1202161486;

        assert_eq!(expected, part2(input));
    }
}
