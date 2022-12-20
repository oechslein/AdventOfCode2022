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

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day20/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day20/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> isize {
    solve(parse(file_name, 1), 1)
}

pub fn solve_part2(file_name: &str) -> isize {
    const DECRYPTION_KEY: isize = 811589153;
    solve(parse(file_name, DECRYPTION_KEY), 10)
}

////////////////////////////////////////////////////////////////////////////////////

fn solve(original_input: Vec<(isize, usize)>, times: i32) -> isize {
    let mut input = original_input.clone();
    for _ in 0..times {
        for value in original_input.iter() {
            let index = input.iter().position(|x| *x == *value).unwrap();
            rotate(&mut input, index);
        }
    }
    compute_result(input)
}

fn compute_result(input: Vec<(isize, usize)>) -> isize {
    let zero_pos = input.iter().position(|(x, _counter)| *x == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|pos| input[(zero_pos + pos) % input.len()].0)
        .sum()
}

fn rotate(input: &mut Vec<(isize, usize)>, index: usize) {
    let value = input.remove(index);
    // rust modulo is not the same as python modulo (-1 % 5 = -1 in rust, -1 % 5 = 4 in python) => rem_euclid
    let new_index = ((index as isize) + value.0).rem_euclid(input.len() as isize) as usize;
    input.insert(new_index, value);
}

fn parse(file_name: &str, decryption_key: isize) -> Vec<(isize, usize)> {
    utils::file_to_lines(file_name)
        .map(move |line| utils::str_to::<isize>(line.as_str()) * decryption_key)
        .enumerate()
        .map(|(index, x)| (x, index))
        .collect()
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 3);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 7004);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 1623178306);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 17200008919529);
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
