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

struct Heightmap<'a> {
    heights: &'a [u8],
    rows: usize,
    cols: usize,
}

impl<'a> Heightmap<'a> {
    fn at(&self, r: isize, c: isize) -> Option<u8> {
        if r < 0 || c < 0 {
            return None;
        }

        let r = r as usize;
        let c = c as usize;
        if r < self.rows && c < self.cols {
            Some(self.heights[r * self.cols + c])
        } else {
            None
        }
    }

    fn neighbors(&self, r: usize, c: usize) -> impl Iterator<Item = u8> {
        let adj = [
            self.at(r as isize - 1, c as isize),
            self.at(r as isize + 1, c as isize),
            self.at(r as isize, c as isize - 1),
            self.at(r as isize, c as isize + 1),
        ];

        adj.into_iter().flatten()
    }

    fn risk(&self, r: usize, c: usize) -> Option<u8> {
        let h = self.at(r as isize, c as isize).unwrap();
        for n in self.neighbors(r, c) {
            if n < h {
                return None;
            }
        }
        Some(1 + h)
    }
}

fn part1(map: &Heightmap) -> u64 {
    let mut sum = 0;
    for r in 0..map.rows {
        for c in 0..map.cols {
            sum += map.risk(r, c).unwrap_or(0) as u64;
            print!("{} ", sum);
        }
    }
    sum
}

fn main() {
    let lines: Vec<String> = io::BufReader::new(io::stdin())
        .lines()
        .map(|r| r.unwrap())
        .collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut heights = Vec::new();
    for r in 0..rows {
        let l = &lines[r];
        for c in l.chars() {
            heights.push(c.to_digit(10).unwrap() as u8);
        }
    }
    assert_eq!(rows * cols, heights.len());

    let heightmap = Heightmap {
        heights: &heights,
        rows,
        cols,
    };

    println!("{}", part1(&heightmap));
}
