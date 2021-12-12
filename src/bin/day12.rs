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

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::iter::Iterator;

use multimap::MultiMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Cave(String);

impl Cave {
    fn is_small(&self) -> bool {
        self.0.chars().next().unwrap().is_lowercase()
    }
}

fn read_graph<'a>(lines: impl Iterator<Item = String>) -> MultiMap<Cave, Cave> {
    lines
        .flat_map(|l| {
            let mut iter = l.split("-");
            let c1 = Cave(iter.next().unwrap().to_string());
            let c2 = Cave(iter.next().unwrap().to_string());
            [(c1.clone(), c2.clone()), (c2, c1)]
        })
        .collect()
}

fn dfs_count(
    map: &MultiMap<Cave, Cave>,
    cur: Cave,
    visited: &mut HashMap<Cave, usize>,
    mut can_visit_twice: bool,
) -> usize {
    if cur.0 == "end" {
        return 1;
    }

    let cnt = visited.entry(cur.clone()).or_insert(0);
    if cur.is_small() && *cnt == 1 {
        if cur.0 != "start" && can_visit_twice {
            can_visit_twice = false;
        } else {
            return 0;
        }
    } else if cur.is_small() && *cnt == 2 {
        return 0;
    }

    *cnt += 1;

    let result = map
        .get_vec(&cur)
        .unwrap()
        .iter()
        .map(|dst| dfs_count(map, dst.clone(), visited, can_visit_twice))
        .sum();
    *visited.get_mut(&cur).unwrap() -= 1;
    result
}

fn part1(map: &MultiMap<Cave, Cave>) -> usize {
    let mut visited: HashMap<Cave, usize> = HashMap::new();
    dfs_count(map, Cave("start".to_string()), &mut visited, false)
}

fn part2(map: &MultiMap<Cave, Cave>) -> usize {
    let mut visited: HashMap<Cave, usize> = HashMap::new();
    dfs_count(map, Cave("start".to_string()), &mut visited, true)
}

fn main() {
    let map = read_graph(io::BufReader::new(io::stdin()).lines().map(|r| r.unwrap()));
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
