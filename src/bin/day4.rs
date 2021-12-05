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
use std::string::ToString;
use std::vec::Vec;

#[derive(Clone, Copy, Debug)]
struct Board {
    cells: [u8; 25],
    indices: [Option<u8>; 100],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Victory {
    round: usize,
    score: usize,
}

impl Board {
    fn read(nums: impl IntoIterator<Item = u8>) -> Board {
        let mut nums = nums.into_iter();
        let mut cells = [0; 25];
        let mut indices = [None; 100];
        for (i, c) in cells.iter_mut().enumerate() {
            let n = nums.next().unwrap();
            *c = n;
            indices[n as usize] = Some(i as u8);
        }

        Board { cells, indices }
    }

    fn check(&self, draws: impl IntoIterator<Item = u8>) -> Option<Victory> {
        let mut rows: [u8; 5] = [0; 5];
        let mut cols: [u8; 5] = [0; 5];
        for (s, d) in draws.into_iter().enumerate() {
            let i = match self.indices[d as usize] {
                Some(i) => i,
                None => continue,
            };
            let r = i / 5;
            let c = i % 5;
            rows[r as usize] |= 1 << c;
            cols[c as usize] |= 1 << r;

            if rows[r as usize] == 0b11111 || cols[c as usize] == 0b11111 {
                let mut unmarked_sum: usize = 0;
                for r in 0..5 {
                    for c in 0..5 {
                        if 0 == rows[r] & (1 << c) {
                            unmarked_sum += self.cells[r * 5 + c] as usize;
                        }
                    }
                }
                return Some(Victory {
                    round: s,
                    score: d as usize * unmarked_sum,
                });
            }
        }

        None
    }
}

fn read_input() -> (Vec<u8>, Vec<Board>) {
    let mut lines = io::BufReader::new(io::stdin()).lines().map(Result::unwrap);
    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut board_nums = lines
        .map(|s| {
            s.split_whitespace()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .into_iter()
        })
        .flatten()
        .map(|s| s.parse().unwrap())
        .peekable();
    let mut boards = Vec::new();
    while board_nums.peek().is_some() {
        boards.push(Board::read(&mut board_nums));
    }
    (draws, boards)
}

fn part1(draws: &[u8], boards: &[Board]) -> usize {
    boards
        .iter()
        .filter_map(|b| b.check(draws.iter().copied()))
        .min()
        .unwrap()
        .score
}

fn part2(draws: &[u8], boards: &[Board]) -> usize {
    boards
        .iter()
        .filter_map(|b| b.check(draws.iter().copied()))
        .max()
        .unwrap()
        .score
}

fn main() {
    let (draws, boards) = read_input();
    println!("{}", part1(&draws, &boards));
    println!("{}", part2(&draws, &boards));
}
