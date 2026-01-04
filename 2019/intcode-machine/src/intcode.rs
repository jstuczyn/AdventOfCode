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

use crate::cpu::Cpu;
use crate::io::{self, NoIO};
use crate::memory::Memory;
use anyhow::bail;

pub struct IntCodeMachine<M, IO = NoIO> {
    cpu: Cpu,
    memory: M,
    io: IO,
    instruction: usize,
    exec_limit: usize,
}

impl<M, IO> IntCodeMachine<M, IO> {
    pub fn run(&mut self) -> anyhow::Result<&mut Self>
    where
        M: Memory,
        IO: io::IO,
    {
        loop {
            if self.cpu.tick(&mut self.memory, &mut self.io)?.is_none() {
                return Ok(self);
            }
            if self.instruction > self.exec_limit {
                bail!("run out of execution budget")
            }
            self.instruction += 1;
        }
    }

    pub fn with_io<IO2>(self, io: IO2) -> IntCodeMachine<M, IO2> {
        IntCodeMachine {
            cpu: self.cpu,
            memory: self.memory,
            io,
            instruction: self.instruction,
            exec_limit: self.exec_limit,
        }
    }

    pub fn load(memory: M, io: IO) -> Self {
        IntCodeMachine {
            cpu: Default::default(),
            memory,
            io,
            instruction: 0,
            // reasonable default
            exec_limit: 1_000_000,
        }
    }

    #[must_use]
    pub fn with_exec_limit(&mut self, n: usize) -> &mut Self {
        self.exec_limit = n;
        self
    }

    pub fn dump_memory(&self) -> Vec<isize>
    where
        M: Memory,
    {
        self.memory.dump()
    }

    pub fn memory_output(&self) -> isize
    where
        M: Memory,
    {
        self.memory.load(0).unwrap_or_default()
    }

    pub fn io(&self) -> &IO {
        &self.io
    }
}
