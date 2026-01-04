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
use std::collections::VecDeque;
use std::io;
use std::io::Write;
use std::io::stdout;

pub trait IO {
    fn next_input(&mut self) -> anyhow::Result<isize>;

    fn write_to_output(&mut self, output: isize) -> anyhow::Result<()>;
}

pub struct InteractiveIO;

impl IO for InteractiveIO {
    fn next_input(&mut self) -> anyhow::Result<isize> {
        print!("> IN: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;
        input.trim().parse().context("malformed input")
    }

    fn write_to_output(&mut self, output: isize) -> anyhow::Result<()> {
        writeln!(stdout(), "{}", output)?;
        Ok(())
    }
}

pub struct HardcodedIO {
    pub(crate) queued_input: VecDeque<isize>,
    pub(crate) queued_output: VecDeque<isize>,
}

impl IO for HardcodedIO {
    fn next_input(&mut self) -> anyhow::Result<isize> {
        self.queued_input.pop_front().context("exhausted input")
    }

    fn write_to_output(&mut self, output: isize) -> anyhow::Result<()> {
        self.queued_output.push_back(output);
        Ok(())
    }
}

impl HardcodedIO {
    pub fn new(input: impl IntoIterator<Item = isize>) -> Self {
        HardcodedIO {
            queued_input: input.into_iter().collect(),
            queued_output: Default::default(),
        }
    }

    pub fn last_output(&self) -> Option<isize> {
        self.queued_output.back().copied()
    }
}

#[derive(Default)]
pub struct NoIO;

impl IO for NoIO {
    fn next_input(&mut self) -> anyhow::Result<isize> {
        bail!("no io device available")
    }

    fn write_to_output(&mut self, output: isize) -> anyhow::Result<()> {
        let _ = output;
        bail!("no io device available")
    }
}
