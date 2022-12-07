// Copyright 2022 Jedrzej Stuczynski
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

use crate::types::FileSystem;

pub fn part1(input: FileSystem) -> usize {
    input.sum_count_dirs_with_max_size(100000)
}

pub fn part2(input: FileSystem) -> usize {
    input.delete_smallest_needed_dir(70000000, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FileSystem;

    fn sample_input() -> FileSystem {
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 95437;
        assert_eq!(expected, part1(sample_input()))
    }

    #[test]
    fn part2_sample_input() {
        let expected = 24933642;
        assert_eq!(expected, part2(sample_input()))
    }
}
