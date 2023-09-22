// Copyright 2022-2023 Jedrzej Stuczynski
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

use std::fs;
use std::io;
use std::path::Path;

pub fn read_input<P, T, F>(path: P, parse: F) -> io::Result<T>
where
    P: AsRef<Path>,
    F: Fn(&str) -> anyhow::Result<T>,
{
    let raw = fs::read_to_string(path)?;
    parse(&raw).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("input could not be parsed into desired type - {err}"),
        )
    })
}
