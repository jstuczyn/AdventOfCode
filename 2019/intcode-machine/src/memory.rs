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

use anyhow::{Context, bail};

pub trait Memory {
    fn load(&self, address: usize) -> anyhow::Result<usize>;

    fn store(&mut self, address: usize, data: usize) -> anyhow::Result<()>;

    fn dump(&self) -> Vec<usize>;
}

impl Memory for Vec<usize> {
    fn load(&self, address: usize) -> anyhow::Result<usize> {
        self.get(address)
            .copied()
            .context(format!("Segmentation Fault load from {address}"))
    }

    fn store(&mut self, address: usize, data: usize) -> anyhow::Result<()> {
        if self.len() < address {
            bail!("Segmentation Fault store at {address}");
        }
        self[address] = data;
        Ok(())
    }

    fn dump(&self) -> Vec<usize> {
        self.clone()
    }
}
