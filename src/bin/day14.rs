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
use std::str::FromStr;

use itertools::Itertools;

struct RuleMap {
    map: HashMap<(char, char), char>,
}

impl RuleMap {
    fn new() -> RuleMap {
        RuleMap {
            map: HashMap::new()
        }
    }

    fn add_rule_str(&mut self, s: &str) {
        let mut iter = s.split_whitespace();
        let mut elem_iter = iter.next().unwrap().chars();
        let elem1 = elem_iter.next().unwrap();
        let elem2 = elem_iter.next().unwrap();
        assert_eq!(iter.next().unwrap(), "->");
        let ins = iter.next().unwrap().chars().next().unwrap();
        self.map.insert((elem1, elem2), ins);
    }

    fn apply(&self, s: &str) -> String {
        let mut result = String::new();
        result.push(s.chars().next().unwrap());
        for (e1, e2) in s.chars().tuple_windows() {
            if let Some(ins) = self.map.get(&(e1, e2)) {
                result.push(*ins);
            }
            result.push(e2);
        }
        result
    }

    fn apply_counts(&self, paircounts: &mut HashMap<(char, char), usize>) {
        let mut newcounts = HashMap::<(char, char), usize>::new();
        for ((c1, c2), n) in paircounts.iter() {
            if let Some(ins) = self.map.get(&(*c1, *c2)) {
                newcounts.insert((*c1, *ins), *n);
                newcounts.insert((*ins, *c2), *n);
            } else {
                newcounts.insert((*c1, *c2), *n);
            }
        }
        *paircounts = newcounts;
    }
}

fn map_add<K: Eq + Hash>(map: &mut HashMap<K, usize>, other: &HashMap<K, usize>) {
    for (k, v) in other.iter() {
        *map.entry(k).or_insert(0) += v;
    }
}

fn part1(rules: &RuleMap, template: &str) -> usize {
    let mut polymer = template.to_string();
    for _i in 0..10 {
        polymer = rules.apply(&polymer);
    }
    let counts = polymer.chars().counts();
    if let itertools::MinMaxResult::MinMax((_, lce), (_, mce)) = counts.iter().minmax_by_key(|(_c, e)| *e) {
        mce - lce
    } else {
        panic!();
    }
}

fn run(rules: &RuleMap, template: &str, iters: usize) -> HashMap<char, usize> {
    match template.len() {
        0 => return HashMap::new(),
        1 => return HashMap::from([template.chars().next().unwrap(), 1]),
        _ => (),
    };

    let polymer: String = template.chars().take(2).collect();
    let mut counts = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(&c).or_insert(0) += 1;
    }
    for (e1, e2) in template.chars().tuple_windows() {
        *counts.entry((e1, e2)).or_insert(0) += 1;
    }
    for _i in 0..iters {
        rules.apply_counts(&mut counts);
    }

    let mut elem_counts = HashMap::new() {

    }
}

fn main() {
    let mut lines = io::BufReader::new(io::stdin())
    .lines()
    .map(|r| r.unwrap());
    let template = lines.next().unwrap();
    let _blank = lines.next().unwrap();
    let rules = lines.fold(RuleMap::new(), |mut m, s| { m.add_rule_str(&s); m });
    println!("{}", part1(&rules, &template));
}
