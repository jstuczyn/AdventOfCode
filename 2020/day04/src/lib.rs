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

// legacy code
#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use crate::passport::{Passport, RawPassport};
use aoc_solution::Aoc;

mod passport;

#[derive(Aoc)]
pub struct Day04;

pub fn part1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(RawPassport::try_from)
        .filter(Result::is_ok)
        .count()
}

pub fn part2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(RawPassport::try_from)
        .filter(Result::is_ok)
        .map(|raw_pass| Passport::try_from(raw_pass.unwrap()))
        .filter(Result::is_ok)
        .filter(|pass| pass.as_ref().unwrap().validate())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
                .to_string(),
            "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"
                .to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"
            .to_string(),
        ];

        let expected = 2;

        assert_eq!(expected, part1(input))
    }

    #[test]
    fn part2_sample_input() {
        let input = vec![
            "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"
                .to_string(),
            "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946"
                .to_string(),
            "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"
                .to_string(),
            "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
                .to_string(),
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
                .to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"
                .to_string(),
            "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"
                .to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
            .to_string(),
        ];

        let expected = 4;

        assert_eq!(expected, part2(input))
    }
}
