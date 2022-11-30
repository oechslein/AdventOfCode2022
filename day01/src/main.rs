#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]
#![feature(hash_drain_filter)]

#[macro_use]
extern crate derive_builder;
extern crate num_derive;

extern crate test;

use std::process::ExitCode;

use itertools::{cloned, Itertools};

use core::num;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::io::BufRead;
use std::ops::{Add, RangeFrom, SubAssign};
use std::str::FromStr;
use std::{error, mem};

use std::fs;

use array2d::Array2D;

use rand::prelude::*;

mod utils;
mod grid;


////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("input.txt"));
    utils::with_measure("Part 2", || solve_part2("input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

fn solve_part1(file_name: &str) -> i64 {
    let elements: Vec<Vec<char>> = utils::file_to_lines(file_name)
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut a = Array2D::from_columns(&elements);
    let mut step = 1;

    if cfg!(debug_assertions) {
        print_array(&a);
        println!();
    }

    loop {
        let new_a = perform_step(&a);

        if a == new_a {
            break;
        }
        a = new_a;
        step += 1;
    }
    if cfg!(debug_assertions) {
        print_array(&a);
        println!();
    }

    step
}

fn perform_step(a: &Array2D<char>) -> Array2D<char> {
    let mut new_a = a.clone();
    for y in 0..a.row_len() {
        for x in 0..a.column_len() {
            if a[(x, y)] == '>' && a[((x + 1) % a.column_len(), y)] == '.' {
                new_a[((x + 1) % a.column_len(), y)] = '>';
                new_a[(x, y)] = '.';
            }
        }
    }
    let mut new_a2 = new_a.clone();
    for x in 0..a.column_len() {
        for y in 0..a.row_len() {
            if new_a[(x, y)] == 'v' && new_a[(x, (y + 1) % a.row_len())] == '.' {
                new_a2[(x, (y + 1) % a.row_len())] = 'v';
                new_a2[(x, y)] = '.';
            }
        }
    }

    if cfg!(debug_assertions) {
        print_array(a);
        println!();
        println!();
    }

    new_a2
}

fn print_array(a: &Array2D<char>) {
    assert_eq!(a.row_len(), a.row_len());
    assert_eq!(a.column_len(), a.column_len());
    for y in 0..a.row_len() {
        for x in 0..a.column_len() {
            print!("{}", a[(x, y)]);
        }
        println!();
    }
}

fn solve_part2(file_name: &str) -> i64 {
    solve_part1(file_name)
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() {}

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 58);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 386);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test2.txt"), 58);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 386);
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
