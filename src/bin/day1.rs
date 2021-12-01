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

use itertools::Itertools;

fn main() {
    let nums: std::vec::Vec<u32> = io::BufReader::new(io::stdin())
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse().unwrap())
        .collect();

    // Part 1
    println!(
        "{}",
        nums.iter().tuple_windows().filter(|(a, b)| a < b).count()
    );

    // Part 2
    println!(
        "{}",
        nums.iter()
            .tuple_windows()
            .map(|(x, y, z)| x + y + z)
            .tuple_windows()
            .filter(|(a, b)| a < b)
            .count()
    );
}
