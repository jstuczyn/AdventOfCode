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

use anyhow::Result;

// we need separate trait, i.e. we can't just use `FromStr`,
// because of all the custom rules for say `Vec<T>`
// TODO: or maybe we should just create wrapper containers instead?
pub trait AocInputParser {
    type Output;

    fn parse_input(raw: &str) -> Result<Self::Output>;
}

pub trait AocParseExt {
    fn parse_aoc_input<F: AocInputParser>(&self) -> Result<F::Output>;
}

impl AocParseExt for str {
    fn parse_aoc_input<F: AocInputParser>(&self) -> Result<F::Output> {
        <F as AocInputParser>::parse_input(self)
    }
}
