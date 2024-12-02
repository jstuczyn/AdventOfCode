// Copyright 2023 Jedrzej Stuczynski
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

use anyhow::bail;
use aoc_common::types::Pixel;
use std::iter::once;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instruction::Noop)
        } else if let Some(suffix) = s.strip_prefix("addx ") {
            Ok(Instruction::Addx(suffix.parse()?))
        } else {
            bail!("{s} is not a valid instruction")
        }
    }
}

impl Instruction {
    fn addx_value(&self) -> Option<isize> {
        match self {
            Instruction::Noop => None,
            Instruction::Addx(val) => Some(*val),
        }
    }

    fn cost(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    x_register: isize,
    current_cycle: usize,

    instruction_pointer: usize,
    remaining_instruction_work: usize,
    instructions: Vec<Instruction>,
}

impl Cpu {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Cpu {
            x_register: 1,
            current_cycle: 0,
            instruction_pointer: 0,
            remaining_instruction_work: instructions[0].cost(),
            instructions,
        }
    }

    #[cfg(test)]
    pub fn reset(&mut self) {
        self.x_register = 1;
        self.current_cycle = 0;
        self.instruction_pointer = 0;
        self.remaining_instruction_work = self.instructions[0].cost();
    }

    pub fn start_cycle(&mut self) {
        self.current_cycle += 1;
    }

    pub fn execute_cycle(&mut self) {
        self.remaining_instruction_work -= 1;

        // see if the last instruction is done being executed and apply the result and advance the IP
        if self.remaining_instruction_work == 0 {
            let cur = &self.instructions[self.instruction_pointer];
            if let Some(addx) = cur.addx_value() {
                self.x_register += addx
            }
            self.instruction_pointer += 1;

            // see if there are more instructions
            if let Some(next) = self.instructions.get(self.instruction_pointer) {
                self.remaining_instruction_work = next.cost()
            }
        }
    }

    #[cfg(test)]
    pub fn run_cycle(&mut self) {
        self.start_cycle();
        self.execute_cycle();
    }

    #[cfg(test)]
    pub fn x_register(&self) -> isize {
        self.x_register
    }

    #[cfg(test)]
    pub fn current_cycle(&self) -> usize {
        self.current_cycle
    }

    pub fn signal_strength(&self) -> isize {
        self.current_cycle as isize * self.x_register
    }

    #[cfg(test)]
    pub fn run_until_cycle(&mut self, cycle: usize) {
        for _ in 1..cycle {
            self.run_cycle()
        }
        self.start_cycle()
    }

    pub fn signal_strength_sum(&mut self, cycles: &[usize]) -> isize {
        let mut sum = 0;

        for &stop in cycles {
            loop {
                self.start_cycle();

                if self.current_cycle == stop {
                    sum += self.signal_strength();
                    self.execute_cycle();
                    break;
                } else {
                    self.execute_cycle();
                }
            }
        }

        sum
    }
}

// the size is defined as 40x6
pub struct Crt {
    cpu: Cpu,
    display: [[Pixel; 40]; 6],
    // display: [char; 240],
}

impl Crt {
    pub fn new(cpu: Cpu) -> Self {
        Crt {
            cpu,
            display: [[Pixel::default(); 40]; 6],
        }
    }

    pub fn draw(&mut self) {
        for row in &mut self.display {
            for (i, x) in row.iter_mut().enumerate() {
                self.cpu.start_cycle();

                if (self.cpu.x_register - 1..=self.cpu.x_register + 1).contains(&(i as isize)) {
                    *x = Pixel::Active
                }
                self.cpu.execute_cycle();
            }
        }
    }

    pub fn to_display(&self, readable: bool) -> String {
        self.display
            .into_iter()
            .flat_map(|row| {
                row.into_iter()
                    .map(|p| {
                        if readable {
                            p.to_readable()
                        } else {
                            char::from(p)
                        }
                    })
                    .chain(once('\n'))
            })
            .collect()
    }
}
