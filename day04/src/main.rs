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

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day04/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day04/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse_input(file_name)
        .filter(|(interval1, interval2)| {
            is_fully_contained(interval1, interval2) || is_fully_contained(interval2, interval1)
        })
        .count()
}

pub fn solve_part2(file_name: &str) -> usize {
    parse_input(file_name)
        .filter(|(interval1, interval2)| {
            is_fully_contained(interval1, interval2) || overlaps(interval1, interval2)
        })
        .count()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input<'a>(
    file_name: &'a str,
) -> impl Iterator<Item = ((usize, usize), (usize, usize))> + 'a {
    utils::file_to_string(file_name)
        .replace("\r\n", "\n")
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|interval| {
                    interval
                        .split('-')
                        .map(utils::str_to::<usize>)
                        .collect_tuple::<(_, _)>()
                        .unwrap()
                })
                .collect_tuple::<((usize, usize), (usize, usize))>()
                .unwrap()
        })
        .collect_vec()
        .into_iter()
}

fn is_fully_contained(interval1: &(usize, usize), interval2: &(usize, usize)) -> bool {
    let (interval1, interval2) = if interval1.0 <= interval2.0 {
        (interval1, interval2)
    } else {
        (interval2, interval1)
    };
    interval1.0 <= interval2.0 && interval2.1 <= interval1.1
}

fn overlaps(interval1: &(usize, usize), interval2: &(usize, usize)) -> bool {
    let (interval1, interval2) = if interval1.0 <= interval2.0 {
        (interval1, interval2)
    } else {
        (interval2, interval1)
    };
    interval1.0 <= interval2.0 && interval2.0 <= interval1.1 && interval1.1 <= interval2.1
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 2);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 471);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 4);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 888);
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
