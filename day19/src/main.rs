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

use std::collections::VecDeque;
use fxhash::FxHashSet;
use itertools::Itertools;
use rayon::prelude::*;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day19/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day19/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////
pub fn solve_part1(file_name: &str) -> usize {
    let blueprints = parse_blueprints(file_name);
    //println!("{:?}", blueprints);

    const LIMIT: usize = 24;

    blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * get_max_geodes(LIMIT, blueprint))
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let blueprints = parse_blueprints(file_name).into_iter().take(3).collect_vec();
    //println!("{:?}", blueprints);

    const LIMIT: usize = 32;

    blueprints
        .par_iter()
        .map(|blueprint| get_max_geodes(LIMIT, blueprint))
        .product()
}

////////////////////////////////////////////////////////////////////////////////////

fn get_max_geodes(limit: usize, blueprint: &Blueprint) -> usize {
    let mut max_geode_robot_per_time = vec![0; limit + 1];
    let mut visited_nodes = FxHashSet::default();
    let mut open_nodes = VecDeque::new();
    open_nodes.push_front(Node::new());
    let mut finished_nodes = vec![];
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

        if max_geode_robot_per_time[node.spent_minutes] > node.robots[Element::Geode as usize] {
            //println!("Pruned: {:?} {}", node, max_geode_robot_per_time[node.spent_minutes]);
            continue;
        }

        max_geode_robot_per_time[node.spent_minutes] = node.robots[Element::Geode as usize];

        node.spent_minutes += 1;

        // first check what robots are possible to buy
        let possible_robots = Element::iter()
            .filter(|robot| node.can_buy_robot(*robot, blueprint))
            .collect_vec();
        // then collect
        node.robots_collect();

        // then buy (if possible)
        // option buy robot if enough elements
        for robot in possible_robots {
            let mut new_node = node.clone();
            new_node.buy_robot(robot, blueprint);
            if !visited_nodes.contains(&new_node) {
                visited_nodes.insert(new_node.clone());
                open_nodes.push_back(new_node);
            } else {
                //println!("Already visited: {:?}", new_node);
            }
        }

        // option do nothing
        if !visited_nodes.contains(&node) {
            visited_nodes.insert(node.clone());
            open_nodes.push_back(node);
        } else {
            //println!("Already visited: {:?}", node);
        }

        // todo prune nodes buy removing nodes with same time but less geode robots
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
#[repr(usize)]
enum Element {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Element {
    fn from_index(index: usize) -> Element {
        match index {
            0 => Element::Ore,
            1 => Element::Clay,
            2 => Element::Obsidian,
            3 => Element::Geode,
            _ => panic!("Invalid index"),
        }
    }

    fn iter() -> impl Iterator<Item = Element> {
        (0..ELEMENT_COUNT).map(Element::from_index)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    elements: [usize; ELEMENT_COUNT],
    robots: [usize; ELEMENT_COUNT],
    spent_minutes: usize,
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

    fn buy_robot(&mut self, robot: Element, blueprint: &Blueprint) {
        self.robots[robot as usize] += 1;
        for element in Element::iter() {
            self.elements[element as usize] -= blueprint.elemen_costs_for_robot(robot, element);
        }
    }

    fn can_buy_robot(&mut self, robot: Element, blueprint: &Blueprint) -> bool {
        for element in Element::iter() {
            if self.elements[element as usize] < blueprint.elemen_costs_for_robot(robot, element) {
                return false;
            }
        }
        return true;
    }
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: usize,               // = costs x ore
    clay_robot_cost: usize,              // = costs x ore
    obsidian_robot_cost: (usize, usize), // = costs x ore, y clay
    geode_robot_cost: (usize, usize),    // = costs x ore, y obsidian
}

impl Blueprint {
    fn elemen_costs_for_robot(&self, robot: Element, element: Element) -> usize {
        match (robot, element) {
            (Element::Ore, Element::Ore) => self.ore_robot_cost,
            (Element::Clay, Element::Ore) => self.clay_robot_cost,
            (Element::Obsidian, Element::Ore) => self.obsidian_robot_cost.0,
            (Element::Obsidian, Element::Clay) => self.obsidian_robot_cost.1,
            (Element::Geode, Element::Ore) => self.geode_robot_cost.0,
            (Element::Geode, Element::Obsidian) => self.geode_robot_cost.1,
            _ => 0,
        }
    }
}

fn parse_blueprints(file_name: &str) -> Vec<Blueprint> {
    let mut input = vec![];
    for line in utils::file_to_lines(file_name) {
        let mut blueprint: Vec<(usize, usize)> = vec![];
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
            id: input.len() + 1,
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
