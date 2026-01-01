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
use crate::memory::Memory;
use anyhow::bail;

pub struct IntCodeMachine<M> {
    cpu: Cpu,
    memory: M,
    instruction: usize,
    exec_limit: usize,
}

impl<M> IntCodeMachine<M> {
    pub fn run(&mut self) -> anyhow::Result<()>
    where
        M: Memory,
    {
        loop {
            if self.cpu.tick(&mut self.memory)?.is_none() {
                return Ok(());
            }
            if self.instruction > self.exec_limit {
                println!("run out of execution budget");
                bail!("run out of execution budget")
            }
            self.instruction += 1;
        }
    }

    pub fn load(memory: M) -> Self {
        IntCodeMachine {
            cpu: Default::default(),
            memory,
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

    pub fn dump_memory(&self) -> Vec<usize>
    where
        M: Memory,
    {
        self.memory.dump()
    }

    pub fn output(&self) -> usize
    where
        M: Memory,
    {
        self.memory.load(0).unwrap_or_default()
    }
}
