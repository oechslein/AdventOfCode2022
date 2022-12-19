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

mod rock;
mod floor;
use floor::*;


////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day17/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day17/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

// test: 1*41=41
// input: 2*2*3*29*29 = 10092

const WIDTH: usize = 7;
const REMOVE_UNREACHABLE_LINES: bool = false;
const ROCK_AMOUNT: usize = 5;
const DRAW_FALLING_ROCKS: bool = false;
const DRAW_FLOOR: bool = false;

pub fn solve_part1(file_name: &str) -> usize {
    Floor::new(utils::file_to_string(file_name)).solve(2022)
}

pub fn solve_part2(file_name: &str) -> usize {
    Floor::new(utils::file_to_string(file_name)).solve(1_000_000_000_000)
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 3068);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 3114);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 1514285714288);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 1540804597682);
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
