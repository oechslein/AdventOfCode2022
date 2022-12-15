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
    utils::with_measure("Part 1", || solve_part1("day15/test.txt", 10));
    //utils::with_measure("Part 1", || solve_part1("day15/input.txt", 2000000));

    //utils::with_measure("Part 2", || solve_part2("day15/test.txt", 20));
    utils::with_measure("Part 2", || solve_part2("day15/test.txt", 4000000));
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

pub fn solve_part2(file_name: &str, max_x: isize) -> usize {
    let input = parse_sensor_beacon_list(file_name);
    let mut grid = create_grid(&input);

    (0..=max_x)
        .cartesian_product(0..=max_x)
        .map(|(x, y)| Coor2DMut::new(x, y))
        .for_each(|coor| {
            if grid.get(&coor).is_none() {
                grid.set(coor, '.');
            }
        });

    //grid.print('.');
    //println!("");

    for (_index, (sensor, beacon)) in input.iter().enumerate() {
        let max_manhattan_distance = sensor.manhattan_distance(&beacon);
        println!("{}: {:?} {:?}", _index, sensor, beacon);

        // can this sensor / beacon combination influence the row?
        // row must be in reach of sensor + (distance between sensor and beacon)
        if (0..max_x).map(|y| sensor.y - y).all(|diff_y| diff_y.abs() > max_manhattan_distance as isize) {
            continue;
        }
        /*
        let sensor_row_distance = sensor.y - max_x;
        if sensor_row_distance.abs() > max_manhattan_distance as isize {
            continue;
        }
         */

        for row in 0..=max_x {
            // not all neighbors are relevant, only those that match row
            for neighboor in
                get_all_neighbors_within_for_row(&grid, &sensor, max_manhattan_distance, row)
            {
                match grid.get(&neighboor) {
                    None => {grid.set(neighboor, '#');},
                    Some('.') => {grid.set(neighboor, '#');},
                    _ => {}
                }
            }
        }
    }
    //grid.print('x');
    //println!("");

    let x = (0..=max_x)
        .cartesian_product(0..=max_x)
        .map(|(x, y)| Coor2DMut::new(x, y))
        .filter(|coor| grid.get(coor) == Some(&'.'))
        .collect_vec();
    println!("{:?}", x);

    grid.all_cells()
        .filter(|(coor, _)| coor.y == max_x)
        .filter(|(_, ch)| ch == &&'#')
        .count()
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

fn parse_sensor_beacon_list(file_name: &str) -> Vec<(Coor2DMut<isize>, Coor2DMut<isize>)> {
    utils::file_to_lines(file_name)
        .map(|line| {
            let line = line
                .replace("Sensor at x=", "")
                .replace(": closest beacon is at x=", ",")
                .replace(", y=", ",");
            let (sensor_x, sensor_y, beacon_x, beacon_y) = line
                .split(",")
                .map(utils::str_to::<isize>)
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
        get_all_neighbors_within_old(grid, sensor_coor, max_manhattan_distance)
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
        assert_eq!(solve_part1("input.txt", 2000000), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt", 20), 42);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt", 4000000), 42);
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
