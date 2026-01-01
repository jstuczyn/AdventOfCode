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

use crate::alu::Alu;
use crate::control_unit::ControlUnit;
use crate::instruction::{Instruction, OpCode, WriteDestination};
use crate::memory::Memory;
use crate::registers::Registers;
use tracing::{debug, trace};

#[derive(Default)]
pub(crate) struct Cpu {
    alu: Alu,
    reg: Registers,
    control_unit: ControlUnit,

    decoded_instruction: Instruction,

    // Internal latches
    operand_a: usize,
    operand_b: usize,
    alu_out: usize,
    dest_addr: usize,
    // imm: usize,
}

impl Cpu {
    pub(crate) fn tick(&mut self, memory: &mut dyn Memory) -> anyhow::Result<Option<()>> {
        debug!("tick");
        trace!("memory: {:?}", memory.dump());
        self.reg.log_state();

        // FETCH
        self.reg.ir.clear();
        self.fetch(memory)?;
        self.reg.pc += self.reg.ir.size();

        // DECODE
        self.decode()?;

        // EXECUTE
        if self.execute(memory)?.is_none() {
            // halted
            return Ok(None);
        }

        // WRITE BACK
        self.write_back(memory)?;
        Ok(Some(()))
    }

    fn fetch(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        // read address of the next instruction
        // (updates MDR)
        self.reg.mar = self.reg.pc;
        self.load_memory(memory)?;
        self.reg.ir.set_op_code(self.reg.mdr);

        let operands = self.reg.ir.expected_operands();
        for offset in 1..=operands {
            let operand = self.load_memory_from(memory, self.reg.pc + offset)?;
            self.reg.ir.push_operand(operand);
        }

        Ok(())
    }

    fn decode(&mut self) -> anyhow::Result<()> {
        let decoded = self.control_unit.decode_opcode(self.reg.ir.op_code())?;

        let operands = self.reg.ir.operands(decoded.operands_count)?;
        self.decoded_instruction =
            Instruction::new(decoded.op_code, operands, decoded.control_signals);

        Ok(())
    }

    /// Load the value from the address specified by MAR register into MDR register
    fn load_memory(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        self.reg.mdr = memory.load(self.reg.mar)?;
        Ok(())
    }

    /// Store the value from the MDR register into memory address specified by the MAR register
    fn store_memory(&mut self, memory: &mut dyn Memory) -> anyhow::Result<()> {
        memory.store(self.reg.mar, self.reg.mdr)
    }

    /// Load memory value from the specified address
    fn load_memory_from(&mut self, memory: &dyn Memory, address: usize) -> anyhow::Result<usize> {
        self.reg.mar = address;
        self.load_memory(memory)?;
        Ok(self.reg.mdr)
    }

    /// Store the provided value to the specified memory address
    fn store_to_memory(
        &mut self,
        memory: &mut dyn Memory,
        address: usize,
        value: usize,
    ) -> anyhow::Result<()> {
        self.reg.mdr = value;
        self.reg.mar = address;
        self.store_memory(memory)
    }

    fn load_operand_a(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        let addr = self.decoded_instruction.operand(0)?;
        let op = self.load_memory_from(memory, addr)?;
        self.operand_a = op;
        trace!("load op A from {addr}: {op}");
        Ok(())
    }

    fn load_operand_b(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        let addr = self.decoded_instruction.operand(1)?;
        let op = self.load_memory_from(memory, addr)?;
        self.operand_b = op;
        trace!("load op B from {addr}: {op}");
        Ok(())
    }

    fn execute(&mut self, memory: &mut dyn Memory) -> anyhow::Result<Option<()>> {
        let op_code = self.decoded_instruction.op_code;
        if self.decoded_instruction.signals.use_alu {
            self.load_operand_a(memory)?;
            self.load_operand_b(memory)?;
        }

        match op_code {
            OpCode::Add | OpCode::Mul => {
                self.dest_addr = self.decoded_instruction.operand(2)?;

                self.alu_out = self.alu.execute(op_code, self.operand_a, self.operand_b)?;
                self.reg.acc = self.alu_out;

                Ok(Some(()))
            }
            OpCode::Halt => Ok(None),
        }
    }

    fn write_back(&mut self, memory: &mut dyn Memory) -> anyhow::Result<()> {
        // store the result in the specified address
        let sig = self.decoded_instruction.signals;
        if sig.write_enable && sig.write_destination == WriteDestination::Memory {
            self.store_to_memory(memory, self.dest_addr, self.reg.acc)?;
        }

        Ok(())
    }
}
