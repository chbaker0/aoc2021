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
use std::iter::Iterator;
use std::string::String;

fn parse(num: &str) -> Vec<bool> {
    let mut res = Vec::new();
    for d in num.chars() {
        res.push(match d {
            '0' => false,
            '1' => true,
            _ => panic!(),
        });
    }
    res
}

fn to_int(num: &[bool]) -> u32 {
    let mut i = 0;
    for d in num.iter() {
        i = i << 1;
        if *d {
            i += 1;
        }
    }
    i
}

fn count_bit(nums: &[Vec<bool>], pos: usize) -> Option<usize> {
    let mut count = 0;
    for n in nums {
        if pos >= n.len() {
            return None;
        }
        if n[pos] {
            count += 1;
        }
    }
    Some(count)
}

fn gamma_epsilon(nums: &[Vec<bool>]) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0.. {
        gamma = gamma << 1;
        epsilon = epsilon << 1;
        if let Some(count) = count_bit(nums, pos) {
            if count > nums.len() / 2 {
                gamma += 1;
            } else {
                epsilon += 1;
            }
        } else {
            break;
        }
    }
    (gamma >> 1, epsilon >> 1)
}

fn part1(nums: &[String]) -> u32 {
    let nums: Vec<_> = nums.iter().map(|s| parse(&s)).collect();
    let (gamma, epsilon) = gamma_epsilon(&nums);
    gamma * epsilon
}

fn rating(mut nums: Vec<Vec<bool>>, criteria: bool) -> u32 {
    let mut pos = 0;
    while let Some(count) = count_bit(&nums, pos) {
        if nums.len() <= 1 {
            break;
        }
        let keep = criteria != ((count * 2) >= nums.len());
        nums.retain(|n| n[pos] == keep);
        pos += 1;
    }
    assert_eq!(1, nums.len());
    to_int(&nums[0])
}

fn part2(nums: &[String]) -> u32 {
    let nums: Vec<_> = nums.iter().map(|s| parse(&s)).collect();
    let o2 = rating(nums.clone(), true);
    let co2 = rating(nums, false);
    o2 * co2
}

fn main() {
    let lines: Vec<String> = io::BufReader::new(io::stdin())
        .lines()
        .map(Result::unwrap)
        .collect();
    println!("{}", part1(&lines));
    println!("{}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [&'static str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_part1() {
        let strs: Vec<_> = INPUT.iter().map(|s| String::from(*s)).collect();
        assert_eq!(198, part1(&strs));
    }

    #[test]
    fn test_part2() {
        let strs: Vec<_> = INPUT.iter().map(|s| String::from(*s)).collect();
        assert_eq!(230, part2(&strs));
    }
}
