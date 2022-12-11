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

use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day11/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day11/input.txt"));
}

const DEBUG_PRINT: bool = false;

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let monkey_vec = parse(file_name);
    do_rounds(20, monkey_vec, None)
        .into_iter()
        .map(|m| m.item_inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn solve_part2(file_name: &str) -> usize {
    let monkey_vec = parse(file_name);
    let divisible_product = Some(monkey_vec.iter().map(|m| m.divisible_by).product());
    do_rounds(10000, monkey_vec, divisible_product)
        .into_iter()
        .map(|m| m.item_inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> Vec<Monkey> {
    utils::file_to_lines(file_name)
        .chunks(7)
        .into_iter()
        .map(Monkey::new)
        .collect_vec()
}

fn do_rounds(
    rounds: usize,
    mut monkey_vec: Vec<Monkey>,
    product_of_divisible: Option<usize>,
) -> Vec<Monkey> {
    for _round in 1..=rounds {
        for monkey_index in 0..monkey_vec.len() {
            while let Some(item) = monkey_vec[monkey_index].items.pop_front() {
                let monkey = &mut monkey_vec[monkey_index];
                let new_item = monkey.inspect_item(item, product_of_divisible);
                let new_monkey_index = monkey.get_next_monkey(new_item);
                monkey_vec[new_monkey_index].items.push_back(new_item);
            }
        }

        if DEBUG_PRINT {
            println!(
                "After round {}, the monkeys are holding items with these worry levels:",
                _round
            );
            for (monkey_index, monkey) in monkey_vec.iter().enumerate() {
                println!(
                    "Monkey {}: {:?}",
                    monkey_index,
                    monkey.items.iter().join(", ")
                );
            }
        }
    }
    monkey_vec
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    divisible_by: usize,
    monkey_index_true: usize,
    monkey_index_false: usize,
    item_inspection_count: usize,
}

impl Monkey {
    fn get_next_monkey(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.monkey_index_true
        } else {
            self.monkey_index_false
        }
    }

    fn inspect_item(&mut self, item: usize, product_of_divisible: Option<usize>) -> usize {
        self.item_inspection_count += 1;
        let result = self.op.apply(item);
        if let Some(product_of_divisible) = product_of_divisible {
            // the result can be reduced as long as it is still divisible by all monkeys
            result % product_of_divisible
        } else {
            result / 3
        }
    }

    fn new(money: itertools::Chunk<impl Iterator<Item = String>>) -> Monkey {
        let monkey = money.skip(1).take(5).collect_vec();
        let starting_items = monkey[0]
            .replace("  Starting items: ", "")
            .split(", ")
            .map(utils::str_to::<usize>)
            .collect();
        let op_str = monkey[1].replace("  Operation: new = old ", "");
        let op: Operation = Operation::from_str(&op_str).unwrap();
        let divisible_by = monkey[2]
            .replace("  Test: divisible by ", "")
            .parse::<usize>()
            .unwrap();
        let monkey_true = monkey[3]
            .replace("    If true: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        let monkey_false = monkey[4]
            .replace("    If false: throw to monkey ", "")
            .parse::<usize>()
            .unwrap();
        Monkey {
            items: starting_items,
            op: op,
            divisible_by: divisible_by,
            monkey_index_true: monkey_true,
            monkey_index_false: monkey_false,
            item_inspection_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}
impl Operation {
    fn apply(&self, item: usize) -> usize {
        match self {
            Operation::Add(x) => item + x,
            Operation::Mul(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, operand) = s.split_whitespace().collect_tuple().unwrap();
        let op = match op {
            "+" => Operation::Add(utils::str_to(operand)),
            "*" => {
                if operand == "old" {
                    Operation::Square
                } else {
                    Operation::Mul(utils::str_to(operand))
                }
            }
            _ => unreachable!(),
        };
        Ok(op)
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
        assert_eq!(solve_part1("test.txt"), 10605);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 62491);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 2713310158);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 17408399184);
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
