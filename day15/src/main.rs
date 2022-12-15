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

use std::collections::{HashSet, VecDeque};

use grid::{
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{Coor2DMut, Neighborhood, Topology},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    //utils::with_measure("Part 1", || solve_part1("day15/test.txt", 10));
    //utils::with_measure("Part 1", || solve_part1("day15/input.txt", 2000000));

    utils::with_measure("Part 2", || solve_part2("day15/test.txt", 20));
    utils::with_measure("Part 2", || solve_part2("day15/input.txt", 4000000));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str, row: isize) -> usize {
    let input = parse_sensor_beacon_list(file_name);
    let mut grid = create_grid(&input);

    //grid.print('.');
    //println!("");

    for (_index, (sensor, beacon)) in input.iter().enumerate() {
        let max_manhattan_distance = sensor.manhattan_distance(&beacon);

        // can this sensor / beacon combination influence the row?
        // row must be in reach of sensor + (distance between sensor and beacon)
        let sensor_row_distance = sensor.y - row;
        if sensor_row_distance.abs() > max_manhattan_distance as isize {
            continue;
        }

        // not all neighbors are relevant, only those that match row
        for neighboor in
            get_all_neighbors_within_for_row(&grid, &sensor, max_manhattan_distance, row)
        {
            if grid.get(&neighboor).is_none() || grid.get(&neighboor) == Some(&'.') {
                grid.set(neighboor, '#');
            }
        }
    }
    //grid.print('.');
    //println!("");

    grid.all_cells()
        .filter(|(coor, _)| coor.y == row)
        .filter(|(_, ch)| ch == &&'#')
        .count()
}

pub fn solve_part2(file_name: &str, max_x: i32) -> i64 {
    let input: Vec<(Coor2DMut<i32>, Coor2DMut<i32>)> = parse_sensor_beacon_list(file_name);

    // Rectangles (start corner and end corner, inclusive). Begin with one covering whole possible area
    let mut possibilities = vec![([-max_x, 0], [max_x, 2 * max_x])];
    let mut new_possibilities = Vec::new();
    for (sensor_coor, beacon_coor) in input {
        let radius = sensor_coor.manhattan_distance(&beacon_coor) as i32;

        // Coordinate system rotated by 45°. Only even coordinates in target system are integer in source system
        let center = Coor2DMut::new(sensor_coor.x - sensor_coor.y, sensor_coor.x + sensor_coor.y);
        let start = Coor2DMut::new(center.x - radius, center.y - radius).to_array();
        let end = Coor2DMut::new(center.x + radius, center.y + radius).to_array();

        for &p in &possibilities {
            let (p_start, p_end) = p;
            if !(0..2).all(|i| start[i] <= p_end[i] && p_start[i] <= end[i]) {
                new_possibilities.push(p);
            } else {
                if start[0] > p_start[0] {
                    new_possibilities.push((p_start, [start[0] - 1, p_end[1]]));
                }
                if p_end[0] > end[0] {
                    new_possibilities.push(([end[0] + 1, p_start[1]], p_end));
                }
                if start[1] > p_start[1] {
                    new_possibilities.push((
                        [std::cmp::max(start[0], p_start[0]), p_start[1]],
                        [std::cmp::min(end[0], p_end[0]), start[1] - 1],
                    ));
                }
                if p_end[1] > end[1] {
                    new_possibilities.push((
                        [std::cmp::max(start[0], p_start[0]), end[1] + 1],
                        [std::cmp::min(end[0], p_end[0]), p_end[1]],
                    ));
                }
            }
        }
        possibilities.clear();

        std::mem::swap(&mut possibilities, &mut new_possibilities);
    }

    // Assume there is a 1x1 rectangle somewhere within the allowed area
    for (start, end) in possibilities {
        if start == end && (start[0] + start[1]) % 2 == 0 {
            // Transform back into original coordinate system
            let pos = [(start[1] + start[0]) / 2, (start[1] - start[0]) / 2];
            if pos.iter().all(|&x| x >= 0 && x <= max_x) {
                return pos[0] as i64 * max_x as i64 + pos[1] as i64;
            }
        }
    }
    unreachable!("No solution found");
}

////////////////////////////////////////////////////////////////////////////////////

fn create_grid(input: &Vec<(Coor2DMut<isize>, Coor2DMut<isize>)>) -> GridHashMap<char> {
    let mut grid: GridHashMap<char> = GridHashMapBuilder::default()
        .neighborhood(Neighborhood::Orthogonal)
        .build()
        .unwrap();
    for (sensor, beacon) in input.iter() {
        grid.set(sensor.clone(), 'S');
        grid.set(beacon.clone(), 'B');
    }
    grid
}

fn parse_sensor_beacon_list<T: Clone + Ord + Eq>(
    file_name: &str,
) -> Vec<(Coor2DMut<T>, Coor2DMut<T>)>
where
    T: std::str::FromStr,
    <T>::Err: std::fmt::Debug,
{
    utils::file_to_lines(file_name)
        .map(|line| {
            let line = line
                .replace("Sensor at x=", "")
                .replace(": closest beacon is at x=", ",")
                .replace(", y=", ",");
            let (sensor_x, sensor_y, beacon_x, beacon_y) = line
                .split(",")
                .map(utils::str_to::<T>)
                .collect_tuple()
                .unwrap();
            let sensor = Coor2DMut::new(sensor_x, sensor_y);
            let beacon = Coor2DMut::new(beacon_x, beacon_y);
            (sensor, beacon)
        })
        .collect_vec()
}

fn get_all_neighbors_within_old(
    grid: &GridHashMap<char>,
    coor: &Coor2DMut<isize>,
    max_manhattan_distance: usize,
) -> HashSet<Coor2DMut<isize>> {
    let mut neighbors_within = HashSet::new();
    let mut open_neighbors = vec![];
    open_neighbors.push(coor.clone());

    while let Some(curr_coor) = open_neighbors.pop() {
        neighbors_within.insert(curr_coor.clone());
        for neighbor in grid.neighborhood_cell_indexes(&curr_coor) {
            if !neighbors_within.contains(&neighbor)
                && coor.manhattan_distance(&neighbor) <= max_manhattan_distance
            {
                open_neighbors.push(neighbor.clone());
            }
        }
    }
    neighbors_within
}

fn get_all_neighbors_within_for_row(
    _grid: &GridHashMap<char>,
    sensor_coor: &Coor2DMut<isize>,
    max_manhattan_distance: usize,
    row: isize,
) -> HashSet<Coor2DMut<isize>> {
    // search from coor.x, row to the left and to the right, stop is manhatten_distance is reached

    let mut neighbors_within = HashSet::new();
    let mut distance_to_sensor_x = 0;

    loop {
        if sensor_coor
            .manhattan_distance(&Coor2DMut::new(sensor_coor.x + distance_to_sensor_x, row))
            > max_manhattan_distance
        {
            break;
        }
        neighbors_within.insert(Coor2DMut::new(sensor_coor.x + distance_to_sensor_x, row));
        neighbors_within.insert(Coor2DMut::new(sensor_coor.x - distance_to_sensor_x, row));
        distance_to_sensor_x += 1;
    }

    /*
    println!("{:?}", neighbors_within.iter().collect_vec());
    println!(
        "{:?}",
        get_all_neighbors_within_old(_grid, sensor_coor, max_manhattan_distance)
            .into_iter()
            .filter(|c| c.y == row)
            .collect_vec()
    );
    */
    neighbors_within
    //get_all_neighbors_within_old(grid, coor, max_manhattan_distance)
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt", 10), 26);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt", 2000000), 5299855);
    }

    #[test]
    fn test2() {
        //assert_eq!(solve_part2("test.txt", 20), 56000011);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt", 4000000), 13615843289729);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt", 2000000));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt", 4000000));
    }
}
