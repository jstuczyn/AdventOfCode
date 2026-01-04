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
use crate::instruction::{
    ControlSignals, IoOperation, OpCode, OperandMode, PcUpdate, WriteDataSource,
};
use anyhow::bail;
use aoc_common::helpers::split_into_digits;
use std::iter::repeat;
use tracing::debug;

#[derive(Default)]
pub(crate) struct ControlUnit;

pub(crate) struct DecodedInstruction {
    pub(crate) op_code: OpCode,
    pub(crate) operands_modes: Vec<OperandMode>,
    pub(crate) control_signals: ControlSignals,
    pub(crate) operands_count: usize,
}

impl ControlUnit {
    fn decode_opcode(&self, cir: usize) -> anyhow::Result<OpCode> {
        let cir_digits = split_into_digits(cir);

        let len = cir_digits.len();
        let last = cir_digits.last();
        let penultimate = len.checked_sub(2).and_then(|id| cir_digits.get(id));

        // "The opcode is a two-digit number"
        let raw_op_code = match (last, penultimate) {
            (Some(last), Some(penultimate)) => penultimate * 10 + last,
            (Some(last), None) => *last,
            // this should be unreachable
            _ => bail!("Malformed opcode digits"),
        };

        OpCode::try_from(raw_op_code)
    }

    fn decode_operand_modes(
        &self,
        cir: usize,
        op_code: OpCode,
    ) -> anyhow::Result<Vec<OperandMode>> {
        let cir_digits = split_into_digits(cir);
        let operands = op_code.operands();

        repeat(0)
            .chain(cir_digits)
            .rev()
            .skip(2)
            .take(operands)
            .map(TryInto::try_into)
            .collect()
    }

    pub(crate) fn decode_instruction(&self, cir: usize) -> anyhow::Result<DecodedInstruction> {
        let op_code = self.decode_opcode(cir)?;
        let operands_modes = self.decode_operand_modes(cir, op_code)?;

        debug!("DECODE: {cir} => {op_code} + {operands_modes:?}");

        let mut control_signals = ControlSignals::default();
        let operands_count = op_code.operands();

        match op_code {
            OpCode::Add => {
                control_signals.use_alu = true;
                control_signals.write_enable = true;
                control_signals.write_data_source = WriteDataSource::AluResult;
                control_signals.alu_op = AluOp::Add;
            }
            OpCode::Mul => {
                control_signals.use_alu = true;
                control_signals.write_enable = true;
                control_signals.write_data_source = WriteDataSource::AluResult;
                control_signals.alu_op = AluOp::Mul;
            }
            OpCode::Input => {
                control_signals.use_alu = false;
                control_signals.write_enable = true;
                control_signals.write_data_source = WriteDataSource::IOInput;
                control_signals.io_op = IoOperation::Read;
            }
            OpCode::Output => {
                control_signals.use_alu = false;
                control_signals.write_enable = false;
                control_signals.io_op = IoOperation::Write;
            }
            OpCode::JumpIfTrue => {
                control_signals.use_alu = true;
                control_signals.alu_op = AluOp::CmpNotZero;
                control_signals.write_enable = false;
                control_signals.pc_update = PcUpdate::Conditional;
                control_signals.write_data_source = WriteDataSource::None;
            }
            OpCode::JumpIfFalse => {
                control_signals.use_alu = true;
                control_signals.alu_op = AluOp::CmpZero;
                control_signals.write_enable = false;
                control_signals.pc_update = PcUpdate::Conditional;
                control_signals.write_data_source = WriteDataSource::None;
            }
            OpCode::SetLT => {
                control_signals.use_alu = true;
                control_signals.alu_op = AluOp::LessThanOutcome;
                control_signals.write_enable = true;
                control_signals.write_data_source = WriteDataSource::AluResult;
            }
            OpCode::SetEq => {
                control_signals.use_alu = true;
                control_signals.alu_op = AluOp::Equals;
                control_signals.write_enable = true;
                control_signals.write_data_source = WriteDataSource::AluResult;
            }
            OpCode::Halt => (),
        }

        Ok(DecodedInstruction {
            op_code,
            operands_modes,
            control_signals,
            operands_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_opcode() {
        let cu = ControlUnit;
        assert_eq!(OpCode::Halt, cu.decode_opcode(99).unwrap());
        assert_eq!(OpCode::Halt, cu.decode_opcode(1099).unwrap());
        assert_eq!(OpCode::Add, cu.decode_opcode(1).unwrap());
        assert_eq!(OpCode::Add, cu.decode_opcode(101).unwrap());
        assert_eq!(OpCode::Add, cu.decode_opcode(1001).unwrap());
    }

    #[test]
    fn decode_operands_modes() {
        use OperandMode::*;
        let cu = ControlUnit;
        assert_eq!(
            Vec::<OperandMode>::new(),
            cu.decode_operand_modes(99, OpCode::Halt).unwrap()
        );
        assert_eq!(
            vec![Position, Position, Position],
            cu.decode_operand_modes(1, OpCode::Add).unwrap()
        );
        assert_eq!(
            vec![Immediate, Position, Position],
            cu.decode_operand_modes(101, OpCode::Add).unwrap()
        );
        assert_eq!(
            vec![Position, Immediate, Position],
            cu.decode_operand_modes(1001, OpCode::Add).unwrap()
        );
        assert_eq!(
            vec![Position, Immediate],
            cu.decode_operand_modes(1001, OpCode::JumpIfFalse).unwrap()
        );
    }
}
