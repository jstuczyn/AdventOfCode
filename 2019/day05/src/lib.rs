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

pub mod utils;

#[derive(Aoc)]
pub struct Day05;

const ADD_OP_CODE: isize = 1;
const MUL_OP_CODE: isize = 2;
const INPUT_OP_CODE: isize = 3;
const OUTPUT_OP_CODE: isize = 4;
const JMP_TRUE_OP_CODE: isize = 5;
const JMP_FALSE_OP_CODE: isize = 6;
const LESS_THAN_OP_CODE: isize = 7;
const EQUALS_OP_CODE: isize = 8;
const HALT_OP_CODE: isize = 99;

const POSITION_MODE: usize = 0;
const IMMEDIATE_MODE: usize = 1;

// TODO: in hindsight stdout and stdio should have been injected with dependency injection to be
// able to actually test OP3 and OP4

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParamMode {
    Position,
    Immediate,
}

impl TryFrom<usize> for ParamMode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use ParamMode::*;

        match value {
            POSITION_MODE => Ok(Position),
            IMMEDIATE_MODE => Ok(Immediate),
            _ => Err(()),
        }
    }
}

type HeadPositionUpdate = usize;

#[derive(Debug)]
pub enum OpCodeExecutionError {
    TapeError,
    InvalidOpArguments,
    ExecutionFailure,
    ExecutionFinished,
}

impl From<TapeError> for OpCodeExecutionError {
    fn from(_: TapeError) -> Self {
        OpCodeExecutionError::TapeError
    }
}

enum OpCode {
    Add(Vec<ParamMode>),
    Mul(Vec<ParamMode>),
    In,
    Out(Vec<ParamMode>),
    Jt(Vec<ParamMode>),
    Jf(Vec<ParamMode>),
    Lt(Vec<ParamMode>),
    Eq(Vec<ParamMode>),
    Halt,
    #[allow(dead_code)]
    Er(isize),
}

impl OpCode {
    fn execute(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        use OpCode::*;
        match self {
            Add(param_modes) => self.execute_add(tape, head_position, param_modes.clone()),
            Mul(param_modes) => self.execute_mul(tape, head_position, param_modes.clone()),
            Jt(param_modes) => self.execute_jump_true(tape, head_position, param_modes.clone()),
            Jf(param_modes) => self.execute_jump_false(tape, head_position, param_modes.clone()),
            Lt(param_modes) => self.execute_less_than(tape, head_position, param_modes.clone()),
            Eq(param_modes) => self.execute_equals(tape, head_position, param_modes.clone()),

            In => self.execute_input(tape, head_position),
            Out(param_modes) => self.execute_output(tape, head_position, param_modes.clone()),

            Halt => Err(OpCodeExecutionError::ExecutionFinished),
            Er(_) => Err(OpCodeExecutionError::ExecutionFailure),
        }
    }

    fn mode_tape_read(
        &self,
        tape: &Tape,
        tape_idx: usize,
        param_mode: ParamMode,
    ) -> Result<isize, OpCodeExecutionError> {
        let literal_value = tape.read(tape_idx)?;
        match param_mode {
            ParamMode::Position => {
                if literal_value < 0 {
                    Err(OpCodeExecutionError::InvalidOpArguments)
                } else {
                    Ok(tape.read(literal_value as usize)?)
                }
            }
            ParamMode::Immediate => Ok(literal_value),
        }
    }

    fn execute_add(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let result = self.mode_tape_read(tape, head_position + 1, param_modes[0])?
            + self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        let output_idx = tape.read(head_position + 3)?;
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_mul(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let result = self.mode_tape_read(tape, head_position + 1, param_modes[0])?
            * self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        let output_idx = tape.read(head_position + 3)?;
        tape.write(output_idx as usize, result)?;

        Ok(head_position + 4)
    }

    fn execute_less_than(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param1 = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let param2 = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;
        let store_target = tape.read(head_position + 3)?;

        if param1 < param2 {
            tape.write(store_target as usize, 1)?;
        } else {
            tape.write(store_target as usize, 0)?;
        }

        Ok(head_position + 4)
    }

    fn execute_jump_true(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let jump_target = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        if param != 0 {
            Ok(jump_target as usize)
        } else {
            Ok(head_position + 3)
        }
    }

    fn execute_jump_false(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let jump_target = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;

        if param == 0 {
            Ok(jump_target as usize)
        } else {
            Ok(head_position + 3)
        }
    }

    fn execute_equals(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let param1 = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        let param2 = self.mode_tape_read(tape, head_position + 2, param_modes[1])?;
        let store_target = tape.read(head_position + 3)?;

        if param1 == param2 {
            tape.write(store_target as usize, 1)?;
        } else {
            tape.write(store_target as usize, 0)?;
        }

        Ok(head_position + 4)
    }

    fn execute_input(
        &self,
        tape: &mut Tape,
        head_position: usize,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let output_idx = tape.read(head_position + 1)?;

        // Read the user input
        println!("Provide the system required input...");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let input_value = buffer.trim().parse::<isize>().unwrap();

        tape.write(output_idx as usize, input_value)?;

        Ok(head_position + 2)
    }

    fn execute_output(
        &self,
        tape: &mut Tape,
        head_position: usize,
        param_modes: Vec<ParamMode>,
    ) -> Result<HeadPositionUpdate, OpCodeExecutionError> {
        let output_val = self.mode_tape_read(tape, head_position + 1, param_modes[0])?;
        println!("Test result: {}", output_val);
        Ok(head_position + 2)
    }
}

impl From<isize> for OpCode {
    fn from(code: isize) -> Self {
        use OpCode::*;

        // make sure the opcode itself is positive, otherwise we have an invalid execution
        if code < 0 {
            return Er(code);
        }

        let digits = utils::num_to_digits_vec(code as usize);

        let mut opcode_digits: Vec<_> = std::iter::repeat(0)
            .chain(digits.clone())
            .rev()
            .take(2)
            .collect();
        opcode_digits.reverse();
        let op_code_value = utils::digits_vec_to_num(&opcode_digits);

        let num_args = match op_code_value as isize {
            ADD_OP_CODE => 3,
            MUL_OP_CODE => 3,
            JMP_TRUE_OP_CODE => 2,
            JMP_FALSE_OP_CODE => 2,
            LESS_THAN_OP_CODE => 3,
            EQUALS_OP_CODE => 3,
            INPUT_OP_CODE => 0,
            OUTPUT_OP_CODE => 1,
            HALT_OP_CODE => 0,
            _ => 0,
        };

        let param_modes_vec: Vec<_> = std::iter::repeat(0)
            .chain(digits)
            .rev()
            .skip(2)
            .take(num_args)
            .map(|x| ParamMode::try_from(x).unwrap())
            .collect();

        match op_code_value as isize {
            ADD_OP_CODE => Add(param_modes_vec),
            MUL_OP_CODE => Mul(param_modes_vec),
            JMP_TRUE_OP_CODE => Jt(param_modes_vec),
            JMP_FALSE_OP_CODE => Jf(param_modes_vec),
            LESS_THAN_OP_CODE => Lt(param_modes_vec),
            EQUALS_OP_CODE => Eq(param_modes_vec),
            INPUT_OP_CODE => In,
            OUTPUT_OP_CODE => Out(param_modes_vec),
            HALT_OP_CODE => Halt,
            _ => Er(code),
        }
    }
}

#[derive(Debug)]
pub enum TapeError {
    WriteOutOfRangeError,
    ReadOutOfRangeError,
}

pub struct Tape(Vec<isize>);

impl Tape {
    pub fn new(input: Vec<isize>) -> Self {
        Tape(input)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn write(&mut self, position: usize, value: isize) -> Result<(), TapeError> {
        if self.0.len() < position {
            return Err(TapeError::WriteOutOfRangeError);
        }

        self.0[position] = value;
        Ok(())
    }

    fn read(&self, position: usize) -> Result<isize, TapeError> {
        if self.0.len() < position {
            return Err(TapeError::ReadOutOfRangeError);
        }

        Ok(self.0[position])
    }
}

#[derive(Debug)]
pub enum IntcodeMachineError {
    TapeOutOfBoundsError,
    ExecutionFailure,
}

impl From<TapeError> for IntcodeMachineError {
    fn from(_: TapeError) -> Self {
        IntcodeMachineError::TapeOutOfBoundsError
    }
}

impl From<OpCodeExecutionError> for IntcodeMachineError {
    fn from(_: OpCodeExecutionError) -> Self {
        IntcodeMachineError::ExecutionFailure
    }
}

pub struct IntcodeMachine {
    tape: Tape,
    head_position: usize,
    output: isize,
}

impl IntcodeMachine {
    pub fn new(tape: Tape) -> Self {
        IntcodeMachine {
            tape,
            head_position: 0,
            output: 0,
        }
    }

    fn update_head(&mut self, val: HeadPositionUpdate) -> Result<(), IntcodeMachineError> {
        // check if new head is within 0..tape.len()
        if !(0..self.tape.len()).contains(&val) {
            return Err(IntcodeMachineError::TapeOutOfBoundsError);
        }

        self.head_position = val;
        Ok(())
    }

    pub fn run(&mut self) -> Result<isize, IntcodeMachineError> {
        loop {
            let op = OpCode::from(self.tape.read(self.head_position)?);
            let head_update = match op.execute(&mut self.tape, self.head_position) {
                Err(err) => {
                    return match err {
                        OpCodeExecutionError::ExecutionFinished => {
                            self.output = self.tape.read(0)?;
                            Ok(self.output)
                        }
                        _ => Err(IntcodeMachineError::ExecutionFailure),
                    }
                }
                Ok(head_update) => head_update,
            };

            self.update_head(head_update)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine_works_on_negative_values() {
        assert_eq!(
            1101,
            IntcodeMachine::new(Tape::new(vec![1101, 100, -1, 4, 0]))
                .run()
                .unwrap()
        );
    }

    #[cfg(test)]
    mod day2_intcode_machine_reimplementation {
        use super::*;

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode1() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![1, 0, 0, 0, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_tiny_input_with_opcode2() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 3, 0, 3, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_average_size_input() {
            assert_eq!(
                2,
                IntcodeMachine::new(Tape::new(vec![2, 4, 4, 5, 99, 0]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_longer_input() {
            assert_eq!(
                30,
                IntcodeMachine::new(Tape::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]))
                    .run()
                    .unwrap()
            )
        }

        #[test]
        fn produces_expected_output_for_a_lengthy_input() {
            assert_eq!(
                3500,
                IntcodeMachine::new(Tape::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]))
                    .run()
                    .unwrap()
            )
        }

        // #[test]
        // fn produces_expected_output_for_day2_input() {
        //     let mut day2_tape = Tape::new(read_input_file("inputs/2019/day02"));
        //     // do the substitutions
        //     day2_tape.0[1] = 12;
        //     day2_tape.0[2] = 2;
        //     assert_eq!(4_138_687, IntcodeMachine::new(day2_tape).run().unwrap())
        // }
    }

    #[cfg(test)]
    mod opcode_parsing {
        use super::*;

        #[test]
        fn works_for_basic_addition() {
            match OpCode::from(1) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Position, param_vec[0]);
                    assert_eq!(ParamMode::Position, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }

                _ => panic!("expected Add"),
            }
        }

        #[test]
        fn works_for_basic_addition_with_zero_prefix() {
            match OpCode::from(101) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Immediate, param_vec[0]);
                    assert_eq!(ParamMode::Position, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }
                _ => panic!("expected Add"),
            }
        }

        #[test]
        fn work_for_addition_with_implicit_mode() {
            match OpCode::from(1101) {
                OpCode::Add(param_vec) => {
                    assert_eq!(ParamMode::Immediate, param_vec[0]);
                    assert_eq!(ParamMode::Immediate, param_vec[1]);
                    assert_eq!(ParamMode::Position, param_vec[2]);
                }
                _ => panic!("expected Add"),
            }
        }
    }
}
