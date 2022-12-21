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

use std::{collections::HashMap, fmt::Display, str::FromStr};

use fxhash::FxHashMap;
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    //utils::with_measure("Part 1", || solve_part1("day21/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day21/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> isize {
    let monkey_map = parse_part1(file_name);
    println!("{:?}", monkey_map);

    let root_monkey = monkey_map.get(ROOT_MONKEY).unwrap();
    root_monkey.apply(&monkey_map)
}

pub fn solve_part2(file_name: &str) -> isize {
    let monkey_map = parse_part1(file_name);
    println!("{:?}", monkey_map);

    let (op1, op2) = monkey_map
        .get(ROOT_MONKEY)
        .unwrap()
        .operation
        .get_used_monkey_names()
        .map(|monkey_name| monkey_map.get(monkey_name).unwrap())
        .collect_tuple()
        .unwrap();
    // only op1 depend on human
    assert_eq!(
        op1.count_monkey_usage(&HUMAN_MONKEY.to_string(), &monkey_map),
        1
    );
    assert_eq!(
        op2.count_monkey_usage(&HUMAN_MONKEY.to_string(), &monkey_map),
        0
    );

    // so we can evalualte op2
    let op2_part2 = op2.convert_to_part2(&monkey_map);
    println!("op2_part2: {}", op2_part2);
    let op2_value = op2_part2.eval();
    println!("op2_value: {}", op2_value);

    // convert to OperationPart2
    let op1_part2 = op1.convert_to_part2(&monkey_map);
    println!("op1_part2: {}", op1_part2);

    // Ie op1 needs to eval to op2_value
    let x = op1_part2.solve(op2_value);
    println!("{}", x);
    // ((4 + (2 * (600 - 3))) / 4) =

    x
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
enum OperationPart2 {
    Number(isize),
    Human,
    Add(Box<OperationPart2>, Box<OperationPart2>),
    Sub(Box<OperationPart2>, Box<OperationPart2>),
    Mul(Box<OperationPart2>, Box<OperationPart2>),
    Div(Box<OperationPart2>, Box<OperationPart2>),
}
impl OperationPart2 {
    fn eval(&self) -> isize {
        match self {
            OperationPart2::Number(n) => *n,
            OperationPart2::Add(op1, op2) => op1.eval() + op2.eval(),
            OperationPart2::Sub(op1, op2) => op1.eval() - op2.eval(),
            OperationPart2::Mul(op1, op2) => op1.eval() * op2.eval(),
            OperationPart2::Div(op1, op2) => op1.eval() / op2.eval(),
            OperationPart2::Human => panic!("Can't eval human"),
        }
    }

    fn depends_on_human(&self) -> bool {
        match self {
            OperationPart2::Number(_) => false,
            OperationPart2::Human => true,
            OperationPart2::Add(op1, op2) => op1.depends_on_human() || op2.depends_on_human(),
            OperationPart2::Sub(op1, op2) => op1.depends_on_human() || op2.depends_on_human(),
            OperationPart2::Mul(op1, op2) => op1.depends_on_human() || op2.depends_on_human(),
            OperationPart2::Div(op1, op2) => op1.depends_on_human() || op2.depends_on_human(),
        }
    }

    pub fn solve(&self, value: isize) -> isize {
        /*
            X = (OP1 op OP2)
            (if OP1 depends on HUMAN)
            => (X invert(op) eval(OP2)) = OP1

            X = (OP1 op OP2)
            (if OP2 depends on HUMAN)
            => (eval(OP1) invert(op) X) = OP2

            X= HUMAN?!
        */
        match self {
            OperationPart2::Number(n) => *n,
            OperationPart2::Human => value,
            OperationPart2::Add(op1, op2) => {
                if op1.depends_on_human() {
                    op1.solve(value - op2.eval())
                } else {
                    assert!(!op1.depends_on_human() && op2.depends_on_human());
                    op2.solve(value - op1.eval())
                }
            }
            OperationPart2::Sub(op1, op2) => {
                if op1.depends_on_human() {
                    op1.solve(value + op2.eval())
                } else {
                    assert!(!op1.depends_on_human() && op2.depends_on_human());
                    op2.solve(op1.eval() - value)
                }
            }
            OperationPart2::Mul(op1, op2) => {
                if op1.depends_on_human() {
                    op1.solve(value / op2.eval())
                } else {
                    assert!(!op1.depends_on_human() && op2.depends_on_human());
                    op2.solve(value / op1.eval())
                }
            }
            OperationPart2::Div(op1, op2) => {
                if op1.depends_on_human() {
                    op1.solve(value * op2.eval())
                } else {
                    assert!(!op1.depends_on_human() && op2.depends_on_human());
                    op2.solve(op1.eval() / value)
                }
            }
        }
    }
}

impl Display for OperationPart2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationPart2::Number(n) => write!(f, "{}", n),
            OperationPart2::Human => write!(f, "Human"),
            OperationPart2::Add(op1, op2) => write!(f, "({} + {})", op1, op2),
            OperationPart2::Sub(op1, op2) => write!(f, "({} - {})", op1, op2),
            OperationPart2::Mul(op1, op2) => write!(f, "({} * {})", op1, op2),
            OperationPart2::Div(op1, op2) => write!(f, "({} / {})", op1, op2),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

type MonkeyIndex = String;
const ROOT_MONKEY: &str = "root";
const HUMAN_MONKEY: &str = "humn";

#[derive(Debug, Clone)]
struct MonkeyPart1 {
    name: MonkeyIndex,
    operation: OperationPart1,
}

impl MonkeyPart1 {
    fn apply(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyPart1>) -> isize {
        self.operation.eval(monkey_map)
    }

    fn count_monkey_usage(
        &self,
        monkey_name: &MonkeyIndex,
        monkey_map: &FxHashMap<MonkeyIndex, MonkeyPart1>,
    ) -> isize {
        if monkey_name == &self.name {
            return 1;
        }
        self.operation
            .get_used_monkey_names()
            .map(|monkey_index| {
                monkey_map
                    .get(monkey_index)
                    .unwrap()
                    .count_monkey_usage(monkey_name, monkey_map)
            })
            .sum()
    }

    fn eval(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyPart1>) -> isize {
        self.operation.eval(monkey_map)
    }

    pub fn convert_to_part2(
        &self,
        monkey_map: &FxHashMap<MonkeyIndex, MonkeyPart1>,
    ) -> OperationPart2 {
        let convert_to_part2_inner = |monkey_name: &MonkeyIndex| -> Box<OperationPart2> {
            let monkey = monkey_map.get(monkey_name).unwrap();
            Box::new(monkey.convert_to_part2(monkey_map))
        };

        if self.name == HUMAN_MONKEY.to_string() {
            return OperationPart2::Human;
        }
        match self {
            MonkeyPart1 {
                name: _,
                operation: OperationPart1::Number(n),
            } => OperationPart2::Number(*n),
            MonkeyPart1 {
                name: _,
                operation: OperationPart1::Add(monkey1, monkey2),
            } => OperationPart2::Add(
                convert_to_part2_inner(monkey1),
                convert_to_part2_inner(monkey2),
            ),
            MonkeyPart1 {
                name: _,
                operation: OperationPart1::Sub(monkey1, monkey2),
            } => OperationPart2::Sub(
                convert_to_part2_inner(monkey1),
                convert_to_part2_inner(monkey2),
            ),
            MonkeyPart1 {
                name: _,
                operation: OperationPart1::Mul(monkey1, monkey2),
            } => OperationPart2::Mul(
                convert_to_part2_inner(monkey1),
                convert_to_part2_inner(monkey2),
            ),
            MonkeyPart1 {
                name: _,
                operation: OperationPart1::Div(monkey1, monkey2),
            } => OperationPart2::Div(
                convert_to_part2_inner(monkey1),
                convert_to_part2_inner(monkey2),
            ),
        }
    }
}

#[derive(Debug, Clone)]
enum OperationPart1 {
    Number(isize),
    Add(MonkeyIndex, MonkeyIndex),
    Sub(MonkeyIndex, MonkeyIndex),
    Mul(MonkeyIndex, MonkeyIndex),
    Div(MonkeyIndex, MonkeyIndex),
}
impl OperationPart1 {
    fn eval(&self, monkey_map: &FxHashMap<MonkeyIndex, MonkeyPart1>) -> isize {
        match self {
            OperationPart1::Number(n) => *n,
            OperationPart1::Add(monkey1, monkey2) => {
                let monkey1 = monkey_map.get(monkey1).unwrap();
                let monkey2 = monkey_map.get(monkey2).unwrap();
                monkey1.apply(monkey_map) + monkey2.apply(monkey_map)
            }
            OperationPart1::Sub(monkey1, monkey2) => {
                let monkey1 = monkey_map.get(monkey1).unwrap();
                let monkey2 = monkey_map.get(monkey2).unwrap();
                monkey1.apply(monkey_map) - monkey2.apply(monkey_map)
            }
            OperationPart1::Mul(monkey1, monkey2) => {
                let monkey1 = monkey_map.get(monkey1).unwrap();
                let monkey2 = monkey_map.get(monkey2).unwrap();
                monkey1.apply(monkey_map) * monkey2.apply(monkey_map)
            }
            OperationPart1::Div(monkey1, monkey2) => {
                let monkey1 = monkey_map.get(monkey1).unwrap();
                let monkey2 = monkey_map.get(monkey2).unwrap();
                monkey1.apply(monkey_map) / monkey2.apply(monkey_map)
            }
        }
    }

    fn get_used_monkey_names(&self) -> impl Iterator<Item = &MonkeyIndex> {
        match self {
            OperationPart1::Number(_) => vec![].into_iter(),
            OperationPart1::Add(monkey1, monkey2) => vec![monkey1, monkey2].into_iter(),
            OperationPart1::Sub(monkey1, monkey2) => vec![monkey1, monkey2].into_iter(),
            OperationPart1::Mul(monkey1, monkey2) => vec![monkey1, monkey2].into_iter(),
            OperationPart1::Div(monkey1, monkey2) => vec![monkey1, monkey2].into_iter(),
        }
    }
}

impl Display for OperationPart1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationPart1::Number(n) => write!(f, "{}", n),
            OperationPart1::Add(op1, op2) => write!(f, "({} + {})", op1, op2),
            OperationPart1::Sub(op1, op2) => write!(f, "({} - {})", op1, op2),
            OperationPart1::Mul(op1, op2) => write!(f, "({} * {})", op1, op2),
            OperationPart1::Div(op1, op2) => write!(f, "({} / {})", op1, op2),
        }
    }
}

impl FromStr for OperationPart1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('+') {
            let (monkey1, monkey2) = s.split_once(" + ").unwrap();
            Ok(OperationPart1::Add(
                monkey1.to_string(),
                monkey2.to_string(),
            ))
        } else if s.contains('-') {
            let (monkey1, monkey2) = s.split_once(" - ").unwrap();
            Ok(OperationPart1::Sub(
                monkey1.to_string(),
                monkey2.to_string(),
            ))
        } else if s.contains('*') {
            let (monkey1, monkey2) = s.split_once(" * ").unwrap();
            Ok(OperationPart1::Mul(
                monkey1.to_string(),
                monkey2.to_string(),
            ))
        } else if s.contains('/') {
            let (monkey1, monkey2) = s.split_once(" / ").unwrap();
            Ok(OperationPart1::Div(
                monkey1.to_string(),
                monkey2.to_string(),
            ))
        } else {
            Ok(OperationPart1::Number(s.parse().unwrap()))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_part1(
    file_name: &str,
) -> HashMap<String, MonkeyPart1, std::hash::BuildHasherDefault<fxhash::FxHasher>> {
    let monkey_map: FxHashMap<MonkeyIndex, MonkeyPart1> = utils::file_to_lines(file_name)
        .map(|line| {
            let (name, operation) = line.split_once(": ").unwrap();
            (
                name.to_string(),
                MonkeyPart1 {
                    name: name.to_string(),
                    operation: OperationPart1::from_str(operation).unwrap(),
                },
            )
        })
        .collect();
    monkey_map
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
        assert_eq!(solve_part2("input.txt"), 42);
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
