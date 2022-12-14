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

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day06/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day06/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    solve(file_name, 4)
}

pub fn solve_part2(file_name: &str) -> usize {
    solve(file_name, 14)
}

////////////////////////////////////////////////////////////////////////////////////

fn solve(file_name: &str, length_marker: usize) -> usize {
    utils::file_to_string(file_name)
        .replace("\r\n", "\n")
        .chars()
        .collect_vec()
        .windows(length_marker)
        .enumerate()
        .find(|(_, window)| window.iter().all_unique())
        .unwrap()
        .0
        + length_marker
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 7);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1702);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 19);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 3559);
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
