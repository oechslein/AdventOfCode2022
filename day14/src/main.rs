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

use std::{
    collections::{hash_set, HashSet},
    ops::{Range, RangeInclusive},
};

use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Coor2D, Neighborhood, Topology},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day14/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day14/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let (mut grid, rocks, sand_entry, max_rock_y) = parse(file_name, None);
    print_grid(&grid);

    let sand_count = simulate_sands(&mut grid, rocks, sand_entry, Some(max_rock_y), None);
    print_grid(&grid);

    sand_count
}

pub fn solve_part2(file_name: &str) -> usize {
    let floor_y_diff = 2;
    let (mut grid, rocks, sand_entry, max_rock_y) = parse(file_name, Some(floor_y_diff));
    print_grid(&grid);

    let sand_count = simulate_sands(
        &mut grid,
        rocks,
        sand_entry,
        None,
        Some(floor_y_diff + max_rock_y),
    );
    print_grid(&grid);

    sand_count + 1 // +1 for the sand entry position
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(
    file_name: &str,
    floor_y_diff: Option<usize>,
) -> (GridArray<char>, HashSet<Coor2D>, Coor2D, usize) {
    let rocks: HashSet<Coor2D> = utils::file_to_lines(file_name)
        .flat_map(|line| {
            line.split(" -> ")
                .map(|t| {
                    Coor2D::from_tuple(t.split(',').map(utils::str_to).collect_tuple().unwrap())
                })
                .tuple_windows()
                .flat_map(|(pos1, pos2)| {
                    assert!(pos1.x == pos2.x || pos1.y == pos2.y);
                    utils::inclusive_range_always(pos1.x, pos2.x)
                        .cartesian_product(utils::inclusive_range_always(pos1.y, pos2.y))
                        .map(Coor2D::from_tuple)
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect();
    let sand_entry = Coor2D::new(500, 0);
    let (min_x, max_x) = rocks
        .iter()
        .chain(vec![&sand_entry].into_iter())
        .map(|coor| coor.x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = rocks
        .iter()
        .chain(vec![&sand_entry].into_iter())
        .map(|coor| coor.y)
        .minmax()
        .into_option()
        .unwrap();
    let mut grid: GridArray<char> = GridArrayBuilder::default()
        .topology(Topology::Bounded)
        .neighborhood(Neighborhood::Square)
        .width(max_x + max_y + 1)
        .height(max_y + floor_y_diff.unwrap_or(0) + 1)
        .build()
        .unwrap();

    rocks.iter().for_each(|coor| {
        grid.set(coor.x, coor.y, '#');
    });
    grid.set(sand_entry.x, sand_entry.y, '+');
    (grid, rocks, sand_entry, max_y)
}

fn simulate_sands(
    grid: &mut GridArray<char>,
    mut solid_coors_set: HashSet<Coor2D>,
    sand_entry: Coor2D,
    max_rock_y: Option<usize>,
    floor_y: Option<usize>,
) -> usize {
    let mut sand_count = 0;
    loop {
        let sandpos = let_sand_fall(&sand_entry, &solid_coors_set, max_rock_y, floor_y);
        match sandpos {
            None => break,
            Some(sand_pos) if sand_pos == sand_entry => break,
            Some(sand_pos) => {
                grid.set(sand_pos.x, sand_pos.y, 'o');
                solid_coors_set.insert(sand_pos);
                sand_count += 1;
                //print_grid(&grid);
                //println!("");
            }
        }
    }
    sand_count
}

fn let_sand_fall(
    start_coor: &Coor2D,
    solid_coors_set: &HashSet<Coor2D>,
    max_rock_y: Option<usize>,
    floor_y: Option<usize>,
) -> Option<Coor2D> {
    let no_solid = |coor: Coor2D| {
        if solid_coors_set.contains(&coor) {
            return None;
        } else if floor_y.is_some() && coor.y as usize >= floor_y.unwrap() {
            return None;
        } else {
            return Some(coor);
        }
    };

    let mut curr_coor = start_coor.clone();
    loop {
        let next_coor = curr_coor.clone() + Coor2D::new(0, 1);
        if max_rock_y.is_some() && next_coor.y as usize > max_rock_y.unwrap() {
            return None;
        }
        if let Some(next_coor) = no_solid(next_coor.clone()) {
            curr_coor = next_coor;
            continue;
        }
        if let Some(next_coor) = no_solid(next_coor.clone() - Coor2D::new(1, 0)) {
            curr_coor = next_coor;
            continue;
        }
        if let Some(next_coor) = no_solid(next_coor.clone() + Coor2D::new(1, 0)) {
            curr_coor = next_coor;
            continue;
        }

        // all blocked
        return Some(curr_coor);
    }
}

fn print_grid(grid: &GridArray<char>) {
    let (min_corr, max_coor) = grid.all_cells().filter(|(_, ch)| ch != &&'\0').fold(
        (
            Coor2D::new(usize::MAX, usize::MAX),
            Coor2D::new(usize::MIN, usize::MIN),
        ),
        |(coor_min, coor_max), (coor, _)| (coor_min.min(coor.clone()), coor_max.max(coor)),
    );

    let (min_x, max_x) = min_corr.to_tuple();
    let (min_y, max_y) = max_coor.to_tuple();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let ch = grid.get_unchecked(x, y);
            if ch != &'\0' {
                print!("{}", ch);
            } else {
                print!(".");
            }
        }
        println!();
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
        assert_eq!(solve_part1("test.txt"), 24);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 885);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 93);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 28691);
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
