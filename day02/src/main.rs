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

extern crate test;

use std::{cmp::Reverse, str::FromStr};

use itertools::Itertools;
use utils::{self, str_to};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day01/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day01/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse_input_part_1(&utils::file_to_string(file_name))
        .map(player_score)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let input = utils::file_to_string(file_name);
    let input = parse_input_part_2(&input);
    input.map(add_needed_move).map(player_score).sum()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input_part_1<'a>(input: &'a String) -> impl Iterator<Item = (Move, Move)> + 'a {
    input.split('\n').filter_map(|line| {
        line.trim()
            .split(' ')
            .map(|x| Move::from_str(x).unwrap())
            .collect_tuple::<(_, _)>()
    })
}

fn parse_input_part_2<'a>(input: &'a String) -> impl Iterator<Item = (Move, RoundOutcome)> + 'a {
    input.split('\n').filter_map(|line| {
        line.trim()
            .split(' ')
            .collect_tuple::<(_, _)>()
            .and_then(|(c1, c2)| Some((Move::from_str(c1).unwrap(), RoundOutcome::from_str(c2).unwrap())))
    })
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    ROCK,
    SCISSORS,
    PAPER,
}
use Move::*;

impl FromStr for Move {
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

fn last_player_wins_p((opponent_move, own_move): (Move, Move)) -> bool {
    match own_move {
        ROCK => opponent_move == SCISSORS,
        SCISSORS => opponent_move == PAPER,
        PAPER => opponent_move == ROCK,
    }
}

fn round_outcome((opponent_move, own_move): (Move, Move)) -> RoundOutcome {
    if opponent_move == own_move {
        DRAW
    } else if last_player_wins_p((opponent_move, own_move)) {
        WIN
    } else {
        assert!(last_player_wins_p((own_move, opponent_move)));
        LOSS
    }
}

fn player_score((opponent_move, own_move): (Move, Move)) -> usize {
    (match own_move {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    }) + (match round_outcome((opponent_move, own_move)) {
        LOSS => 0,
        DRAW => 3,
        WIN => 6,
    })
}

fn add_needed_move((opponent_move, outcome): (Move, RoundOutcome)) -> (Move, Move) {
    (
        opponent_move,
        match (outcome, opponent_move) {
            (DRAW, ROCK) => ROCK,
            (DRAW, SCISSORS) => SCISSORS,
            (DRAW, PAPER) => PAPER,
            (WIN, ROCK) => PAPER,
            (WIN, SCISSORS) => ROCK,
            (WIN, PAPER) => SCISSORS,
            (LOSS, ROCK) => SCISSORS,
            (LOSS, SCISSORS) => PAPER,
            (LOSS, PAPER) => ROCK,
        },
    )
}

////////////////////////////////////////////////////////////////////////////////////
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
