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

use crate::instruction::raw_op_code_operands;
use anyhow::bail;
use std::fmt::Debug;
use tracing::trace;

#[derive(Default)]
pub(crate) struct Registers {
    /// Program Counter
    // Also known as Instruction Address Register
    /// Holds address of instruction to be executed next
    pub(crate) pc: usize,

    // keep track where we are in a program
    /// Instruction Register
    /// Holds instruction while it's decoded/executed
    pub(crate) ir: InstructionRegister,

    /// Memory Address Register
    pub(crate) mar: usize,

    /// Memory Data Register
    pub(crate) mdr: isize,
    // /// Destination Register
    // pub(crate) dest: usize,
    // /// General Purpose Registers
    // /// Hold intermediary results or addresses during execution of instructions
    // pub(crate) general_purpose: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct InstructionRegister {
    op_code: usize,
    operands: Vec<isize>,
}

impl InstructionRegister {
    pub(crate) fn set_op_code(&mut self, op_code: usize) {
        self.op_code = op_code;
    }

    pub(crate) fn op_code(&self) -> usize {
        self.op_code
    }

    pub(crate) fn expected_operands(&self) -> usize {
        raw_op_code_operands(self.op_code)
    }

    pub(crate) fn push_operand(&mut self, operand: isize) {
        self.operands.push(operand);
    }

    pub(crate) fn clear(&mut self) {
        self.op_code = 0;
        self.operands.clear();
    }

    pub(crate) fn size(&self) -> usize {
        1 + self.operands.len()
    }

    pub(crate) fn raw_operands(&self, count: usize) -> anyhow::Result<Vec<isize>> {
        if self.operands.len() < count {
            bail!(
                "insufficient number of operands in instruction register. want: {count}, got: {}. opcode: {}",
                self.operands.len(),
                self.op_code
            );
        }
        Ok(self.operands[..count].to_vec())
    }
}

impl Registers {
    pub(crate) fn log_state(&self) {
        trace!("PC: {}", self.pc);
        trace!("IR: {:#?}", self.ir);
        trace!("MAR: {}", self.mar);
        trace!("MDR: {}", self.mdr);
    }
}
