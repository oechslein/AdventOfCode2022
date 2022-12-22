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

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day19/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day19/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

type UInt = u16;

pub fn solve_part1(file_name: &str) -> UInt {
    let blueprints = parse_blueprints(file_name);
    //println!("{:?}", blueprints);

    blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * get_max_geodes(24, blueprint))
        .sum()
}

pub fn solve_part2(file_name: &str) -> UInt {
    let blueprints = parse_blueprints(file_name)
        .into_iter()
        .take(3)
        .collect_vec();
    //println!("{:?}", blueprints);

    blueprints
        .par_iter()
        .map(|blueprint| get_max_geodes(32, blueprint))
        .product()
}

////////////////////////////////////////////////////////////////////////////////////

fn get_max_geodes(limit: UInt, blueprint: &Blueprint) -> UInt {
    let mut max_geode_robot_per_time = vec![0; limit as usize + 1];
    let mut visited_nodes = FxHashSet::default();
    let mut open_nodes = VecDeque::new();
    open_nodes.push_front(Node::new());
    let mut finished_nodes = vec![];

    let mut add_node_if_not_visited = |node: Node, open_nodes: &mut VecDeque<_>| {
        if !visited_nodes.contains(&node) {
            visited_nodes.insert(node.clone());
            open_nodes.push_back(node);
        } else {
            //println!("Already visited: {:?}", new_node);
        }
    };

    // Calculate the maximum amount for every type of bot so that the creation of a new bot of any type is never bottlenecked
    let max_robots: FxHashMap<Robot, UInt> = Robot::iter()
        .map(|robot_that_generate_element| {
            (robot_that_generate_element, {
                Robot::iter()
                    .map(|robot| {
                        blueprint.element_costs_for_robot(robot, robot_that_generate_element)
                    })
                    .filter(|cost| *cost != 0) // Genodes can't be created by any robot but needed of course
                    .max()
                    .unwrap_or(limit)
            })
        })
        .collect();
    //println!("max_robots: {:?}", max_robots);

    while let Some(mut node) = open_nodes.pop_front() {
        if node.spent_minutes >= limit {
            if false {
                println!(
                    "geode: {}, node: {:?} ",
                    node.elements[Element::Geode as usize],
                    node
                );
            }
            finished_nodes.push(node);
            continue;
        }

        if max_geode_robot_per_time[node.spent_minutes as usize]
            > node.robots[Robot::Geode as usize]
        {
            //println!("Pruned: {:?} {}", node, max_geode_robot_per_time[node.spent_minutes]);
            continue;
        }

        max_geode_robot_per_time[node.spent_minutes as usize] =
            node.robots[Element::Geode as usize];

        node.spent_minutes += 1;

        // first check what robots are possible to buy
        let possible_robots = Robot::iter()
            .rev()
            .filter(|robot| {
                node.can_buy_robot(*robot, blueprint)
                    && node.robots[*robot as usize] <= max_robots[robot]
            })
            .collect_vec();
        // then collect
        node.robots_collect();

        // then buy (if possible)
        // option buy robot if enough elements
        for robot in possible_robots {
            let mut new_node = node.clone();
            new_node.buy_robot(robot, blueprint);
            add_node_if_not_visited(new_node, &mut open_nodes);
        }

        // option do nothing
        add_node_if_not_visited(node, &mut open_nodes);
    }

    let max_node = finished_nodes
        .into_iter()
        .max_by_key(|node| node.elements[Element::Geode as usize])
        .unwrap();
    let max_geodes = max_node.elements[Element::Geode as usize];
    //println!("Max: {}, Max node: {:?}", max_geodes, max_node);
    max_geodes
}

////////////////////////////////////////////////////////////////////////////////////

const ELEMENT_COUNT: usize = 4;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Element {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

type Robot = Element;

impl Element {
    fn from_index(index: UInt) -> Element {
        match index {
            0 => Element::Ore,
            1 => Element::Clay,
            2 => Element::Obsidian,
            3 => Element::Geode,
            _ => panic!("Invalid index"),
        }
    }

    fn iter() -> impl DoubleEndedIterator<Item = Element> {
        (0..ELEMENT_COUNT as UInt).map(Element::from_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    elements: [UInt; ELEMENT_COUNT],
    robots: [UInt; ELEMENT_COUNT],
    spent_minutes: UInt,
}

impl Node {
    fn new() -> Node {
        let mut node = Node {
            elements: [0; ELEMENT_COUNT],
            robots: [0; ELEMENT_COUNT],
            spent_minutes: 0,
        };
        node.robots[Element::Ore as usize] = 1;
        node
    }

    fn robots_collect(&mut self) {
        for i in 0..ELEMENT_COUNT {
            self.elements[i] += self.robots[i];
        }
    }

    fn buy_robot(&mut self, robot: Robot, blueprint: &Blueprint) {
        self.robots[robot as usize] += 1;
        for element in Element::iter() {
            self.elements[element as usize] -= blueprint.element_costs_for_robot(robot, element);
        }
    }

    fn can_buy_robot(&mut self, robot: Robot, blueprint: &Blueprint) -> bool {
        for element in Element::iter() {
            if self.elements[element as usize] < blueprint.element_costs_for_robot(robot, element) {
                return false;
            }
        }
        return true;
    }
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Blueprint {
    id: UInt,
    ore_robot_cost: UInt,              // = costs x ore
    clay_robot_cost: UInt,             // = costs x ore
    obsidian_robot_cost: (UInt, UInt), // = costs x ore, y clay
    geode_robot_cost: (UInt, UInt),    // = costs x ore, y obsidian
}

impl Blueprint {
    fn element_costs_for_robot(&self, robot: Robot, element: Element) -> UInt {
        match (robot, element) {
            (Robot::Ore, Element::Ore) => self.ore_robot_cost,
            (Robot::Clay, Element::Ore) => self.clay_robot_cost,
            (Robot::Obsidian, Element::Ore) => self.obsidian_robot_cost.0,
            (Robot::Obsidian, Element::Clay) => self.obsidian_robot_cost.1,
            (Robot::Geode, Element::Ore) => self.geode_robot_cost.0,
            (Robot::Geode, Element::Obsidian) => self.geode_robot_cost.1,
            _ => 0,
        }
    }
}

fn parse_blueprints(file_name: &str) -> Vec<Blueprint> {
    let mut input = vec![];
    for line in utils::file_to_lines(file_name) {
        let mut blueprint = vec![];
        let start_pos = line.find(": ").unwrap();
        let line = line
            .replace(".", "")
            .replace(" robot costs ", "")
            .replace("ore", "")
            .replace("clay", "")
            .replace("obsidian", "")
            .replace("geode", "");
        for elem in line[start_pos + 1..].split("Each ").skip(1) {
            if elem.contains(" and ") {
                blueprint.push(
                    elem.split(" and ")
                        .map(|x| utils::str_to(x.trim()))
                        .collect_tuple()
                        .unwrap(),
                );
            } else {
                blueprint.push((utils::str_to(elem.trim()), 0));
            }
        }

        let blueprint = Blueprint {
            id: input.len() as UInt + 1,
            ore_robot_cost: blueprint[0].0,
            clay_robot_cost: blueprint[1].0,
            obsidian_robot_cost: blueprint[2],
            geode_robot_cost: blueprint[3],
        };

        input.push(blueprint);
    }
    input
}

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 33);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1192);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 2604);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 14725);
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
