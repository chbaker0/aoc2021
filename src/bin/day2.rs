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
use std::string::String;

#[derive(Clone, Copy, Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    depth: i32,
    horizontal: i32,
}

fn parse_command(cmd: &str) -> Command {
    let (dir, off_s) = cmd.split_once(' ').unwrap();
    let off = off_s.parse().unwrap();
    assert!(off > 0);
    match dir {
        "forward" => Command::Forward(off),
        "down" => Command::Down(off),
        "up" => Command::Up(off),
        _ => panic!("invalid input!"),
    }
}

fn apply_command(mut pos: Pos, cmd: Command) -> Pos {
    match cmd {
        Command::Forward(off) => pos.horizontal += off,
        Command::Down(off) => pos.depth += off,
        Command::Up(off) => pos.depth -= off,
    }
    pos
}

fn part1<'a>(lines: impl IntoIterator<Item = &'a str>) -> i32 {
    let pos = lines.into_iter().map(parse_command).fold(
        Pos {
            depth: 0,
            horizontal: 0,
        },
        apply_command,
    );
    pos.depth * pos.horizontal
}

#[derive(Clone, Copy, Debug)]
struct State {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

fn apply_command_2(mut state: State, cmd: Command) -> State {
    match cmd {
        Command::Forward(off) => {
            state.horizontal += off;
            state.depth += off * state.aim;
        }
        Command::Down(off) => state.aim += off,
        Command::Up(off) => state.aim -= off,
    }
    state
}

fn part2<'a>(lines: impl IntoIterator<Item = &'a str>) -> i32 {
    let pos = lines.into_iter().map(parse_command).fold(
        State {
            depth: 0,
            horizontal: 0,
            aim: 0,
        },
        apply_command_2,
    );
    pos.depth * pos.horizontal
}

fn main() {
    let lines: Vec<String> = io::BufReader::new(io::stdin())
        .lines()
        .map(Result::unwrap)
        .collect();
    println!("{}", part1(lines.iter().map(String::as_str)));
    println!("{}", part2(lines.iter().map(String::as_str)));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: [&'static str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(150, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2(INPUT));
    }
}
