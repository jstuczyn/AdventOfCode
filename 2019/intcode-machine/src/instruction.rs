// Copyright 2026 Jedrzej Stuczynski
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

use crate::cpu::alu::AluOp;
use anyhow::{Context, anyhow};
use std::fmt::{Display, Formatter};

// an unfortunate consequence of the intcode instruction set
#[inline]
pub const fn raw_op_code_operands(op_code: usize) -> usize {
    match normalise_op_code(op_code) {
        op_code if op_code == OpCode::Add as usize => 3,
        op_code if op_code == OpCode::Mul as usize => 3,
        op_code if op_code == OpCode::Input as usize => 1,
        op_code if op_code == OpCode::Output as usize => 1,
        op_code if op_code == OpCode::JumpIfTrue as usize => 2,
        op_code if op_code == OpCode::JumpIfFalse as usize => 2,
        op_code if op_code == OpCode::SetLT as usize => 3,
        op_code if op_code == OpCode::SetEq as usize => 3,
        op_code if op_code == OpCode::Halt as usize => 0,
        _ => {
            // decode will fail anyway
            0
        }
    }
}

#[inline]
pub const fn normalise_op_code(raw: usize) -> usize {
    if raw > 100 {
        let last_two = raw % 100;
        let last = last_two % 10;
        let penultimate = last_two / 10;
        if penultimate != 0 {
            penultimate * 10 + last
        } else {
            last
        }
    } else {
        raw
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(usize)]
pub(crate) enum OpCode {
    /// ADD two numbers together
    Add = 1,

    /// MULTIPLY two numbers together
    Mul = 2,

    /// Take a single integer input and save it to the specified address
    Input = 3,

    /// Output value at the specified address
    Output = 4,

    /// If the first parameter is non-zero, set the IP to the value of the second parameter
    JumpIfTrue = 5,

    /// If the first parameter is zero, set the IP to the value of the second parameter
    JumpIfFalse = 6,

    /// If the first parameter is less than the second parameter,
    /// store 1 at the position specified by the third parameter, otherwise store 0.
    SetLT = 7,

    /// If the first parameter is equals to the second parameter,
    /// store 1 at the position specified by the third parameter, otherwise store 0.
    SetEq = 8,

    /// Halt the program execution
    #[default]
    Halt = 99,
}

impl OpCode {
    #[inline]
    pub(crate) const fn operands(&self) -> usize {
        raw_op_code_operands(*self as usize)
    }

    pub(crate) fn is_halt(&self) -> bool {
        *self == OpCode::Halt
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Mul => write!(f, "MUL"),
            OpCode::Input => write!(f, "IN"),
            OpCode::Output => write!(f, "OUT"),
            OpCode::JumpIfTrue => write!(f, "JMPT"),
            OpCode::JumpIfFalse => write!(f, "JMPF"),
            OpCode::SetLT => write!(f, "LT"),
            OpCode::SetEq => write!(f, "EQ"),
            OpCode::Halt => write!(f, "HLT"),
        }
    }
}

impl TryFrom<usize> for OpCode {
    type Error = anyhow::Error;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            n if n == Self::Add as usize => Ok(Self::Add),
            n if n == Self::Mul as usize => Ok(Self::Mul),
            n if n == Self::Input as usize => Ok(Self::Input),
            n if n == Self::Output as usize => Ok(Self::Output),
            n if n == Self::JumpIfTrue as usize => Ok(Self::JumpIfTrue),
            n if n == Self::JumpIfFalse as usize => Ok(Self::JumpIfFalse),
            n if n == Self::SetLT as usize => Ok(Self::SetLT),
            n if n == Self::SetEq as usize => Ok(Self::SetEq),
            n if n == Self::Halt as usize => Ok(Self::Halt),
            other => Err(anyhow!("unknown opcode {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(usize)]
pub(crate) enum OperandMode {
    Position = 0,
    Immediate = 1,
}

impl TryFrom<usize> for OperandMode {
    type Error = anyhow::Error;

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            n if n == Self::Position as usize => Ok(Self::Position),
            n if n == Self::Immediate as usize => Ok(Self::Immediate),
            other => Err(anyhow!("unknown operand mode {other}")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Operand {
    pub(crate) value: isize,
    pub(crate) mode: OperandMode,
}

impl Operand {
    pub(crate) fn new(value: isize, mode: OperandMode) -> Self {
        Operand { value, mode }
    }
}

/// Full decoded instruction
#[derive(Debug, Default)]
pub(crate) struct Instruction {
    /// Specifies the action that’s required to be performed.
    pub(crate) op_code: OpCode,

    pub(crate) operands: Vec<Operand>,

    pub(crate) signals: ControlSignals,
}

impl Instruction {
    pub(crate) fn new(op_code: OpCode, operands: Vec<Operand>, signals: ControlSignals) -> Self {
        Instruction {
            op_code,
            operands,
            signals,
        }
    }

    pub(crate) fn operand(&self, index: usize) -> anyhow::Result<Operand> {
        self.operands
            .get(index)
            .copied()
            .context(format!("invalid operand {index}"))
    }

    pub(crate) fn last_operand(&self) -> anyhow::Result<Operand> {
        self.operands
            .last()
            .copied()
            .context("no operands available")
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct ControlSignals {
    /// Should ALU be used?
    pub(crate) use_alu: bool,

    // ignored if use_alu = false
    /// Alu operation to be executed
    pub(crate) alu_op: AluOp,

    pub(crate) pc_update: PcUpdate,

    pub(crate) write_enable: bool,

    // ignored if write_enable = false
    pub(crate) write_data_source: WriteDataSource,

    pub(crate) io_op: IoOperation,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) enum PcUpdate {
    /// Increment the current value of the Program Counter (based on the number of operands)
    #[default]
    Sequential,

    /// Update of the Program Counter depends on the result of the execution of the current instruction
    Conditional,
    // We have no unconditional jumps
}

impl PcUpdate {
    pub(crate) fn is_conditional(&self) -> bool {
        *self == PcUpdate::Conditional
    }

    pub(crate) fn is_sequential(&self) -> bool {
        *self == PcUpdate::Sequential
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) enum WriteDataSource {
    /// No write-back (e.g. jumps)
    None,

    /// Use result of the ALU operation (ALU_OUT)
    #[default]
    AluResult,

    /// Use the IO Device
    IOInput,
}

impl WriteDataSource {
    pub(crate) fn is_alu_result(&self) -> bool {
        *self == WriteDataSource::AluResult
    }

    pub(crate) fn is_io(&self) -> bool {
        *self == WriteDataSource::IOInput
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) enum IoOperation {
    /// Don't interact with the IO Device
    #[default]
    None,

    /// Read from the IO Device
    Read,

    /// Write to the IO Device
    Write,
}

impl IoOperation {
    pub(crate) fn is_read(&self) -> bool {
        *self == IoOperation::Read
    }

    pub(crate) fn is_write(&self) -> bool {
        *self == IoOperation::Write
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_code_normalisation() {
        assert_eq!(10, normalise_op_code(10));
        assert_eq!(1, normalise_op_code(1));
        assert_eq!(1, normalise_op_code(101));
        assert_eq!(99, normalise_op_code(99));
        assert_eq!(99, normalise_op_code(199));
        assert_eq!(1, normalise_op_code(10001));
        assert_eq!(1, normalise_op_code(11101));
        assert_eq!(10, normalise_op_code(1010));
    }
}
