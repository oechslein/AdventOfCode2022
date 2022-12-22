//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
#![feature(test)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

use std::str::FromStr;

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day02/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day02/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let input = utils::file_to_string(file_name).replace("\r\n", "\n");
    parse_input_part(&input)
        .map(Move::set_round_outcome)
        .map(Move::player_score)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let input = utils::file_to_string(file_name).replace("\r\n", "\n");
    parse_input_part(&input)
        .map(Move::set_player_move)
        .map(Move::player_score)
        .sum()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input_part(input: &str) -> impl Iterator<Item = Move> + '_ {
    input
        .split('\n')
        .filter_map(|line| Move::from_str(line).ok())
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
            println!("{first:?} {second:?}");
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
    Rock,
    Scissor,
    Paper,
}
use MoveEnum::*;

impl FromStr for MoveEnum {
    type Err = String;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let move_char = x.chars().next().unwrap();
        if move_char == 'A' || move_char == 'X' {
            Ok(Rock)
        } else if move_char == 'B' || move_char == 'Y' {
            Ok(Paper)
        } else if move_char == 'C' || move_char == 'Z' {
            Ok(Scissor)
        } else {
            Err(x.to_string())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RoundOutcome {
    Draw,
    Win,
    Loss,
}

use RoundOutcome::*;

impl FromStr for RoundOutcome {
    type Err = String;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        let move_char = x.chars().next().unwrap();
        if move_char == 'X' {
            Ok(Loss)
        } else if move_char == 'Y' {
            Ok(Draw)
        } else if move_char == 'Z' {
            Ok(Win)
        } else {
            Err(x.to_string())
        }
    }
}

impl Move {
    fn last_player_wins_p(self) -> bool {
        match self.player_move {
            Rock => self.opponent_move == Scissor,
            Scissor => self.opponent_move == Paper,
            Paper => self.opponent_move == Rock,
        }
    }

    fn set_round_outcome(mut self) -> Self {
        self.player_outcome = {
            if self.opponent_move == self.player_move {
                Draw
            } else if self.last_player_wins_p() {
                Win
            } else {
                Loss
            }
        };
        self
    }

    fn set_player_move(mut self) -> Self {
        self.player_move = match (self.player_outcome, self.opponent_move) {
            (Draw, _) => self.opponent_move,
            (Loss, Rock) | (Win, Paper) => Scissor,
            (Loss, Scissor) | (Win, Rock) => Paper,
            (Loss, Paper) | (Win, Scissor) => Rock,
        };
        self
    }

    fn player_score(self) -> usize {
        self.player_move.move_score() + self.outcome_score()
    }

    fn outcome_score(self) -> usize {
        match self.player_outcome {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

impl MoveEnum {
    fn move_score(self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
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
