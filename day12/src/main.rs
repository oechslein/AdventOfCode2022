//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
#![feature(test)]
//#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]

use grid::grid_array::GridArray;
use grid::grid_types::{Coor2D, Neighborhood, Topology};

use pathfinding::prelude::dijkstra;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day12/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day12/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let grid = parse_grid(file_name);

    let goal_pos = &find_first_pos(&grid, 'E');
    let start_pos = &find_first_pos(&grid, 'S');
    let result = dijkstra(
        start_pos,
        |coor| get_successor(&grid, coor, get_weigth),
        |coor| coor == goal_pos,
    );
    //println!("{:?}", result);
    result.map(|result| result.1).unwrap()
}

pub fn solve_part2(file_name: &str) -> usize {
    let grid = parse_grid(file_name);

    // search from goal to any start pos
    let goal_pos = &find_first_pos(&grid, 'E');

    let result = dijkstra(
        goal_pos,
        |coor| get_successor(&grid, coor, get_weigth_reverse),
        |coor| {
            let c = *grid.get_unchecked(coor.x, coor.y);
            c == 'S' || c == 'a'
        },
    );
    //println!("{:?}", result);
    result.map(|result| result.1).unwrap()
}

////////////////////////////////////////////////////////////////////////////////////

#[allow(clippy::type_complexity)]
fn get_successor<'a>(
    grid: &'a GridArray<char>,
    coor: &Coor2D,
    weight_fn: fn(&(Coor2D, &char), (Coor2D, &char)) -> Option<(Coor2D, usize)>,
) -> impl IntoIterator<Item = (Coor2D, usize)> + 'a {
    let curr_cell = (coor.clone(), grid.get_unchecked(coor.x, coor.y));
    grid.neighborhood_cells(coor.x, coor.y)
        .filter_map(move |neighbor_cell| weight_fn(&curr_cell, neighbor_cell))
}

fn get_weigth(
    curr_cell: &(Coor2D, &char),
    neighbor_cell: (Coor2D, &char),
) -> Option<(Coor2D, usize)> {
    let (curr_cell_number, neighbor_cell_number) =
        (get_value(*curr_cell.1), get_value(*neighbor_cell.1));
    if (neighbor_cell_number <= curr_cell_number) || (neighbor_cell_number == curr_cell_number + 1)
    {
        Some((neighbor_cell.0, 1))
    } else {
        None
    }
}

fn get_weigth_reverse(
    curr_cell: &(Coor2D, &char),
    neighbor_cell: (Coor2D, &char),
) -> Option<(Coor2D, usize)> {
    let (curr_cell_number, neighbor_cell_number) =
        (get_value(*curr_cell.1), get_value(*neighbor_cell.1));
    if (neighbor_cell_number >= curr_cell_number) || (neighbor_cell_number == curr_cell_number - 1)
    {
        Some((neighbor_cell.0, 1))
    } else {
        None
    }
}

fn get_value(cell: char) -> u32 {
    match cell {
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        _ => cell as u32,
    }
}

fn find_first_pos(grid: &GridArray<char>, find_char: char) -> Coor2D {
    grid.all_cells().find(|(_, c)| **c == find_char).unwrap().0
}

fn parse_grid(file_name: &str) -> GridArray<char> {
    let input = utils::file_to_string(file_name);
    GridArray::from_newline_separated_string(Topology::Bounded, Neighborhood::Orthogonal, &input)
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 31);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 352);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 29);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 345);
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
