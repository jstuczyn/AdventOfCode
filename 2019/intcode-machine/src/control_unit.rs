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

use crate::instruction::{ControlSignals, OpCode, WriteDestination};

#[derive(Default)]
pub(crate) struct ControlUnit;

pub(crate) struct DecodedInstruction {
    pub(crate) op_code: OpCode,
    pub(crate) control_signals: ControlSignals,
    pub(crate) operands_count: usize,
}

impl ControlUnit {
    pub(crate) fn decode_opcode(&self, cir: usize) -> anyhow::Result<DecodedInstruction> {
        let op_code = OpCode::try_from(cir)?;

        let mut control_signals = ControlSignals::default();
        let mut operands_count = 0;
        match op_code {
            OpCode::Add | OpCode::Mul => {
                control_signals.use_alu = true;
                control_signals.write_enable = true;
                control_signals.write_destination = WriteDestination::Memory;
                operands_count = 3;
            }
            OpCode::Halt => (),
        }

        Ok(DecodedInstruction {
            op_code,
            control_signals,
            operands_count,
        })
    }
}
