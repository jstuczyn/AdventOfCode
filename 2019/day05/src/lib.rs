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

use aoc_common::parsing::CommaSeparatedParser;
use aoc_solution::Aoc;
use intcode_machine::{HardcodedIO, IntCodeMachine};

#[derive(Aoc)]
#[aoc(input = Vec<isize>)]
#[aoc(parser = CommaSeparatedParser)]
#[aoc(part1(output = isize, runner = part1))]
#[aoc(part2(output = isize, runner = part2))]
pub struct Day05;

pub fn part1(memory: Vec<isize>) -> isize {
    let io = HardcodedIO::new([1]);
    let mut intcode_machine = IntCodeMachine::load(memory, io);
    if intcode_machine
        .run()
        .inspect_err(|err| eprintln!("intcode execution failure: {err}"))
        .is_err()
    {
        return 0;
    }

    intcode_machine.io().last_output().unwrap_or_default()
}

pub fn part2(memory: Vec<isize>) -> isize {
    let io = HardcodedIO::new([5]);
    let mut intcode_machine = IntCodeMachine::load(memory, io);
    if intcode_machine
        .run()
        .inspect_err(|err| eprintln!("intcode execution failure: {err}"))
        .is_err()
    {
        return 0;
    }

    intcode_machine.io().last_output().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    fn run_test_machine(memory: &[isize], input: isize) -> anyhow::Result<isize> {
        IntCodeMachine::load(memory.to_vec(), HardcodedIO::new([input]))
            .run()?
            .io()
            .last_output()
            .context("no output")
    }

    #[test]
    fn sample_small_comp_8_inputs() -> anyhow::Result<()> {
        let below_8 = [-99999, -42, -8, 0, 1, 7];
        let not_below_8 = [8, 9, 42, 99999];

        let memory = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(1, run_test_machine(&memory, 8)?);
        assert_eq!(0, run_test_machine(&memory, 42)?);

        let memory = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        for below in below_8 {
            assert_eq!(1, run_test_machine(&memory, below)?);
        }
        for not_below in not_below_8 {
            assert_eq!(0, run_test_machine(&memory, not_below)?);
        }

        let memory = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(1, run_test_machine(&memory, 8)?);
        assert_eq!(0, run_test_machine(&memory, 42)?);

        let memory = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        for below in below_8 {
            assert_eq!(1, run_test_machine(&memory, below)?);
        }
        for not_below in not_below_8 {
            assert_eq!(0, run_test_machine(&memory, not_below)?);
        }

        Ok(())
    }

    #[test]
    fn sample_jump_tests() -> anyhow::Result<()> {
        let memory = [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(0, run_test_machine(&memory, 0)?);
        assert_eq!(1, run_test_machine(&memory, 42)?);

        let memory = [3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(0, run_test_machine(&memory, 0)?);
        assert_eq!(1, run_test_machine(&memory, 42)?);

        Ok(())
    }

    #[test]
    fn sample_larger_input() -> anyhow::Result<()> {
        let memory = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        let below_8 = [-99999, -42, -8, 0, 1, 7];

        for b8 in below_8 {
            assert_eq!(999, run_test_machine(&memory, b8)?);
        }

        assert_eq!(1000, run_test_machine(&memory, 8)?);

        let above_8 = [9, 42, 99999];
        for a8 in above_8 {
            assert_eq!(1001, run_test_machine(&memory, a8)?,);
        }

        Ok(())
    }
}
