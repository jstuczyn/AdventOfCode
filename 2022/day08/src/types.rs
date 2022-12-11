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

use anyhow::anyhow;
use common::parsing::parse_input_lines;
use std::str::FromStr;

type Height = usize;

#[derive(Debug, Clone)]
pub struct Forest {
    // yes, we're keeping duplicate information, but it's purely read-only
    rows: Vec<Vec<Height>>,
    columns: Vec<Vec<Height>>,
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        struct TreeRow(Vec<Height>);

        impl FromStr for TreeRow {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let row = s
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| d as usize)
                            .ok_or_else(|| anyhow!("{c} cannot be converted to digit in base 10"))
                    })
                    .collect::<Result<_, _>>()?;
                Ok(TreeRow(row))
            }
        }

        let parsed_rows: Vec<TreeRow> = parse_input_lines(s)?;
        let rows = parsed_rows.into_iter().map(|r| r.0).collect::<Vec<_>>();
        let num_columns = rows[0].len();

        let columns = (0..num_columns)
            .map(|i| rows.iter().map(|row| row[i]).collect())
            .collect::<Vec<_>>();

        Ok(Forest { rows, columns })
    }
}

impl Forest {
    pub fn count_visible_trees(&self) -> usize {
        let rows = self.rows.len();
        let columns = self.columns.len();

        (0..columns)
            .flat_map(|x| (0..rows).map(move |y| self.is_visible(x, y)))
            .filter(|v| *v)
            .count()
    }

    pub fn highest_scenic_score(&self) -> usize {
        let rows = self.rows.len();
        let columns = self.columns.len();

        (0..columns)
            .flat_map(|x| (0..rows).map(move |y| self.scenic_score(x, y)))
            .max()
            .unwrap_or_default()
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.rows[y][x];

        // check left part of the row
        let mut left = self.rows[y]
            .iter()
            .take(x)
            .rev()
            .take_while(|height| height < &&tree_height)
            .count();
        // adjust for stopping due to hitting tree rather than an edge
        if x != left {
            left += 1;
        }

        // check right part of the row
        let mut right = self.rows[y]
            .iter()
            .skip(x + 1)
            .take_while(|height| height < &&tree_height)
            .count();
        // adjust for stopping due to hitting tree rather than an edge
        if x + right < self.columns.len() - 1 {
            right += 1
        }

        // check upper part of the column
        let mut up = self.columns[x]
            .iter()
            .take(y)
            .rev()
            .take_while(|height| height < &&tree_height)
            .count();
        // adjust for stopping due to hitting tree rather than an edge
        if y != up {
            up += 1;
        }

        // check lower part of the column
        let mut down = self.columns[x]
            .iter()
            .skip(y + 1)
            .take_while(|height| height < &&tree_height)
            .count();
        // adjust for stopping due to hitting tree rather than an edge
        if y + down < self.rows.len() - 1 {
            down += 1
        }

        left * right * up * down
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.rows[y][x];

        // edge tree
        if x == 0 || y == 0 || x == self.columns.len() - 1 || y == self.rows.len() - 1 {
            return true;
        }

        // check left part of the row
        if self.rows[y]
            .iter()
            .take(x)
            .all(|height| height < &tree_height)
        {
            return true;
        }

        // check right part of the row
        if self.rows[y]
            .iter()
            .skip(x + 1)
            .all(|height| height < &tree_height)
        {
            return true;
        }

        // check upper part of the column
        if self.columns[x]
            .iter()
            .take(y)
            .all(|height| height < &tree_height)
        {
            return true;
        }

        // check lower part of the column
        if self.columns[x]
            .iter()
            .skip(y + 1)
            .all(|height| height < &tree_height)
        {
            return true;
        }

        false
    }
}
