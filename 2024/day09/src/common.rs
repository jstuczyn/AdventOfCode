// Copyright 2024 Jedrzej Stuczynski
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

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Sector {
    block: Block,
    size: usize,
}

impl Sector {
    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Copy, Clone, Debug)]
pub struct File {
    id: usize,
    sector: Sector,
}

#[derive(Copy, Clone, Debug)]
pub enum FileOrSpace {
    File(File),
    Space(Sector),
}

impl File {
    #[inline]
    pub fn file_checksum(&self) -> usize {
        if self.id == 0 {
            return 0;
        }
        let start_block = self.sector.block;
        let end_block = self.sector.block + self.sector.size;
        if start_block == 0 {
            self.id * (end_block * (end_block - 1)) / 2
        } else {
            let sum_to_end = ((end_block - 1) * (end_block)) / 2;
            let sum_to_start = ((start_block - 1) * start_block) / 2;
            self.id * (sum_to_end - sum_to_start)
        }
    }

    pub fn size(&self) -> usize {
        self.sector.size
    }

    pub fn try_assign(self, available_sector: Sector) -> (Self, Option<FileOrSpace>) {
        match self.size().cmp(&available_sector.size()) {
            // we filled space perfectly
            Ordering::Equal => (
                File {
                    id: self.id,
                    sector: available_sector,
                },
                None,
            ),
            // we have leftover file
            Ordering::Greater => (
                File {
                    id: self.id,
                    sector: available_sector,
                },
                Some(FileOrSpace::File(File {
                    id: self.id,
                    sector: Sector {
                        block: self.sector.block,
                        size: self.size() - available_sector.size(),
                    },
                })),
            ),
            // we have leftover space
            Ordering::Less => (
                File {
                    id: self.id,
                    sector: Sector {
                        block: available_sector.block,
                        size: self.size(),
                    },
                },
                Some(FileOrSpace::Space(Sector {
                    block: available_sector.block + self.sector.size,
                    size: available_sector.size() - self.size(),
                })),
            ),
        }
    }
}

type Block = usize;

#[derive(Clone)]
pub struct DiskMap {
    files: BTreeMap<Block, File>,
    free_spaces: BTreeMap<Block, Sector>,
}

impl FromStr for DiskMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut block = 0;
        let mut files = BTreeMap::new();
        let mut free_spaces = BTreeMap::new();

        for (i, digit) in s.chars().enumerate() {
            // SAFETY: the AOC input consists of only digits
            let size: usize = digit as usize - '0' as usize;
            let sector = Sector { block, size };
            if i % 2 == 0 {
                files.insert(block, File { id, sector });
                id += 1;
            } else if size != 0 {
                free_spaces.insert(block, sector);
            }
            block += size;
        }
        Ok(Self { files, free_spaces })
    }
}

impl DiskMap {
    #[allow(clippy::unwrap_used)]
    fn move_last_file_with_split(&mut self) {
        // SAFETY: during defragmentation procedure we have at least one file at all times...
        let file = self.files.pop_last().unwrap().1;

        // SAFETY: 'move_last_file' is not called if we don't have any free spaces left
        let free_space = self.free_spaces.pop_first().unwrap().1;

        let (updated, leftover) = file.try_assign(free_space);
        self.files.insert(updated.sector.block, updated);

        match leftover {
            Some(FileOrSpace::File(file)) => {
                self.files.insert(file.sector.block, file);
            }
            Some(FileOrSpace::Space(sector)) => {
                self.free_spaces.insert(sector.block, sector);
            }
            None => {}
        }
    }

    #[allow(clippy::unwrap_used)]
    fn is_fragmented(&self) -> bool {
        if let Some((first_empty_block, _)) = self.free_spaces.first_key_value() {
            // SAFETY: we have non-empty file map
            let last_file_block = self.files.last_key_value().unwrap().0;

            // first empty block is BEFORE final file
            first_empty_block < last_file_block
        } else {
            false
        }
    }

    pub fn defragment_blocks(&mut self) {
        while self.is_fragmented() {
            self.move_last_file_with_split();
        }
        self.free_spaces = BTreeMap::new();
    }

    #[allow(clippy::unwrap_used)]
    fn try_move_file(&mut self, file_position: Block) {
        // SAFETY: this file must exist otherwise the method wouldn't have been called
        let file = self.files.remove(&file_position).unwrap();

        // try to find a spot for the file
        if let Some(space) = self
            .free_spaces
            .values()
            .find(|s| s.size() >= file.size() && s.block < file_position)
        {
            let space_block = space.block;
            let space_removed = self.free_spaces.remove(&space_block).unwrap();
            self.files.insert(
                space_block,
                File {
                    id: file.id,
                    sector: Sector {
                        block: space_block,
                        size: file.size(),
                    },
                },
            );
            if space_removed.size() > file.size() {
                self.free_spaces.insert(
                    space_removed.block + file.size(),
                    Sector {
                        block: space_removed.block + file.sector.size,
                        size: space_removed.size() - file.sector.size,
                    },
                );
            }
        } else {
            // no appropriate space - put the file back in the original location
            self.files.insert(file_position, file);
        }
    }

    pub fn defragment_files(&mut self) {
        let file_blocks_to_move = self.files.keys().rev().copied().collect::<Vec<_>>();
        for file_position in file_blocks_to_move {
            self.try_move_file(file_position);
        }
    }

    pub fn checksum(&self) -> usize {
        self.files.values().map(|file| file.file_checksum()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_calculation() {
        fn add_file(files: &mut BTreeMap<Block, File>, id: usize, size: usize) {
            let block = files
                .last_key_value()
                .map(|(_, f)| f.sector.block + f.sector.size)
                .unwrap_or_default();
            files.insert(
                block,
                File {
                    id,
                    sector: Sector { block, size },
                },
            );
        }

        // 0099811188827773336446555566
        let mut files = BTreeMap::new();
        add_file(&mut files, 0, 2);
        add_file(&mut files, 9, 2);
        add_file(&mut files, 8, 1);
        add_file(&mut files, 1, 3);
        add_file(&mut files, 8, 3);
        add_file(&mut files, 2, 1);
        add_file(&mut files, 7, 3);
        add_file(&mut files, 3, 3);
        add_file(&mut files, 6, 1);
        add_file(&mut files, 4, 2);
        add_file(&mut files, 6, 1);
        add_file(&mut files, 5, 4);
        add_file(&mut files, 6, 2);

        let disk_map = DiskMap {
            files,
            free_spaces: BTreeMap::new(),
        };
        assert_eq!(1928, disk_map.checksum());
    }
}
