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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(desc: &str) -> Point {
        let mut iter = desc.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn parse(desc: &str) -> Line {
        let mut iter = desc.split(' ');
        let p1 = Point::parse(iter.next().unwrap());
        assert_eq!(iter.next().unwrap(), "->");
        let p2 = Point::parse(iter.next().unwrap());
        Line { p1, p2 }
    }

    fn is_not_diagonal(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let dir = |a: i32, b: i32| match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let inc: (i32, i32) = (dir(self.p1.x, self.p2.x), dir(self.p1.y, self.p2.y));

        let add = |p: Point, (u, v)| Point {
            x: p.x + u,
            y: p.y + v,
        };
        let mut p = self.p1;
        let p_last = add(self.p2, inc);
        iter::from_fn(move || {
            if p == p_last {
                return None;
            }
            let cur = p;
            p = add(p, inc);
            Some(cur)
        })
    }
}

fn part1(lines: impl IntoIterator<Item = Line>) -> usize {
    let mut overlaps = HashMap::<Point, u32>::new();
    for l in lines.into_iter().filter(Line::is_not_diagonal) {
        for p in l.points() {
            let e = overlaps.entry(p).or_insert(0);
            *e += 1;
        }
    }
    overlaps.values().filter(|e| **e >= 2).count()
}

fn part2(lines: impl IntoIterator<Item = Line>) -> usize {
    let mut overlaps = HashMap::<Point, u32>::new();
    for l in lines.into_iter() {
        for p in l.points() {
            let e = overlaps.entry(p).or_insert(0);
            *e += 1;
        }
    }
    overlaps.values().filter(|e| **e >= 2).count()
}

fn main() {
    let lines: Vec<Line> = io::BufReader::new(io::stdin())
        .lines()
        .map(|l| Line::parse(&l.unwrap()))
        .collect();
    println!("{}", part1(lines.iter().copied()));
    println!("{}", part2(lines.iter().copied()));
}
