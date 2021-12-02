// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io;
use std::io::prelude::*;
use std::iter::{IntoIterator, Iterator};

use itertools::Itertools;

fn part1(nums: impl IntoIterator<Item = u32>) -> usize {
    nums.into_iter()
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn part2(nums: impl IntoIterator<Item = u32>) -> usize {
    nums.into_iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count()
}

fn main() {
    let nums: std::vec::Vec<u32> = io::BufReader::new(io::stdin())
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", part1(nums.iter().copied()));
    println!("{}", part2(nums.iter().copied()));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, part2(INPUT));
    }
}
