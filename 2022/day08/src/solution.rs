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

use crate::types::Forest;

pub fn part1(input: Forest) -> usize {
    input.count_visible_trees()
}

pub fn part2(input: Forest) -> usize {
    input.highest_scenic_score()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Forest;

    fn sample_input() -> Forest {
        r#"30373
25512
65332
33549
35390"#
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample_input() {
        let expected = 21;
        assert_eq!(expected, part1(sample_input()))
    }
    
    #[test]
    fn part2_sample_input() {
        let expected = 8;
        assert_eq!(expected, part2(sample_input()))
    }
}
