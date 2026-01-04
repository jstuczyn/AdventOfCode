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

use std::fmt::{Display, Formatter};
use tracing::debug;

#[derive(Default)]
pub(crate) struct Alu {
    arithmetic_unit: ArithmeticUnit,

    logic_unit: LogicUnit,
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) enum AluOp {
    Add,

    Mul,

    // does not compute less than directly,
    // but rather outputs `1` or `0` to determine value that should be written back
    LessThanOutcome,

    // does not compute equals directly,
    // but rather outputs `1` or `0` to determine value that should be written back
    Equals,

    CmpZero,

    CmpNotZero,

    #[default]
    Pass,
}

impl Display for AluOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AluOp::Add => write!(f, "ADD"),
            AluOp::Mul => write!(f, "MUL"),
            AluOp::LessThanOutcome => write!(f, "LT"),
            AluOp::Equals => write!(f, "EQ"),
            AluOp::CmpZero => write!(f, "CMPZ"),
            AluOp::CmpNotZero => write!(f, "CMPNZ"),
            AluOp::Pass => write!(f, "PASS"),
        }
    }
}

// The arithmetic unit is responsible for all numerical operations in a computer, such as addition and subtraction.
// It also operates other simple things like increment operation.
#[derive(Default)]
struct ArithmeticUnit;

impl ArithmeticUnit {
    #[inline]
    const fn add(&self, a: isize, b: isize) -> isize {
        a + b
    }

    #[inline]
    const fn mul(&self, a: isize, b: isize) -> isize {
        a * b
    }
}

// The logic unit handles logical operations like AND, OR, and XOR, instead of arithmetic operations. It also performs numerical tests. For example, it checks whether the number is negative or not.
// It also controls if the output of the ALU is zero.
#[derive(Default)]
struct LogicUnit;

impl LogicUnit {
    #[inline]
    const fn is_zero(&self, a: isize) -> isize {
        (a == 0) as isize
    }

    #[inline]
    const fn is_non_zero(&self, a: isize) -> isize {
        (a != 0) as isize
    }

    #[inline]
    const fn less_than(&self, a: isize, b: isize) -> isize {
        (a < b) as isize
    }

    #[inline]
    const fn equals(&self, a: isize, b: isize) -> isize {
        (a == b) as isize
    }
}

impl Alu {
    pub(crate) fn execute(&self, op: AluOp, operand_a: isize, operand_b: isize) -> isize {
        let res = match op {
            AluOp::Add => self.arithmetic_unit.add(operand_a, operand_b),
            AluOp::Mul => self.arithmetic_unit.mul(operand_a, operand_b),
            AluOp::LessThanOutcome => self.logic_unit.less_than(operand_a, operand_b),
            AluOp::Equals => self.logic_unit.equals(operand_a, operand_b),
            AluOp::CmpZero => self.logic_unit.is_zero(operand_a),
            AluOp::CmpNotZero => self.logic_unit.is_non_zero(operand_a),
            AluOp::Pass => 0,
        };
        debug!("{op} {operand_a} {operand_b} => {res}");
        res
    }
}
