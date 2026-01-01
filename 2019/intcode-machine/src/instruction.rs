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

use anyhow::{Context, anyhow};
use std::fmt::{Display, Formatter};

// an unfortunate consequence of the intcode instruction set
#[inline]
pub const fn raw_op_code_operands(op_code: usize) -> usize {
    match op_code {
        op_code if op_code == OpCode::Add as usize => 3,
        op_code if op_code == OpCode::Mul as usize => 3,
        op_code if op_code == OpCode::Halt as usize => 0,
        _ => {
            // decode will fail anyway
            0
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(usize)]
pub(crate) enum OpCode {
    /// ADD two numbers together
    Add = 1,

    /// MULTIPLY two numbers together
    Mul = 2,

    /// Halt the program execution
    #[default]
    Halt = 99,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Add => write!(f, "ADD"),
            OpCode::Mul => write!(f, "MUL"),
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
            n if n == Self::Halt as usize => Ok(Self::Halt),
            other => Err(anyhow!("unknown opcode {other}")),
        }
    }
}

/// Full decoded instruction
#[derive(Debug, Default)]
pub(crate) struct Instruction {
    /// Specifies the action that’s required to be performed.
    pub(crate) op_code: OpCode,

    pub(crate) operands: Vec<usize>,

    pub(crate) signals: ControlSignals,
    // control signals, immediate, destination
}

impl Instruction {
    pub(crate) fn new(op_code: OpCode, operands: Vec<usize>, signals: ControlSignals) -> Self {
        Instruction {
            op_code,
            operands,
            signals,
        }
    }

    pub(crate) fn operand(&self, index: usize) -> anyhow::Result<usize> {
        self.operands
            .get(index)
            .copied()
            .context(format!("invalid operand {index}"))
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct ControlSignals {
    /// Should ALU be used?
    pub(crate) use_alu: bool,
    pub(crate) pc_update: PcUpdate,
    pub(crate) write_enable: bool,
    pub(crate) write_destination: WriteDestination,
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) enum PcUpdate {
    /// Increment the current value of the Program Counter
    #[default]
    Sequential,

    /// Load the value from the memory address specified in the operand and assign it to PC
    MemTarget,
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) enum WriteDestination {
    #[default]
    Memory,
}
