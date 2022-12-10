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
    utils::with_measure("Part 1", || solve_part1("day10/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day10/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Op {
    Noop,
    Addx(isize),
}

struct Register {
    _vec: Vec<isize>,
}

const CYCLE_LENGTH: usize = 3;

impl Register {
    fn new(start_value: isize) -> Self {
        Self {
            _vec: vec![start_value; CYCLE_LENGTH],
        }
    }

    fn get(&self) -> isize {
        *self._vec.last().unwrap()
    }

    fn add(&mut self, value: isize) {
        self._vec[0] += value;
    }

    fn shift(&mut self) {
        let first_value = self._vec[0];
        self._vec.rotate_right(1);
        self._vec[0] = first_value;
    }
}

pub fn solve_part1(file_name: &str) -> isize {
    let mut cycle: usize = 1;
    let mut reg_x: Register = Register::new(1);
    let mut results = Vec::new();
    for op in utils::file_to_lines(file_name).map(line_to_op) {
        match op {
            Op::Noop => {
                //println!("{:?}", op);
                cycle += 1;
                results.push(get_score(cycle, &reg_x));
            }
            Op::Addx(num) => {
                //println!("{:?} x: {} {:?}", op, reg_x.get(), reg_x._vec);
                reg_x.add(num);

                reg_x.shift();
                cycle += 1;
                results.push(get_score(cycle, &reg_x));

                reg_x.shift();
                cycle += 1;
                results.push(get_score(cycle, &reg_x));
            }
        }
    }

    results.into_iter().flatten().sum()
}

pub fn solve_part2(file_name: &str) -> String {
    const CRT_WIDTH: usize = 40;

    let mut cycle: usize = 1;
    let mut reg_x: Register = Register::new(1);
    let mut results = Vec::new();
    for op in utils::file_to_lines(file_name).map(line_to_op) {
        match op {
            Op::Noop => {
                results.push(get_pixel(cycle, &reg_x));
                cycle += 1;
            }
            Op::Addx(num) => {
                reg_x.add(num);

                results.push(get_pixel(cycle, &reg_x));
                reg_x.shift();
                cycle += 1;

                results.push(get_pixel(cycle, &reg_x));
                reg_x.shift();
                cycle += 1;
            }
        }
    }

    let _result = results
        .into_iter()
        .chunks(CRT_WIDTH)
        .into_iter()
        .map(|x| x.collect::<String>())
        .join("\n");
    if cfg!(not(test)) {
        println!("{}", _result);
    }
    _result
}

fn get_pixel(cycle: usize, reg_x: &Register) -> char {
    let col = (cycle as isize - 1) % 40;
    let sprint_x = reg_x.get() as isize;
    if (col == sprint_x) || (col == sprint_x + 1) || (col == sprint_x - 1) {
        '#'
    } else {
        '.'
    }
}

fn line_to_op(line: String) -> Op {
    let op = if line == "noop" {
        Op::Noop
    } else {
        let num = line
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse::<isize>()
            .unwrap();
        Op::Addx(num)
    };
    op
}

fn get_score(cycle: usize, reg: &Register) -> Option<isize> {
    if (cycle == 20) || ((cycle as isize - 20) % 40 == 0) {
        Some(cycle as isize * reg.get())
    } else {
        None
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
        assert_eq!(solve_part1("test.txt"), 13140);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 14240);
    }

    #[test]
    fn test2() {
        assert_eq!(
            solve_part2("test.txt"),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
    }

    #[test]
    fn verify2() {
        assert_eq!(
            solve_part2("input.txt"),
            "###..#....#..#.#....#..#.###..####.#..#.
#..#.#....#..#.#....#.#..#..#....#.#..#.
#..#.#....#..#.#....##...###....#..####.
###..#....#..#.#....#.#..#..#..#...#..#.
#....#....#..#.#....#.#..#..#.#....#..#.
#....####..##..####.#..#.###..####.#..#."
        );
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
