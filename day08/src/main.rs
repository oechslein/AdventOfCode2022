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

use std::iter::repeat;

use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Coor2D, Neighborhood, Topology},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day08/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day08/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    get_visible_trees(&create_forest_grid(file_name)).count()
}

pub fn solve_part2(file_name: &str) -> usize {
    calc_scenic_scores(&create_forest_grid(file_name))
        .into_iter()
        .map(|(.., score)| score)
        .max()
        .unwrap()
}

////////////////////////////////////////////////////////////////////////////////////

type MyGridArrayItemType = u8;
type MyGridArray = GridArray<MyGridArrayItemType>;

fn create_forest_grid(file_name: &str) -> MyGridArray {
    let vecs = utils::file_to_lines(file_name)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as MyGridArrayItemType)
                .collect_vec()
        })
        .collect_vec();
    let mut forest = GridArrayBuilder::default()
        .width(vecs[0].len())
        .height(vecs.len())
        .neighborhood(Neighborhood::Square)
        .topology(Topology::Bounded)
        .build()
        .unwrap();

    forest.set_from_vec(&vecs);
    //forest.print();
    forest
}

fn to_left_iter(coor: Coor2D, _width: usize, _height: usize) -> impl Iterator<Item = Coor2D> {
    (0..coor.x)
        .rev()
        .zip(repeat(coor.y))
        .map(Coor2D::from_tuple)
}

fn to_right_iter(coor: Coor2D, _width: usize, _height: usize) -> impl Iterator<Item = Coor2D> {
    (coor.x + 1.._width)
        .zip(repeat(coor.y))
        .map(Coor2D::from_tuple)
}

fn to_top_iter(coor: Coor2D, _width: usize, _height: usize) -> impl Iterator<Item = Coor2D> {
    repeat(coor.x)
        .zip((0..coor.y).rev())
        .map(Coor2D::from_tuple)
}

fn to_bottom_iter(coor: Coor2D, _width: usize, _height: usize) -> impl Iterator<Item = Coor2D> {
    repeat(coor.x)
        .zip(coor.y + 1.._height)
        .map(Coor2D::from_tuple)
}

////////////////////////////////////////////////////////////////////////////////////

fn get_visible_trees<'a>(forest: &'a MyGridArray) -> impl Iterator<Item = Coor2D> + 'a {
    forest
        .all_cells()
        .filter(move |(coor, tree_size)| {
            forest.is_edge(coor.x, coor.y)
                || all_smaller(
                    to_left_iter(coor.clone(), forest.get_width(), forest.get_height()),
                    &forest,
                    *tree_size,
                )
                || all_smaller(
                    to_right_iter(coor.clone(), forest.get_width(), forest.get_height()),
                    &forest,
                    *tree_size,
                )
                || all_smaller(
                    to_top_iter(coor.clone(), forest.get_width(), forest.get_height()),
                    &forest,
                    *tree_size,
                )
                || all_smaller(
                    to_bottom_iter(coor.clone(), forest.get_width(), forest.get_height()),
                    &forest,
                    *tree_size,
                )
        })
        .map(|(coor, _)| coor)
}

fn all_smaller(
    mut iter: impl Iterator<Item = Coor2D>,
    forest: &MyGridArray,
    tree_size: &MyGridArrayItemType,
) -> bool {
    iter.all(|coor2| forest.get(coor2.x, coor2.y).unwrap() < tree_size)
}

////////////////////////////////////////////////////////////////////////////////////

fn calc_scenic_scores<'a>(forest: &'a MyGridArray) -> impl Iterator<Item = (Coor2D, usize)> + 'a {
    forest.all_cells().map(move |(coor, tree_size)| {
        (
            coor.clone(),
            calc_scenic_score_x_y(forest, coor, *tree_size),
        )
    })
}

fn calc_scenic_score_x_y(
    forest: &MyGridArray,
    coor: Coor2D,
    tree_size: MyGridArrayItemType,
) -> usize {
    let scenic_score_left = amount_of_trees_visible_for_house(
        to_left_iter(coor.clone(), forest.get_width(), forest.get_height()),
        forest,
        tree_size,
    );
    let scenic_score_right = amount_of_trees_visible_for_house(
        to_right_iter(coor.clone(), forest.get_width(), forest.get_height()),
        forest,
        tree_size,
    );
    let scenic_score_top = amount_of_trees_visible_for_house(
        to_top_iter(coor.clone(), forest.get_width(), forest.get_height()),
        forest,
        tree_size,
    );
    let scenic_score_bottom = amount_of_trees_visible_for_house(
        to_bottom_iter(coor.clone(), forest.get_width(), forest.get_height()),
        forest,
        tree_size,
    );
    /*
    println!(
        "left: {:?}, right: {:?}, top: {:?}, bottom: {:?}",
        scenic_score_left, scenic_score_right, scenic_score_top, scenic_score_bottom
    );
     */

    scenic_score_left * scenic_score_right * scenic_score_top * scenic_score_bottom
}

fn amount_of_trees_visible_for_house(
    iter: impl Iterator<Item = Coor2D>,
    forest: &MyGridArray,
    tree_size: MyGridArrayItemType,
) -> usize {
    let mut amount_of_trees = 0;
    for coor2 in iter {
        let tree_size2 = forest.get(coor2.x, coor2.y).unwrap();
        amount_of_trees += 1;
        if tree_size2 >= &tree_size {
            break;
        }
    }
    amount_of_trees
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 21);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1809);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 8);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 479400);
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
