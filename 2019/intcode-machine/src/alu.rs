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

use crate::instruction::OpCode;
use anyhow::bail;
use tracing::debug;

#[derive(Default)]
pub(crate) struct Alu {
    arithmetic_unit: ArithmeticUnit,

    logic_unit: LogicUnit,
}

// The arithmetic unit is responsible for all numerical operations in a computer, such as addition and subtraction.
// It also operates other simple things like increment operation.
#[derive(Default)]
struct ArithmeticUnit;

impl ArithmeticUnit {
    #[inline]
    fn add(&self, a: usize, b: usize) -> usize {
        debug!("EXEC: {a} + {b}");
        a + b
    }

    #[inline]
    fn mul(&self, a: usize, b: usize) -> usize {
        debug!("EXEC: {a} * {b}");
        a * b
    }
}

// The logic unit handles logical operations like AND, OR, and XOR, instead of arithmetic operations. It also performs numerical tests. For example, it checks whether the number is negative or not.
// It also controls if the output of the ALU is zero.
#[derive(Default)]
struct LogicUnit;

impl Alu {
    pub(crate) fn execute(
        &self,
        op: OpCode,
        operand_a: usize,
        operand_b: usize,
    ) -> anyhow::Result<usize> {
        match op {
            OpCode::Add => Ok(self.arithmetic_unit.add(operand_a, operand_b)),
            OpCode::Mul => Ok(self.arithmetic_unit.mul(operand_a, operand_b)),
            OpCode::Halt => bail!("illegal ALU instruction"),
        }
    }
}
