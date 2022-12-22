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
#![allow(clippy::unreadable_literal)]


use std::{
    str::FromStr,
};

use derive_more::{IsVariant, Unwrap};
use fxhash::FxHashMap;
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day21/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day21/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

type MonkeyIndex = String;
const ROOT_MONKEY: &str = "root";
const HUMAN_MONKEY: &str = "humn";

pub fn solve_part1(file_name: &str) -> isize {
    parse(file_name)
        .get(ROOT_MONKEY)
        .unwrap()
        .eval(&parse(file_name))
}

pub fn solve_part2(file_name: &str) -> isize {
    let mut monkey_map = parse(file_name);
    //println!("{:?}", monkey_map);

    // Replace human entry
    monkey_map.insert(
        HUMAN_MONKEY.to_string(),
        MonkeyRule::Human(HUMAN_MONKEY.to_string()),
    );

    let (monkey1, monkey2) = monkey_map
        .get(ROOT_MONKEY)
        .unwrap()
        .get_used_monkeys_iter(&monkey_map)
        .collect_tuple()
        .unwrap();
    // only rule1 depends on human in the data
    assert_eq!(monkey1.count_human_usage(&monkey_map), 1);
    assert_eq!(monkey2.count_human_usage(&monkey_map), 0);

    let monkey2_value = monkey2.eval(&monkey_map);
    let human_value = monkey1.solve(monkey2_value, &monkey_map);
    if cfg!(not(test)) {
        let mut monkey_map = monkey_map.clone();
        monkey_map.insert(HUMAN_MONKEY.to_string(), MonkeyRule::Number(human_value));
        assert_eq!(monkey2_value, monkey1.eval(&monkey_map));
    }
    human_value
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn op_name(&self) -> String {
        match self {
            Operation::Add => "+".to_string(),
            Operation::Sub => "-".to_string(),
            Operation::Mul => "*".to_string(),
            Operation::Div => "/".to_string(),
        }
    }

    fn eval(&self, value1: isize, value2: isize) -> isize {
        match self {
            Operation::Add => value1 + value2,
            Operation::Sub => value1 - value2,
            Operation::Mul => value1 * value2,
            Operation::Div => value1 / value2,
        }
    }

    fn eval_inverse_right(&self, value1: isize, value2: isize) -> isize {
        match self {
            Operation::Add => value1 - value2,
            Operation::Sub => value1 + value2,
            Operation::Mul => value1 / value2,
            Operation::Div => value1 * value2,
        }
    }

    fn eval_inverse_left(&self, value1: isize, value2: isize) -> isize {
        match self {
            Operation::Add => value1 - value2,
            Operation::Sub => value2 - value1,
            Operation::Mul => value1 / value2,
            Operation::Div => value2 / value1,
        }
    }
}

#[derive(Debug, Clone, IsVariant, Unwrap)]
enum MonkeyRule {
    Number(isize),
    Monkey(MonkeyIndex),
    Human(MonkeyIndex),
    Operation(Operation, Box<MonkeyRule>, Box<MonkeyRule>),
}

impl MonkeyRule {
    fn count_human_usage(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyRule>) -> isize {
        if self.is_human() {
            return 1;
        }
        self.get_used_monkeys_iter(monkey_map)
            .map(|monkey| monkey.count_human_usage(monkey_map))
            .sum()
    }

    fn get_used_monkeys_iter<'a>(
        &self,
        monkey_map: &'a FxHashMap<MonkeyIndex, MonkeyRule>,
    ) -> impl Iterator<Item = &'a MonkeyRule> {
        fn get_used_monkey_names(rule: &MonkeyRule) -> impl Iterator<Item = &MonkeyIndex> {
            match rule {
                MonkeyRule::Number(_) => vec![].into_iter(),
                MonkeyRule::Monkey(monkey_name) => vec![monkey_name].into_iter(),
                MonkeyRule::Human(monkey_name) => vec![monkey_name].into_iter(),
                MonkeyRule::Operation(_, rule1, rule2) => {
                    let x = get_used_monkey_names(&rule1)
                        .chain(get_used_monkey_names(&rule2))
                        .collect_vec();
                    x.into_iter()
                }
            }
        }

        get_used_monkey_names(self)
            .map(|monkey_name| monkey_map.get(monkey_name).unwrap())
            .collect_vec()
            .into_iter()
    }

    fn eval(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyRule>) -> isize {
        match self {
            MonkeyRule::Number(n) => *n,
            MonkeyRule::Operation(op, rule1, rule2) => {
                op.eval(rule1.eval(monkey_map), rule2.eval(monkey_map))
            }
            MonkeyRule::Monkey(monkey_name) => {
                monkey_map.get(monkey_name).unwrap().eval(monkey_map)
            }
            MonkeyRule::Human(_) => panic!("Can't eval human, need to solve first"),
        }
    }

    fn depends_on_human(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyRule>) -> bool {
        match self {
            MonkeyRule::Number(_) => false,
            MonkeyRule::Human(_) => true,
            MonkeyRule::Monkey(monkey_name) => monkey_map
                .get(monkey_name)
                .unwrap()
                .depends_on_human(monkey_map),
            MonkeyRule::Operation(_, rule1, rule2) => {
                rule1.depends_on_human(monkey_map) || rule2.depends_on_human(monkey_map)
            }
        }
    }

    pub fn solve(&self, value: isize, monkey_map: &FxHashMap<MonkeyIndex, MonkeyRule>) -> isize {
        match self {
            MonkeyRule::Number(n) => *n,
            MonkeyRule::Human(_) => value,
            MonkeyRule::Monkey(monkey_name) => monkey_map
                .get(monkey_name)
                .unwrap()
                .solve(value, monkey_map),
            MonkeyRule::Operation(op, rule1, rule2) => {
                if rule1.depends_on_human(monkey_map) {
                    rule1.solve(
                        op.eval_inverse_right(value, rule2.eval(monkey_map)),
                        monkey_map,
                    )
                } else {
                    assert!(
                        !rule1.depends_on_human(monkey_map) && rule2.depends_on_human(monkey_map)
                    );
                    rule2.solve(
                        op.eval_inverse_left(value, rule1.eval(monkey_map)),
                        monkey_map,
                    )
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> FxHashMap<String, MonkeyRule> {
    utils::file_to_lines(file_name)
        .map(|line| {
            let (name, operation) = line.split_once(": ").unwrap();
            (name.to_string(), MonkeyRule::from_str(operation).unwrap())
        })
        .collect()
}

impl FromStr for MonkeyRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn _create_monkey(monkey: &str) -> Box<MonkeyRule> {
            Box::new(MonkeyRule::Monkey(monkey.trim().to_string()))
        }

        [
            Operation::Add,
            Operation::Sub,
            Operation::Mul,
            Operation::Div,
        ]
        .into_iter()
        .find(|op| s.contains(&op.op_name()))
        .map(|op| {
            let (monkey1, monkey2) = s.split_once(&op.op_name()).unwrap();
            Ok(MonkeyRule::Operation(
                op,
                _create_monkey(monkey1),
                _create_monkey(monkey2),
            ))
        })
        .unwrap_or_else(|| Ok(MonkeyRule::Number(s.parse().unwrap())))
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
        assert_eq!(solve_part1("test.txt"), 152);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 80326079210554);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 301);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 3617613952378);
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
