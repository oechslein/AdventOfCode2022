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


use std::{collections::HashSet, fmt::Display};

use grid::{
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{Coor2DMut, Neighborhood},
};
use itertools::Itertools;

use gcollections::ops::*;
use interval::interval_set::*;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    //utils::with_measure("Part 1", || solve_part1("day15/test.txt", 10));
    utils::with_measure("Part 1", || solve_part1("day15/input.txt", 2000000));

    //utils::with_measure("Part 2", || solve_part2("day15/test.txt", 20));
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

pub fn solve_part2(file_name: &str, max_x: isize) -> isize {
    let input = parse_sensor_beacon_list(file_name);
    let full_interval = vec![(0, max_x)].to_interval_set();

    for row in (0..=max_x).rev() {
        let mut interval = Vec::new().to_interval_set();
        for (_index, (sensor, beacon)) in input.iter().enumerate() {
            interval = interval.union(&sensor_beacon_row_interval(sensor, beacon, row as isize));
        }

        let intersect = interval.intersection(&full_interval);
        if intersect != full_interval {
            let first_interval = intersect.iter().next().unwrap();
            let x = first_interval.upper() + 1;
            assert!(!intersect.contains(&x));
            return x * 4_000_000 + row;
        }
    }

    unreachable!()
}

fn sensor_beacon_row_interval(
    sensor: &Coor2DMut<isize>,
    beacon: &Coor2DMut<isize>,
    row: isize,
) -> IntervalSet<isize> {
    let radius = sensor.manhattan_distance(beacon) as isize;
    let offset = radius - (sensor.y - row).abs();
    if offset < 0 {
        IntervalSet::empty()
    } else {
        vec![(sensor.x - offset, sensor.x + offset)].to_interval_set()
    }
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

fn parse_sensor_beacon_list<T: Clone + Ord + Eq + Display>(
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

fn get_all_neighbors_within_for_row(
    _grid: &GridHashMap<char>,
    sensor_coor: &Coor2DMut<isize>,
    max_manhattan_distance: usize,
    row: isize,
) -> HashSet<Coor2DMut<isize>> {
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

    neighbors_within
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
