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

use std::collections::HashMap;

use rayon::prelude::*;

use itertools::Itertools;

use pathfinding::prelude::dijkstra;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day16/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day16/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

pub fn solve_part1(file_name: &str) -> isize {
    let valves = parse(file_name);
    //println!("{:?}", valves);

    let valves_with_flow = create_valves_with_flow_and_aa(&valves);
    let valve_shorted_pathes = create_valve_shorted_pathes(&valves);
    //println!("{} {:?}", valve_shorted_pathes.len(), valve_shorted_pathes);

    let limit = 30;
    let max_pressure = get_max_pressure(&valves_with_flow, limit, &valve_shorted_pathes, &valves);
    //println!("max_pressure: {} {:?}", max_pressure, max_elem);

    max_pressure
}

pub fn solve_part2(file_name: &str) -> isize {
    let valves = parse(file_name);
    //println!("{:?}", valves);

    let valves_with_flow = create_valves_with_flow_and_aa(&valves);
    let valve_shorted_pathes = create_valve_shorted_pathes(&valves);
    //println!("{} {:?}", valve_shorted_pathes.len(), valve_shorted_pathes);

    let limit = 26;
    // you have two persons now that can work in parallel, call get_max_pressure twice with every possible split of the valves
    create_splits(&valves_with_flow)
        .collect_vec()
        .iter() // parallelize
        .map(|(valve_set_1, valve_set_2)| {
            get_max_pressure(valve_set_1, limit, &valve_shorted_pathes, &valves)
                + get_max_pressure(valve_set_2, limit, &valve_shorted_pathes, &valves)
        })
        .max()
        .unwrap()
}

////////////////////////////////////////////////////////////////////////////////////

fn create_valve_shorted_pathes(
    valves: &HashMap<String, Valve>,
) -> HashMap<(String, String), isize> {
    let valves_with_flow = create_valves_with_flow_and_aa(valves);
    let mut valve_shorted_pathes = HashMap::new();
    for (start_valve, goal_valve) in valves_with_flow
        .iter()
        .cartesian_product(valves_with_flow.iter())
    {
        let result = dijkstra(
            start_valve,
            |node| {
                let curr_valve = &valves[*node];
                curr_valve.tunnels.iter().map(|t| (t, 1)).collect_vec()
            },
            |node| node == goal_valve,
        );
        valve_shorted_pathes.insert(
            (
                start_valve.clone().to_string(),
                goal_valve.clone().to_string(),
            ),
            result.unwrap().1,
        );
    }
    valve_shorted_pathes
}

fn create_valves_with_flow_and_aa(valves: &HashMap<String, Valve>) -> Vec<&String> {
    valves
        .iter()
        .filter(|(k, v)| v.flow_rate > 0 || k.as_str() == "AA")
        .map(|(k, _)| k)
        .collect_vec()
}

fn create_splits<'a>(
    valves_with_flow: &'a Vec<&'a String>,
) -> impl Iterator<Item = (Vec<&'a String>, Vec<&'a String>)> {
    // only need to split until half since the other half is the same (mirrored)
    (0..=valves_with_flow.len()).flat_map(move |i| create_i_sized_splits(&valves_with_flow, i))
}

fn create_i_sized_splits<'a>(
    valves_with_flow: &'a Vec<&'a String>,
    i: usize,
) -> impl Iterator<Item = (Vec<&'a String>, Vec<&'a String>)> {
    valves_with_flow
        .iter()
        .cloned()
        .combinations(i)
        .map(move |valve_set_1| {
            let valve_set_2 = valves_with_flow
                .iter()
                .cloned()
                .filter(|v| !valve_set_1.contains(v))
                .collect_vec();
            (valve_set_1, valve_set_2)
        })
}

fn get_max_pressure(
    remaining: &Vec<&String>,
    limit: isize,
    valve_shorted_pathes: &HashMap<(String, String), isize>,
    valves: &HashMap<String, Valve>,
) -> isize {
    let mut open = Vec::new();
    open.push(("AA".to_string(), 0, 0, 0, remaining.clone()));
    let mut max_pressure = 0;
    while let Some((curr_tunnel, curr_time, curr_pressure, curr_flow, remaining)) = open.pop() {
        let pressure_at_end = curr_pressure + (limit - curr_time) * curr_flow;
        if pressure_at_end > max_pressure {
            max_pressure = pressure_at_end;
        }
        for new_tunnel_index in 0..remaining.len() {
            let new_tunnel = remaining[new_tunnel_index];
            let needed_minutes =
                valve_shorted_pathes[&(curr_tunnel.clone(), new_tunnel.clone())] + 1; // distance and open
            if curr_time + needed_minutes <= limit {
                open.push((
                    new_tunnel.clone(),
                    curr_time + needed_minutes,
                    curr_pressure + needed_minutes * curr_flow,
                    curr_flow + valves[new_tunnel].flow_rate as isize,
                    remaining
                        .iter()
                        .filter(|v| **v != new_tunnel)
                        .cloned()
                        .collect_vec(),
                ));
            }
        }
    }
    max_pressure
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in utils::file_to_lines(file_name) {
        let line = line
            .replace("Valve ", "")
            .replace(" has flow rate", "")
            .replace(" tunnels lead to valves ", "")
            .replace(" tunnel leads to valve ", "");
        let (node_flow_rate, edges) = line.split(';').collect_tuple().unwrap();
        let (node, flow_rate) = node_flow_rate.split('=').collect_tuple().unwrap();
        //let node: [char; 2] = node.chars().collect_vec().try_into().unwrap();
        let flow_rate = flow_rate.parse::<usize>().unwrap();
        let edges: Vec<String> = edges
            .split(", ")
            .map(|valve| valve.to_string())
            .collect_vec();
        valves.insert(
            node.to_string(),
            Valve {
                name: node.to_string(),
                flow_rate: flow_rate,
                tunnels: edges,
            },
        );
    }
    valves
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 1651);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1741);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 1707);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 2316);
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
