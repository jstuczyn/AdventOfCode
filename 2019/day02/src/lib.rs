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
use intcode_machine::{IntCodeMachine, NoIO};

#[derive(Aoc)]
#[aoc(input = Vec<isize>)]
#[aoc(parser = CommaSeparatedParser)]
#[aoc(part1(output = isize, runner = part1))]
#[aoc(part2(output = isize, runner = part2))]
pub struct Day02;

pub fn part1(mut memory: Vec<isize>) -> isize {
    memory[1] = 12;
    memory[2] = 2;
    let mut intcode_machine = IntCodeMachine::load(memory, NoIO);
    if intcode_machine
        .run()
        .inspect_err(|err| eprintln!("intcode execution failure: {err}"))
        .is_err()
    {
        return 0;
    }

    intcode_machine.memory_output()
}

pub fn part2(memory: Vec<isize>) -> isize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut memory = memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            let mut intcode_machine = IntCodeMachine::load(memory, NoIO);
            if intcode_machine
                .run()
                .inspect_err(|err| eprintln!("intcode execution failure: {err}"))
                .is_err()
            {
                return 0;
            }
            if intcode_machine.memory_output() == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    eprintln!("failed to solve part 2");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod intcode_machine {
        use super::*;

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode1() -> anyhow::Result<()> {
            let mut intcode = IntCodeMachine::load(vec![1, 0, 0, 0, 99], NoIO);
            intcode.run()?;
            assert_eq!(2, intcode.memory_output());
            Ok(())
        }

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode2() -> anyhow::Result<()> {
            let mut intcode = IntCodeMachine::load(vec![2, 3, 0, 3, 99], NoIO);
            intcode.run()?;
            assert_eq!(2, intcode.memory_output());
            Ok(())
        }

        #[test]
        fn produces_expected_output_for_average_size_input() -> anyhow::Result<()> {
            let mut intcode = IntCodeMachine::load(vec![2, 4, 4, 5, 99, 0], NoIO);
            intcode.run()?;
            assert_eq!(2, intcode.memory_output());
            Ok(())
        }

        #[test]
        fn produces_expected_output_for_longer_input() -> anyhow::Result<()> {
            let mut intcode = IntCodeMachine::load(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], NoIO);
            intcode.run()?;

            assert_eq!(30, intcode.memory_output());
            Ok(())
        }

        #[test]
        fn produces_expected_output_for_a_lengthy_input() -> anyhow::Result<()> {
            let mut intcode =
                IntCodeMachine::load(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], NoIO);
            intcode.run()?;
            assert_eq!(3500, intcode.memory_output());
            Ok(())
        }
    }
}
