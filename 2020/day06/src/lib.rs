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
use std::collections::HashMap;

#[derive(Aoc)]
pub struct Day06;

struct Group {
    size: usize,
    answers: HashMap<char, usize>,
}

impl<'a> From<&'a String> for Group {
    fn from(answers_raw: &'a String) -> Self {
        let mut answers = HashMap::new();

        let group_answers: Vec<_> = answers_raw.split_ascii_whitespace().collect();
        let size = group_answers.len();

        group_answers
            .into_iter()
            .flat_map(|answer| answer.chars())
            .for_each(|char| *answers.entry(char).or_insert(0) += 1);

        Group { size, answers }
    }
}

impl Group {
    fn len(&self) -> usize {
        self.answers.len()
    }

    fn all_yes(&self) -> usize {
        self.answers
            .iter()
            .filter(|&(_, count)| *count == self.size)
            .count()
    }
}

pub fn part1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|group_answers| Group::from(group_answers).len())
        .sum()
}

pub fn part2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|group_answers| Group::from(group_answers).all_yes())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "abc".to_string(),
            "a
b
c"
            .to_string(),
            "ab
ac"
            .to_string(),
            "a
a
a
a"
            .to_string(),
            "b".to_string(),
        ];

        let expected = 11;

        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "abc".to_string(),
            "a
b
c"
            .to_string(),
            "ab
ac"
            .to_string(),
            "a
a
a
a"
            .to_string(),
            "b".to_string(),
        ];

        let expected = 6;

        assert_eq!(expected, part2(input))
    }
}
