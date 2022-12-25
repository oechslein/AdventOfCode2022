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

use std::collections::VecDeque;

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day25/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day25/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part2(file_name: &str) -> String {
    num_to_snafu(utils::file_to_lines(file_name).map(snafu_to_num).sum())
}

pub fn solve_part1(file_name: &str) -> String {
    num_to_snafu(utils::file_to_lines(file_name).map(snafu_to_num).sum())
}

fn num_to_snafu(num: usize) -> String {
    let mut num = num;
    let mut result = VecDeque::new();
    while num != 0 {
        let reminder = num % 5;
        num /= 5;
        result.push_front(match reminder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                // instead of 3 we are pushing -2, so we added 5 too much, so add 1 to our new_num (that holds the next digits)
                num += 1;
                '='
            }
            4 => {
                // instead of 4 we are pushing -1, so we added 5 too much, so add 1 to our new_num (that holds the next digits)
                num += 1;
                '-'
            }
            _ => unreachable!(),
        });
    }
    result.into_iter().join("")
}

#[allow(clippy::needless_pass_by_value)]
fn snafu_to_num(snafu: String) -> usize {
    snafu
        .chars()
        .fold(0, |result: isize, ch| {
            result * 5
                + match ch {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => unimplemented!(),
                }
        })
        .try_into()
        .unwrap()
}

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() {
        for (snafu, num) in vec![
            ("1=", 3),
            ("12", 7),
            ("21", 11),
            ("111", 31),
            ("112", 32),
            ("122", 37),
            ("1-12", 107),
            ("2=0=", 198),
            ("2=01", 201),
            ("1=-1=", 353),
            ("12111", 906),
            ("20012", 1257),
            ("1=-0-2", 1747),
        ] {
            // parse snafu from back to front
            let result = snafu_to_num(snafu.to_string());
            assert_eq!(result, num, "{result} != {num}");

            // convert num into snafu
            let result = num_to_snafu(num);
            //println!("{result} != {snafu}");
            assert_eq!(result, snafu, "{result} != {snafu}");
        }
    }

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), "2=-1=0");
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), "122-0==-=211==-2-200");
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), "2=-1=0");
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), "122-0==-=211==-2-200");
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
