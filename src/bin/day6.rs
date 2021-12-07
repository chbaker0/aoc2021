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

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter::{self, IntoIterator, Iterator};
use std::vec::Vec;

fn part1(times: impl IntoIterator<Item = u8>) -> usize {
    let mut counts = [0; 9];
    for t in times.into_iter() {
        assert!(t < 9);
        counts[t as usize] += 1;
    }

    for _day in 0..80 {
        let mut new_counts = [0; 9];
        // Spawn new fish
        new_counts[8] = counts[0];
        new_counts[6] = counts[0];

        // Decrement other timers
        for i in 0..8 {
            new_counts[i] += counts[i + 1];
        }

        counts = new_counts;
    }

    counts.iter().sum()
}

fn main() {
    let input = io::BufReader::new(io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let times: Vec<u8> = input.split(',').map(|s| s.parse().unwrap()).collect();
    println!("{}", part1(times.iter().copied()));
}
