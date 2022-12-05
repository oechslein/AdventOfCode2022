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

use std::collections::VecDeque;

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day05/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day05/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> String {
    solve(file_name, true)
}

pub fn solve_part2(file_name: &str) -> String {
    solve(file_name, false)
}

fn solve(file_name: &str, part1: bool) -> String {
    let (mut stack, moves) = parse(file_name);
    for (amount, from, to) in moves {
        let mut from_stack_values = stack[from - 1].drain(..amount).collect_vec();
        if !part1 {
            from_stack_values.reverse();
        }

        for elem in from_stack_values {
            stack[to - 1].push_front(elem);
        }
    }
    stack
        .iter()
        .map(|x| x.front().unwrap())
        .collect::<String>()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let input = utils::file_to_string(file_name);
    let (first_part, moves_part) = input.split_once("\n\n").unwrap();
    (parse_filled_stacks(first_part), parse_moves(moves_part))
}

fn parse_moves(moves_part: &str) -> Vec<(usize, usize, usize)> {
    let re = regex::Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    re.captures_iter(moves_part)
        .map(|captures| {
            captures
                .iter()
                .skip(1) // first result is full group
                .map(|cap| utils::str_to::<usize>(cap.unwrap().as_str()))
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec()
}

fn parse_filled_stacks(first_part: &str) -> Vec<VecDeque<char>> {
    let first_part = first_part.lines().collect_vec();
    let (first_part, middle_part) = first_part.split_at(first_part.len() - 1);
    let amount_of_stacks = (middle_part[0].len() + 1) / 4;
    let mut stack: Vec<VecDeque<char>> = (0..amount_of_stacks)
        .map(|_| VecDeque::with_capacity(first_part.len()))
        .collect_vec();

    for line in first_part {
        for (stack_index, mut part) in line.chars().chunks(4).into_iter().enumerate() {
            if part.next().unwrap() == '[' {
                stack[stack_index].push_back(part.next().unwrap());
            }
        }
    }

    stack
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), "CMZ");
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), "QMBMJDFTD");
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), "MCD");
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), "NBTVTJNFJ");
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
