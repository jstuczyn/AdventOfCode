// Copyright 2023 Jedrzej Stuczynski
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

use crate::types::Cpu;
use aoc_solution::Aoc;
use common::parsing::LineParser;
use types::Instruction;

mod types;

#[derive(Aoc)]
#[aoc(input = Vec<Instruction>)]
#[aoc(parser = LineParser)]
#[aoc(part1(output = isize, runner = part1))]
#[aoc(part2(output = String, runner = part2))]
pub struct Day10;

pub fn part1(input: Vec<Instruction>) -> isize {
    Cpu::new(input).signal_strength_sum(&[20, 60, 100, 140, 180, 220])
}

pub fn part2(input: Vec<Instruction>) -> String {
    "unimplemented".into()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::types::Cpu;
    use aoc_solution::AocSolution;

    fn sample_input1() -> Vec<Instruction> {
        Day10::parse_input(
            r#"noop
addx 3
addx -5"#,
        )
        .unwrap()
    }

    fn sample_input2() -> Vec<Instruction> {
        Day10::parse_input(
            r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#,
        )
        .unwrap()
    }

    #[test]
    fn small_input_register() {
        let sample = sample_input1();
        let mut cpu = Cpu::new(sample);

        // cycle1
        cpu.run_cycle();
        assert_eq!(1, cpu.x_register());

        // cycle2
        cpu.run_cycle();
        assert_eq!(1, cpu.x_register());

        // cycle3
        cpu.run_cycle();
        assert_eq!(4, cpu.x_register());

        // cycle4
        cpu.run_cycle();
        assert_eq!(4, cpu.x_register());

        // cycle5
        cpu.run_cycle();
        assert_eq!(-1, cpu.x_register());
    }

    #[test]
    fn sample_signal() {
        let sample = sample_input2();
        let mut cpu = Cpu::new(sample);

        cpu.run_until_cycle(20);
        assert_eq!(21, cpu.x_register());
        assert_eq!(20, cpu.current_cycle());
        assert_eq!(420, cpu.signal_strength());

        cpu.reset();
        cpu.run_until_cycle(60);
        assert_eq!(19, cpu.x_register());
        assert_eq!(60, cpu.current_cycle());
        assert_eq!(1140, cpu.signal_strength());

        cpu.reset();
        cpu.run_until_cycle(100);
        assert_eq!(18, cpu.x_register());
        assert_eq!(100, cpu.current_cycle());
        assert_eq!(1800, cpu.signal_strength());

        cpu.reset();
        cpu.run_until_cycle(140);
        assert_eq!(21, cpu.x_register());
        assert_eq!(140, cpu.current_cycle());
        assert_eq!(2940, cpu.signal_strength());

        cpu.reset();
        cpu.run_until_cycle(180);
        assert_eq!(16, cpu.x_register());
        assert_eq!(180, cpu.current_cycle());
        assert_eq!(2880, cpu.signal_strength());

        cpu.reset();
        cpu.run_until_cycle(220);
        assert_eq!(18, cpu.x_register());
        assert_eq!(220, cpu.current_cycle());
        assert_eq!(3960, cpu.signal_strength());

        cpu.reset();
        assert_eq!(
            13140,
            cpu.signal_strength_sum(&[20, 60, 100, 140, 180, 220])
        )
    }

    // #[test]
    // fn part1_sample_input() {
    //     let expected = ();
    //     assert_eq!(expected, part1(sample_input()))
    // }
    //
    // #[test]
    // fn part2_sample_input() {
    //     let expected = ();
    //     assert_eq!(expected, part2(sample_input()))
    // }
}
