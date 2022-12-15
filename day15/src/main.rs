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
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Coor2DMut, Neighborhood, Topology},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day15/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day15/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

type Coor2D_isize = Coor2DMut<isize>;
type Coor2D_usize = Coor2DMut<usize>;

pub fn solve_part1(file_name: &str) -> usize {
    let (mut min_coor, mut max_coor) = (
        Coor2D_isize::new(isize::MAX, isize::MAX),
        Coor2D_isize::new(isize::MIN, isize::MIN),
    );
    let input = utils::file_to_lines(file_name)
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
            let sensor = Coor2D_isize::new(sensor_x, sensor_y);
            let beacon = Coor2D_isize::new(beacon_x, beacon_y);
            min_coor = min_coor.min(&sensor).min(&beacon);
            max_coor = max_coor.max(&sensor).max(&beacon);
            (sensor, beacon)
        })
        .collect_vec();

    let enlarge_x: usize = ((max_coor.x - min_coor.x) / 2 + 1) as usize;
    let enlarge_y: usize = ((max_coor.y - min_coor.y) / 2 + 1) as usize;

    let transform_fn = |coor: &Coor2D_isize| -> Coor2D_usize {
        Coor2D_usize::new(
            (coor.x - min_coor.x) as usize + enlarge_x,
            (coor.y - min_coor.y) as usize + enlarge_y,
        )
    };

    let transform_back_fn = |coor: &Coor2D_usize| -> Coor2D_isize {
        Coor2D_isize::new(
            coor.x as isize + min_coor.x - enlarge_x as isize,
            coor.y as isize + min_coor.y - enlarge_y as isize,
        )
    };

    assert_eq!(
        (enlarge_x, enlarge_y),
        transform_fn(&min_coor).to_tuple(),
        "test 1"
    );
    assert_eq!(
        transform_back_fn(&Coor2D_usize::new(enlarge_x, enlarge_y)),
        min_coor,
        "test 2"
    );

    println!(
        "{} {} {} {}",
        min_coor,
        max_coor,
        transform_fn(&min_coor),
        transform_fn(&max_coor)
    );

    let mut grid: GridArray<char> = GridArrayBuilder::default()
        .topology(Topology::Bounded)
        .neighborhood(Neighborhood::Orthogonal)
        .width(transform_fn(&max_coor).x + enlarge_x + 1)
        .height(transform_fn(&max_coor).y + enlarge_y + 1)
        .build()
        .unwrap();

    for coor in grid.all_indexes() {
        grid.set(coor.x, coor.y, '.');
    }

    for (sensor, beacon) in input.iter() {
        let transformed_sensor = transform_fn(sensor);
        let transformed_beacon = transform_fn(beacon);
        grid.set(transformed_sensor.x, transformed_sensor.y, 'S');
        grid.set(transformed_beacon.x, transformed_beacon.y, 'B');
    }
    grid.print(false);
    println!("");

    {
        let sensor = Coor2D_isize::new(8, 7);
        let beacon = Coor2D_isize::new(2, 10);
        for transformed_neighboor in get_all_neighbors_within(
            &grid,
            &transform_fn(&sensor),
            sensor.manhattan_distance(&beacon),
        ) {
            println!(
                "{} {}",
                transform_back_fn(&transformed_neighboor),
                grid.get_unchecked(transformed_neighboor.x, transformed_neighboor.y)
            );
            if grid.get_unchecked(transformed_neighboor.x, transformed_neighboor.y) == &'.' {
                grid.set(transformed_neighboor.x, transformed_neighboor.y, '#');
            }
        }
    }

    grid.print(false);
    println!("");

    for (sensor, beacon) in input.iter() {
        for transformed_neighboor in get_all_neighbors_within(
            &grid,
            &transform_fn(&sensor),
            sensor.manhattan_distance(&beacon),
        ) {
            if grid.get_unchecked(transformed_neighboor.x, transformed_neighboor.y) == &'.' {
                grid.set(transformed_neighboor.x, transformed_neighboor.y, '#');
            }
        }
    }
    grid.print(false);

    let row = transform_fn(&Coor2D_isize::new(0, 10)).y;
    grid.all_cells().filter(|(coor, _)| coor.y == row).filter(|(_, ch)| ch == &&'#').count()
}

fn get_all_neighbors_within(
    grid: &GridArray<char>,
    coor: &Coor2D_usize,
    max_manhattan_distance: usize,
) -> HashSet<Coor2D_usize> {
    let mut neighbors_within = HashSet::new();
    let mut open_neighbors = vec![];
    open_neighbors.push(coor.clone());

    while let Some(curr_coor) = open_neighbors.pop() {
        neighbors_within.insert(curr_coor.clone());
        for neighbor in grid.neighborhood_cell_indexes(curr_coor.x, curr_coor.y) {
            if !neighbors_within.contains(&neighbor)
                && coor.manhattan_distance(&curr_coor) < max_manhattan_distance
            {
                open_neighbors.push(neighbor.clone());
            }
        }
    }
    neighbors_within
}

pub fn solve_part2(file_name: &str) -> usize {
    let _input = utils::file_to_lines(file_name);
    42
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 26);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 42);
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
