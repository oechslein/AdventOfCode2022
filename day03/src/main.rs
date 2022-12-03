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

use std::{cmp::Reverse, collections::HashSet, str::FromStr};

use itertools::Itertools;
use utils::{self, str_to};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day03/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day03/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse_input_part_1(&utils::file_to_string(file_name)).map(sum_of_types_priority).sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let input = utils::file_to_string(file_name);
    for (rucksack1, rucksack2, rucksack3) in parse_input_part_2(&input) {
        let rucksack1_set: HashSet<_> = HashSet::from_iter(rucksack1.iter());
        let rucksack2_set: HashSet<_> = HashSet::from_iter(rucksack2.iter());
        let rucksack3_set: HashSet<_> = HashSet::from_iter(rucksack3.iter());

        let common_1_2 = rucksack1_set.intersection(&rucksack2_set).map(|x| *x).collect::<HashSet<_>>();
        let x = common_1_2.intersection(&rucksack3_set);
        x.map(|x| type_priority(**x)).sum::<usize>();
        println!("{:?}", x);    
    }
    42
}

////////////////////////////////////////////////////////////////////////////////////

type Group = (RucksackPart2, RucksackPart2, RucksackPart2);
type RucksackPart2 = Vec<char>;

fn parse_input_part_2<'a>(
    input: &'a String,
) -> impl Iterator<Item = Group> + 'a {
    input
        .split('\n')
        .map(|line| {
                line.chars().collect_vec()
        })
        .tuples::<Group>()
}

////////////////////////////////////////////////////////////////////////////////////

type RucksackPart1 = (Compartment, Compartment);
type Compartment = Vec<char>;

fn parse_input_part_1<'a>(input: &'a String) -> impl Iterator<Item = RucksackPart1> + 'a {
    input
        .split('\n')
        .map(|line| {
            (
                line[0..line.len() / 2].chars().collect_vec(),
                line[line.len() / 2..line.len()].chars().collect_vec(),
            )
        })
        .into_iter()
}

fn sum_of_types_priority((compart1, compart2): RucksackPart1) -> usize {
    let compart1_set: HashSet<_> = HashSet::from_iter(compart1.iter());
    let compart2_set: HashSet<_> = HashSet::from_iter(compart2.iter());
    let common_parts = compart1_set.intersection(&compart2_set);
    //println!("{:?},{:?}=>{:?}", compart1, compart2, common_parts.clone());
    let sum_of_types_priority = common_parts
        .into_iter()
        .map(|x| type_priority(**x))
        .sum::<u32>();
    sum_of_types_priority as usize
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
