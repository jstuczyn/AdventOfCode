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

use anyhow::{bail, Context};
use aoc_common::parsing::LineParser;
use aoc_solution::Aoc;
use std::str::FromStr;

#[derive(Aoc)]
#[aoc(input = Vec<Command>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = i64, runner = part1))]
#[aoc(part2(output = i64, runner = part2))]
pub struct Day02;

const FORWARD_CMD: &str = "forward";
const DOWN_CMD: &str = "down";
const UP_CMD: &str = "up";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cmd_magnitude = s.split_ascii_whitespace();
        let raw_cmd = cmd_magnitude.next().context("invalid command")?;
        let magnitude = cmd_magnitude.next().context("invalid command")?.parse()?;

        match raw_cmd {
            FORWARD_CMD => Ok(Command::Forward(magnitude)),
            DOWN_CMD => Ok(Command::Down(magnitude)),
            UP_CMD => Ok(Command::Up(magnitude)),
            _ => bail!("invalid command"),
        }
    }
}

struct Submarine {
    x_pos: i64,
    y_pos: i64,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            x_pos: 0,
            y_pos: 0,
            aim: 0,
        }
    }

    fn move_in_direction(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(magnitude) => self.x_pos += magnitude,
            Command::Down(magnitude) => self.y_pos += magnitude,
            Command::Up(magnitude) => self.y_pos -= magnitude,
        }
    }

    fn steer_in_direction(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(magnitude) => {
                self.x_pos += magnitude;
                self.y_pos += magnitude * self.aim
            }
            Command::Down(magnitude) => self.aim += magnitude,
            Command::Up(magnitude) => self.aim -= magnitude,
        }
    }
}

pub fn part1(input: Vec<Command>) -> i64 {
    let mut sub = Submarine::new();
    for cmd in input {
        sub.move_in_direction(cmd)
    }
    sub.x_pos * sub.y_pos
}

pub fn part2(input: Vec<Command>) -> i64 {
    let mut sub = Submarine::new();
    for cmd in input {
        sub.steer_in_direction(cmd)
    }
    sub.x_pos * sub.y_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = 150;
        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];
        let expected = 900;
        assert_eq!(expected, part2(input))
    }

    #[test]
    fn command_parsing() {
        assert_eq!(Command::Up(42), "up 42".parse().unwrap());
        assert_eq!(Command::Down(123), "down 123".parse().unwrap());
        assert_eq!(Command::Forward(1), "forward 1".parse().unwrap());
    }
}
