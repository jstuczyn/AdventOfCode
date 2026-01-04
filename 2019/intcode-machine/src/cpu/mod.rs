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

use crate::cpu::alu::Alu;
use crate::cpu::control_unit::ControlUnit;
use crate::cpu::registers::Registers;
use crate::instruction::{Instruction, Operand, OperandMode};
use crate::io::IO;
use crate::memory::Memory;
use anyhow::{Context, bail};
use tracing::{debug, trace};

pub(crate) mod alu;
pub(crate) mod control_unit;
pub(crate) mod registers;

#[derive(Default)]
pub(crate) struct Cpu {
    alu: Alu,
    reg: Registers,
    control_unit: ControlUnit,

    decoded_instruction: Instruction,

    // Internal latches
    operand_a: isize,
    operand_b: isize,
    alu_out: isize,

    write_back_value: isize,
    write_back_addr: usize,
}

impl Cpu {
    pub(crate) fn tick(
        &mut self,
        memory: &mut dyn Memory,
        io: &mut dyn IO,
    ) -> anyhow::Result<Option<()>> {
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
        if self.execute(memory, io)?.is_none() {
            // halted
            return Ok(None);
        }

        // WRITE BACK
        self.write_back(memory, io)?;
        Ok(Some(()))
    }

    fn fetch(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        // read address of the next instruction
        // (updates MDR)
        self.reg.mar = self.reg.pc;
        self.load_memory(memory)?;
        if self.reg.mdr < 0 {
            bail!("Invalid opcode (negative value)")
        }
        self.reg.ir.set_op_code(self.reg.mdr as usize);

        let operands = self.reg.ir.expected_operands();
        for offset in 1..=operands {
            let operand = self.load_memory_from(memory, self.reg.pc + offset)?;
            self.reg.ir.push_operand(operand);
        }

        Ok(())
    }

    fn decode(&mut self) -> anyhow::Result<()> {
        let decoded = self
            .control_unit
            .decode_instruction(self.reg.ir.op_code())?;

        let operand_values = self.reg.ir.raw_operands(decoded.operands_count)?;
        if decoded.operands_modes.len() != operand_values.len() {
            bail!("Inconsistent operand for {}", decoded.op_code)
        }

        debug!("{} {operand_values:?}", decoded.op_code);

        let operands = operand_values
            .into_iter()
            .zip(decoded.operands_modes)
            .map(|(value, mode)| Operand::new(value, mode))
            .collect();

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
    fn load_memory_from(&mut self, memory: &dyn Memory, address: usize) -> anyhow::Result<isize> {
        self.reg.mar = address;
        self.load_memory(memory)?;
        Ok(self.reg.mdr)
    }

    /// Store the provided value to the specified memory address
    fn store_to_memory(
        &mut self,
        memory: &mut dyn Memory,
        address: usize,
        value: isize,
    ) -> anyhow::Result<()> {
        self.reg.mdr = value;
        self.reg.mar = address;
        self.store_memory(memory)
    }

    fn resolve_operand(&mut self, operand: Operand, memory: &dyn Memory) -> anyhow::Result<isize> {
        match operand.mode {
            OperandMode::Position => {
                let addr: usize = operand
                    .value
                    .try_into()
                    .context("Segmentation Fault: negative address value")?;
                self.load_memory_from(memory, addr)
            }
            OperandMode::Immediate => Ok(operand.value),
        }
    }

    fn resolve_operand_a(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        let operand = self.decoded_instruction.operand(0)?;
        self.operand_a = self.resolve_operand(operand, memory)?;
        Ok(())
    }

    fn resolve_operand_b(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        let operand = self.decoded_instruction.operand(1)?;
        self.operand_b = self.resolve_operand(operand, memory)?;
        Ok(())
    }

    fn resolve_write_destination(&mut self) -> anyhow::Result<()> {
        // destination is always the final operand
        let operand = self.decoded_instruction.last_operand()?;

        // destination address resolution is special, we take value literally
        self.write_back_addr = operand
            .value
            .try_into()
            .context("Segmentation Fault: negative address value")?;

        Ok(())
    }

    fn resolve_jump_destination(&mut self, memory: &dyn Memory) -> anyhow::Result<()> {
        // destination is always the final operand
        let operand = self.decoded_instruction.last_operand()?;

        self.write_back_addr = self
            .resolve_operand(operand, memory)?
            .try_into()
            .context("Segmentation Fault: negative address value")?;

        Ok(())
    }

    fn execute(&mut self, memory: &mut dyn Memory, io: &mut dyn IO) -> anyhow::Result<Option<()>> {
        // special case
        if self.decoded_instruction.op_code.is_halt() {
            return Ok(None);
        }
        let sigs = self.decoded_instruction.signals;

        if sigs.pc_update.is_conditional() {
            self.resolve_jump_destination(memory)?;
        }

        if sigs.use_alu {
            if sigs.pc_update.is_sequential() {
                self.resolve_write_destination()?;
            }
            self.resolve_operand_a(memory)?;
            self.resolve_operand_b(memory)?;

            let op = sigs.alu_op;
            self.alu_out = self.alu.execute(op, self.operand_a, self.operand_b);
            self.write_back_value = self.alu_out;
        }

        if sigs.io_op.is_read() {
            self.resolve_write_destination()?;
            self.write_back_value = io.next_input()?;
        }

        if sigs.io_op.is_write() {
            self.resolve_operand_a(memory)?;
            self.write_back_value = self.operand_a;
        }

        Ok(Some(()))
    }

    fn write_back(&mut self, memory: &mut dyn Memory, io: &mut dyn IO) -> anyhow::Result<()> {
        let sig = self.decoded_instruction.signals;

        debug!("write back ({})", self.decoded_instruction.op_code);

        // write ALU result to memory
        if sig.write_enable && sig.write_data_source.is_alu_result() {
            debug!(
                "WRITE {} TO {}",
                self.write_back_value, self.write_back_addr
            );
            self.store_to_memory(memory, self.write_back_addr, self.write_back_value)?;
        }

        // write IO input to memory
        if sig.write_enable && sig.io_op.is_read() && sig.write_data_source.is_io() {
            debug!(
                "IO WRITE {} TO {}",
                self.write_back_value, self.write_back_addr
            );
            self.store_to_memory(memory, self.write_back_addr, self.write_back_value)?
        }

        // write IO output from memory
        if sig.io_op.is_write() {
            debug!("IO OUT {}", self.write_back_value);
            io.write_to_output(self.write_back_value)?;
        }

        // update PC on jump instructions if the alu result is a truthy
        if sig.pc_update.is_conditional() && self.alu_out == 1 {
            debug!("SET PC {}", self.write_back_addr);
            self.reg.pc = self.write_back_addr;
        }

        Ok(())
    }
}
