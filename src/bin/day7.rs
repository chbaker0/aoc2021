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
use std::vec::Vec;

const MAX_POSITION: usize = 2000;

fn calc(positions: &[u16], inc: bool) -> u64 {
    let mut costs = [0u64; MAX_POSITION];
    for pos in positions.iter() {
        let pos = *pos as usize;
        assert!(pos < MAX_POSITION, "{} is too large", pos);
        for (dist, target) in (pos..MAX_POSITION)
            .enumerate()
            .chain((0..pos + 1).rev().enumerate())
        {
            let dist = dist as u64;
            if inc {
                costs[target] += dist * (dist + 1) / 2;
            } else {
                costs[target] += dist;
            }
        }
    }

    let (min_pos, min_cost) = costs
        .iter()
        .copied()
        .enumerate()
        .min_by_key(|(_p, c)| *c)
        .unwrap();
    println!("min_pos = {}", min_pos);
    min_cost
}

fn main() {
    let input = io::BufReader::new(io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let positions: Vec<u16> = input.split(',').map(|s| s.parse().unwrap()).collect();
    println!("{}", calc(&positions, false));
    println!("{}", calc(&positions, true));
}
