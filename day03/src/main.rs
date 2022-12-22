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

use std::{collections::HashSet, str::Chars};

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day03/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day03/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> u32 {
    utils::file_to_string(file_name)
        .replace("\r\n", "\n")
        .split('\n')
        .map(split_into_half)
        .map(|(compartment1, compartment2)| {
            type_priority_iterators(vec![compartment1, compartment2])
        })
        .sum::<u32>()
}

pub fn solve_part2(file_name: &str) -> u32 {
    utils::file_to_string(file_name)
        .replace("\r\n", "\n")
        .split('\n')
        .map(str::chars)
        .tuples::<(_, _, _)>()
        .map(|(rucksack1, rucksack2, rucksack3)| {
            type_priority_iterators(vec![rucksack1, rucksack2, rucksack3])
        })
        .sum()
}

////////////////////////////////////////////////////////////////////////////////////

fn split_into_half(line: &str) -> (Chars, Chars) {
    (
        line[0..line.len() / 2].chars(),
        line[line.len() / 2..line.len()].chars(),
    )
}

fn intersect_many<T: Eq + std::hash::Hash + Copy>(
    iterators: Vec<impl Iterator<Item = T>>,
) -> impl Iterator<Item = T> {
    let mut x = iterators.into_iter();
    let mut result: HashSet<_> = x.next().unwrap().collect();
    for next_vec in x {
        let other = next_vec.collect();
        let intersection = result.intersection(&other);
        result = intersection.copied().collect();
    }
    result.into_iter()
}

/*
fn intersect_many_iter<T: Eq + std::hash::Hash + Copy>(
    iterators: impl Iterator<Item = impl Iterator<Item = T>>,
) -> impl Iterator<Item = T> {
    let mut iterators = iterators;
    let mut result: HashSet<_> = HashSet::from_iter(iterators.next().unwrap());
    while let Some(next_vec) = iterators.next() {
        let other = HashSet::from_iter(next_vec);
        let intersection = result.intersection(&other);
        result = HashSet::from_iter(intersection.copied());
    }
    result.into_iter()
}
 */

fn type_priority_iterators(iterators: Vec<impl Iterator<Item = char>>) -> u32 {
    intersect_many(iterators).map(type_priority).sum::<u32>()
}

fn type_priority(x: char) -> u32 {
    if x.is_lowercase() {
        x as u32 - 96
    } else {
        x as u32 - 38
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
        assert_eq!(solve_part1("test.txt"), 157);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 7872);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 70);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 2497);
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
