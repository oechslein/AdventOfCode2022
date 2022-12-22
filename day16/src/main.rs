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

use fxhash::FxHashMap;
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

pub fn solve_part1(file_name: &str) -> usize {
    let tunnel_system = TunnelSystem::new(30, parse(file_name));
    tunnel_system.get_max_pressure(tunnel_system.valves_with_flow.clone())
}

pub fn solve_part2(file_name: &str) -> usize {
    let tunnel_system = TunnelSystem::new(26, parse(file_name));

    // you have two persons now that can work in parallel, call get_max_pressure twice with every possible split of the valves
    tunnel_system
        .create_splits()
        .par_iter() // parallelize
        .cloned()
        .map(|(valve_set_1, valve_set_2)| {
            tunnel_system.get_max_pressure(valve_set_1)
                + tunnel_system.get_max_pressure(valve_set_2)
        })
        .max()
        .unwrap()
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

type ValveId = usize;
type ValveIdMap = FxHashMap<ValveId, Valve>;
type DistanceHashMap = FxHashMap<(ValveId, ValveId), usize>;
type ValveIdVec = Vec<ValveId>;

#[derive(Debug, Clone)]
struct Valve {
    id: ValveId,
    flow_rate: usize,
    tunnels: Vec<ValveId>,
}

impl Eq for Valve {}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl core::hash::Hash for Valve {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Valve {
    fn valve_string_to_number(valve: &str) -> ValveId {
        let mut result = 0;
        for c in valve.chars() {
            result = result * 26 + (c as usize - 'A' as usize + 1);
        }
        result as ValveId
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

struct TunnelSystem {
    limit: usize,
    valves: ValveIdMap,
    valves_with_flow: ValveIdVec,
    valve_to_valve_distances: DistanceHashMap,
}

impl TunnelSystem {
    fn new(limit: usize, valves: ValveIdMap) -> Self {
        let valves_with_flow = Self::_create_valves_with_flow(&valves);

        let valve_to_valve_distances =
            Self::_create_valve_to_valve_distances(&valves, &valves_with_flow);

        TunnelSystem {
            limit,
            valves,
            valves_with_flow,
            valve_to_valve_distances,
        }
    }

    fn _create_valves_with_flow(valves: &ValveIdMap) -> ValveIdVec {
        valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(k, _)| k.clone())
            .collect_vec()
    }

    fn _create_valve_to_valve_distances(
        valves: &ValveIdMap,
        valves_with_flow: &ValveIdVec,
    ) -> DistanceHashMap {
        let start_node = Valve::valve_string_to_number("AA");
        valves_with_flow
            .iter()
            .chain(vec![&start_node].into_iter())
            .cartesian_product(valves_with_flow.iter())
            .map(|(start_valve, goal_valve)| {
                let result = dijkstra(
                    start_valve,
                    |node: &ValveId| -> Vec<(ValveId, usize)> {
                        valves[node].tunnels.iter().map(|t| (*t, 1)).collect_vec()
                    },
                    |node| node == goal_valve,
                );
                ((*start_valve, *goal_valve), result.unwrap().1)
            })
            .collect()
    }

    fn create_splits(&self) -> Vec<(Vec<ValveId>, Vec<ValveId>)> {
        // only need to split until half since the other half is the same (mirrored)
        (0..=self.valves_with_flow.len() / 2)
            .flat_map(move |i| self._create_i_sized_splits(i))
            .collect_vec()
    }

    fn _create_i_sized_splits(&self, i: usize) -> Vec<(Vec<ValveId>, Vec<ValveId>)> {
        self.valves_with_flow
            .iter()
            .cloned()
            .combinations(i)
            .map(move |valve_set_1| {
                let valve_set_2 = self
                    .valves_with_flow
                    .iter()
                    .cloned()
                    .filter(|v| !valve_set_1.contains(v))
                    .collect_vec();
                (valve_set_1, valve_set_2)
            })
            .collect_vec()
    }

    fn get_max_pressure(&self, remaining: ValveIdVec) -> usize {
        let start_node = Node {
            tunnel: Valve::valve_string_to_number("AA"),
            time: 0,
            pressure: 0,
            flow: 0,
            remaining: remaining,
        };
        start_node
            .get_max_pressure_rec(&self.valve_to_valve_distances, &self.valves, self.limit)
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Node {
    tunnel: ValveId,
    time: usize,
    pressure: usize,
    flow: usize,
    remaining: ValveIdVec,
}

impl Node {
    fn get_max_pressure_rec(
        &self,
        valve_shorted_pathes: &DistanceHashMap,
        valves: &ValveIdMap,
        limit: usize,
    ) -> Option<usize> {
        let pressure_at_end = self.pressure + (limit - self.time) * self.flow;
        let max_pressure_rec = self
            .remaining
            .iter()
            .filter_map(|new_tunnel| {
                self.get_max_pressure_rec_for_tunnel(
                    *new_tunnel,
                    valve_shorted_pathes,
                    valves,
                    limit,
                )
            })
            .max();
        Some(pressure_at_end.max(max_pressure_rec.unwrap_or(0)))
    }

    fn get_max_pressure_rec_for_tunnel(
        &self,
        new_tunnel: ValveId,
        valve_shorted_pathes: &DistanceHashMap,
        valves: &ValveIdMap,
        limit: usize,
    ) -> Option<usize> {
        let needed_minutes = valve_shorted_pathes[&(self.tunnel, new_tunnel)] + 1;
        // + distance and +1 for open
        if self.time + needed_minutes > limit {
            None
        } else {
            let new_remaining = self
                .remaining
                .iter()
                .cloned()
                .filter(|v| *v != new_tunnel)
                .collect_vec();
            let new_node = Node {
                tunnel: new_tunnel.clone(),
                time: self.time + needed_minutes,
                pressure: self.pressure + needed_minutes * self.flow,
                flow: self.flow + valves[&new_tunnel].flow_rate,
                remaining: new_remaining,
            };
            new_node.get_max_pressure_rec(valve_shorted_pathes, valves, limit)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> ValveIdMap {
    let mut valves = ValveIdMap::default();
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
        let edges: Vec<usize> = edges
            .split(", ")
            .map(|valve| Valve::valve_string_to_number(valve))
            .collect_vec();
        let id = Valve::valve_string_to_number(node);
        valves.insert(
            id,
            Valve {
                id,
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
