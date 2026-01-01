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

#[cfg(test)]
mod inputs {
    use crate::intcode::IntCodeMachine;
    use crate::memory::Ram;
    use aoc_common::helpers::setup_logger;

    #[test]
    fn sample_d2_input() -> anyhow::Result<()> {
        unsafe {
            ::std::env::set_var("RUST_LOG", "trace");
        }
        setup_logger();

        let mut intcode =
            IntCodeMachine::<Ram>::load(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50].into());
        intcode.run()?;
        let memory = intcode.dump_memory();
        assert_eq!(memory[0], 3500);

        Ok(())
    }
}
