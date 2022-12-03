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

//#![feature(generators, generator_trait)]
//#![feature(drain_filter)]
//#![feature(const_option)]
//#![feature(type_alias_impl_trait)]
//#![feature(hash_drain_filter)]

extern crate test;

use std::cmp::Reverse;

//use grid::grid_array::*;
//#use grid::grid_iteration::*;
//use grid::grid_types::*;
use itertools::Itertools;
use utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////
    utils::with_measure("Part 1", || solve_part1("day01/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day01/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    utils::file_to_string(file_name)
        .split("\n\n")
        .map(|chunks_str| sum_of_nums(chunks_str))
        .max()
        .unwrap()
}

pub fn solve_part2(file_name: &str) -> usize {
    utils::file_to_string(file_name)
        .split("\n\n")
        .map(|chunks_str| sum_of_nums(chunks_str))
        .map(Reverse) // we want the largest but we only have k_smallest
        .k_smallest(3)
        .map(utils::unreverse) // Since elements are Reverse(items) we have to take .0
        .sum()
}

////////////////////////////////////////////////////////////////////////////////////

fn sum_of_nums(chunks_str: &str) -> usize {
    chunks_str.lines().map(utils::str_to::<usize>).sum()
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 24000);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 72602);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 45000);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 207410);
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
