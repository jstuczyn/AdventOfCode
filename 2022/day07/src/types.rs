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

use anyhow::{anyhow, bail};
use aoc_common::parsing::parse_input_lines;
use std::str::FromStr;

const ROOT: &str = "/";
const PARENT_DIR: &str = "..";
const CMD: &str = "$";
const DIR: &str = "dir";
const CD: &str = "cd";
const LS: &str = "ls";

#[derive(Debug, Clone)]
pub enum TerminalOutput {
    Command { cmd: Command },
    FileInfo { details: FileSystemEntry },
}

impl TerminalOutput {
    fn is_root_cd(&self) -> bool {
        match self {
            TerminalOutput::Command {
                cmd: Command::ChangeDir { arg },
            } => arg == ROOT,
            _ => false,
        }
    }
}

impl FromStr for TerminalOutput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(CMD) {
            Ok(TerminalOutput::Command { cmd: s.parse()? })
        } else {
            Ok(TerminalOutput::FileInfo {
                details: s.parse()?,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    ChangeDir { arg: String },
    List,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(CMD).map(|s| s.trim()) {
            Some(raw_cmd) => {
                if let Some(cd_arg) = raw_cmd.strip_prefix(CD) {
                    Ok(Command::ChangeDir {
                        arg: cd_arg.trim().to_string(),
                    })
                } else if raw_cmd == LS {
                    Ok(Command::List)
                } else {
                    bail!("{s} is not a valid command!")
                }
            }
            None => bail!("{s} is not a valid command!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    // use arena tree here
    arena: Vec<FileSystemNode>,
}

impl FromStr for FileSystem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terminal_output: Vec<TerminalOutput> = parse_input_lines(s)?;
        terminal_output.try_into()
    }
}

impl TryFrom<Vec<TerminalOutput>> for FileSystem {
    type Error = anyhow::Error;

    fn try_from(value: Vec<TerminalOutput>) -> Result<Self, Self::Error> {
        let mut fresh_fs = FileSystem::new();
        fresh_fs.rebuild_from_terminal_output(value)?;
        Ok(fresh_fs)
    }
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            arena: vec![FileSystemNode::root()],
        }
    }

    pub fn sum_dirs_with_max_size(&self, max_size: usize) -> usize {
        let mut lookup = vec![None; self.arena.len()];
        let mut total_size = 0;
        for node in &self.arena {
            if node.is_directory() {
                let size = self.node_size(node.idx, &mut lookup);
                if size <= max_size {
                    total_size += size
                }
            }
        }
        total_size
    }

    pub fn delete_smallest_needed_dir(
        &self,
        total_disk_space: usize,
        required_unused_space: usize,
    ) -> usize {
        let mut lookup = vec![None; self.arena.len()];
        let occupied = self.node_size(0, &mut lookup);
        let available = total_disk_space - occupied;
        let needed_extra = required_unused_space - available;

        let mut smallest = usize::MAX;
        for node in &self.arena {
            if node.is_directory() {
                let size = self.node_size(node.idx, &mut lookup);
                if size >= needed_extra && size < smallest {
                    smallest = size
                }
            }
        }
        smallest
    }

    fn rebuild_from_terminal_output(
        &mut self,
        output: Vec<TerminalOutput>,
    ) -> Result<(), anyhow::Error> {
        let mut current_dir_idx = self.arena[0].idx;

        if !output[0].is_root_cd() {
            bail!("the first terminal output was not 'cd /'!")
        }

        // the first output is to move into the root dir
        for output in output.into_iter().skip(1) {
            match output {
                TerminalOutput::Command { cmd } => match cmd {
                    Command::ChangeDir { arg } => {
                        if arg == PARENT_DIR {
                            current_dir_idx = match self.arena[current_dir_idx].parent {
                                Some(parent) => parent,
                                None => bail!(
                                    "{:?} doesn't have a parent!",
                                    self.arena[current_dir_idx]
                                ),
                            }
                        } else {
                            current_dir_idx = self.child_dir_id(current_dir_idx, &arg)?;
                        }
                    }
                    _ => continue,
                },
                TerminalOutput::FileInfo { details } => self.add_fs_entry(current_dir_idx, details),
            }
        }

        Ok(())
    }

    fn add_fs_entry(&mut self, parent_idx: usize, info: FileSystemEntry) {
        let next_id = self.arena.len();
        self.arena[parent_idx].children.push(next_id);
        self.arena
            .push(FileSystemNode::new(next_id, info, parent_idx))
    }

    fn child_dir_id(
        &mut self,
        parent_idx: usize,
        child_name: &str,
    ) -> Result<usize, anyhow::Error> {
        for child_id in &self.arena[parent_idx].children {
            if let FileSystemEntry::Directory { directory } = &self.arena[*child_id].entry {
                if directory.name == child_name {
                    return Ok(*child_id);
                }
            }
        }
        bail!("child {child_name} doesn't exist for parent {parent_idx}!")
    }

    fn node_size(&self, node_idx: usize, lookup: &mut [Option<usize>]) -> usize {
        if let Some(known) = lookup[node_idx] {
            return known;
        }
        let node = &self.arena[node_idx];
        let size = node.entry.size()
            + node
                .children
                .iter()
                .map(|child| self.node_size(*child, lookup))
                .sum::<usize>();
        lookup[node_idx] = Some(size);
        size
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        FileSystem::new()
    }
}

#[derive(Debug, Clone)]
pub struct FileSystemNode {
    idx: usize,
    entry: FileSystemEntry,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl FileSystemNode {
    fn new(idx: usize, entry: FileSystemEntry, parent: usize) -> Self {
        FileSystemNode {
            idx,
            entry,
            parent: Some(parent),
            children: vec![],
        }
    }

    fn root() -> Self {
        FileSystemNode {
            idx: 0,
            entry: FileSystemEntry::Directory {
                directory: Directory {
                    name: ROOT.to_string(),
                },
            },
            parent: None,
            children: vec![],
        }
    }

    fn is_directory(&self) -> bool {
        self.entry.is_directory()
    }
}

#[derive(Debug, Clone)]
pub enum FileSystemEntry {
    Directory { directory: Directory },
    File { file: File },
}

impl FileSystemEntry {
    fn is_directory(&self) -> bool {
        matches!(self, FileSystemEntry::Directory { .. })
    }
}

impl FromStr for FileSystemEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with(DIR) {
            Ok(FileSystemEntry::Directory {
                directory: s.parse()?,
            })
        } else {
            Ok(FileSystemEntry::File { file: s.parse()? })
        }
    }
}

impl FileSystemEntry {
    fn size(&self) -> usize {
        match self {
            FileSystemEntry::Directory { .. } => 0,
            FileSystemEntry::File { file } => file.size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    _name: String,
    size: usize,
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut size_name = s.split_ascii_whitespace();

        let size = size_name
            .next()
            .ok_or_else(|| anyhow!("input did not contain the file size"))?
            .parse()?;

        let name = size_name
            .next()
            .ok_or_else(|| anyhow!("input did not contain the file name"))?
            .to_string();

        Ok(File { _name: name, size })
    }
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
}

impl FromStr for Directory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(DIR) {
            Some(name) => Ok(Directory {
                name: name.trim().to_string(),
            }),
            None => bail!("{s} is not a valid directory!"),
        }
    }
}
