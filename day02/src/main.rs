#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(test)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]

use rayon::prelude::*;

use std::{cmp::Reverse, str::FromStr};

use itertools::Itertools;
use utils::{self, str_to};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day02/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day02/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse_input_part(&utils::file_to_string(file_name))
        .map(Move::set_round_outcome)
        .map(Move::player_score)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    parse_input_part(&utils::file_to_string(file_name))
        .map(Move::set_player_move)
        .map(|x| Move::player_score(x))
        .sum()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input_part<'a>(input: &'a String) -> impl Iterator<Item = Move> + 'a {
    input
        .split('\n')
        .filter_map(|line| Move::from_str(line).ok())
        .into_iter()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    opponent_move: MoveEnum,
    player_move: MoveEnum,
    player_outcome: RoundOutcome,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = line.trim().split(' ').collect_tuple() {
            println!("{:?} {:?}", first, second);
            Ok(Move {
                opponent_move: MoveEnum::from_str(first).unwrap(),
                player_move: MoveEnum::from_str(second).unwrap(),
                player_outcome: RoundOutcome::from_str(second).unwrap(),
            })
        } else {
            Err(line.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MoveEnum {
    ROCK,
    SCISSORS,
    PAPER,
}
use MoveEnum::*;

impl FromStr for MoveEnum {
    type Err = String;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let move_char = x.chars().next().unwrap();
        if move_char == 'A' || move_char == 'X' {
            Ok(ROCK)
        } else if move_char == 'B' || move_char == 'Y' {
            Ok(PAPER)
        } else if move_char == 'C' || move_char == 'Z' {
            Ok(SCISSORS)
        } else {
            Err(x.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RoundOutcome {
    DRAW,
    WIN,
    LOSS,
}

use RoundOutcome::*;

impl FromStr for RoundOutcome {
    type Err = String;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let move_char = x.chars().next().unwrap();
        if move_char == 'X' {
            Ok(LOSS)
        } else if move_char == 'Y' {
            Ok(DRAW)
        } else if move_char == 'Z' {
            Ok(WIN)
        } else {
            Err(x.to_string())
        }
    }
}

impl Move {
    fn last_player_wins_p(&self) -> bool {
        match self.player_move {
            ROCK => self.opponent_move == SCISSORS,
            SCISSORS => self.opponent_move == PAPER,
            PAPER => self.opponent_move == ROCK,
        }
    }

    fn set_round_outcome(mut self) -> Self {
        self.player_outcome = {
            if self.opponent_move == self.player_move {
                DRAW
            } else if self.last_player_wins_p() {
                WIN
            } else {
                LOSS
            }
        };
        self
    }

    fn set_player_move(mut self) -> Self {
        self.player_move = match (self.player_outcome, self.opponent_move) {
            (DRAW, _) => self.opponent_move,
            (WIN, ROCK) => PAPER,
            (WIN, SCISSORS) => ROCK,
            (WIN, PAPER) => SCISSORS,
            (LOSS, ROCK) => SCISSORS,
            (LOSS, SCISSORS) => PAPER,
            (LOSS, PAPER) => ROCK,
        };
        self
    }

    fn player_score(self) -> usize {
        self.player_move.move_score() + self.outcome_score()
    }

    fn outcome_score(&self) -> usize {
        match self.player_outcome {
            LOSS => 0,
            DRAW => 3,
            WIN => 6,
        }
    }
}

impl MoveEnum {
    fn move_score(&self) -> usize {
        match self {
            ROCK => 1,
            PAPER => 2,
            SCISSORS => 3,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 15);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 11449);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 12);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 13187);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
}
