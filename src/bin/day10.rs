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
use std::vec::Vec;

fn is_opening(c: char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn illegal_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

fn dangling_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!(),
    }
}

fn closing(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}

enum Score {
    Corrupted(u64),
    Incomplete(u64),
}

use Score::*;

fn score(line: &str) -> Score {
    let mut corrupt_score = 0;
    let mut stack = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(closing(c));
            continue;
        }

        if c != stack.pop().unwrap() {
            corrupt_score += illegal_score(c);
        }
    }

    if corrupt_score > 0 {
        return Corrupted(corrupt_score);
    }

    let mut incomplete_score = 0;
    for c in stack.iter().rev() {
        incomplete_score *= 5;
        incomplete_score += dangling_score(*c);
    }

    Incomplete(incomplete_score)
}

fn part1<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    lines
        .into_iter()
        .map(|s| match score(s) {
            Corrupted(s) => s,
            _ => 0,
        })
        .sum()
}

fn part2<'a>(lines: impl Iterator<Item = &'a str>) -> u64 {
    let mut scores: Vec<u64> = lines
        .into_iter()
        .filter_map(|s| match score(s) {
            Incomplete(s) => Some(s),
            _ => None,
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let lines: Vec<String> = io::BufReader::new(io::stdin())
        .lines()
        .map(|r| r.unwrap())
        .collect();
    println!("{}", part1(lines.iter().map(|s| s.as_ref())));
    println!("{}", part2(lines.iter().map(|s| s.as_ref())));
}
