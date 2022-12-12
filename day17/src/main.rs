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

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day17/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day17/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let _input = utils::file_to_lines(file_name);
    42
}

pub fn solve_part2(file_name: &str) -> usize {
    let _input = utils::file_to_lines(file_name);
    42
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 42);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 42);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 42);
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
