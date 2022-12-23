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

use fxhash::FxHashSet;
use itertools::Itertools;
use rayon::prelude::*;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day18/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day18/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    count_unconnected_faces(&parse(file_name))
}

pub fn solve_part2(file_name: &str) -> usize {
    let cubes = parse(file_name);

    let minmax = get_minmax(&cubes);

    let holes = get_holes(&cubes, minmax);

    let trapped_holes = get_all_trapped_holes(&holes, &cubes, minmax);

    let mut reachable_faces = 0;
    for (cube_x, cube_y, cube_z) in &cubes {
        for (diff_x, diff_y, diff_z) in diff_iter() {
            let (x, y, z) = (cube_x + diff_x, cube_y + diff_y, cube_z + diff_z);
            if !trapped_holes.contains(&(x, y, z)) && !cubes.contains(&(x, y, z)) {
                reachable_faces += 1;
            }
        }
    }
    reachable_faces
}

////////////////////////////////////////////////////////////////////////////////////

fn get_holes(
    cubes: &FxHashSet<(isize, isize, isize)>,
    minmax: ((isize, isize, isize), (isize, isize, isize)),
) -> Vec<(isize, isize, isize)> {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = minmax;
    (min_x + 1..max_x)
        .cartesian_product(min_y + 1..max_y)
        .cartesian_product(min_z + 1..max_z)
        .map(|((x, y), z)| (x, y, z))
        .filter(|(x, y, z)| !cubes.contains(&(*x, *y, *z)))
        .collect()
}

fn get_all_trapped_holes(
    holes: &Vec<(isize, isize, isize)>,
    cubes: &FxHashSet<(isize, isize, isize)>,
    minmax: ((isize, isize, isize), (isize, isize, isize)),
) -> FxHashSet<(isize, isize, isize)> {
    holes
        .par_iter()
        .cloned()
        .filter(|hole| is_trapped(*hole, cubes, minmax))
        .collect()
}

fn is_trapped(
    hole: (isize, isize, isize),
    cubes: &FxHashSet<(isize, isize, isize)>,
    minmax: ((isize, isize, isize), (isize, isize, isize)),
) -> bool {
    let ((min_x, min_y, min_z), (max_x, max_y, max_z)) = minmax;

    // search all neighbors,
    // - if there is a cube, then it is not trapped
    // stop if neighbor is outside of the grid (min/max)

    let mut open_holes = vec![hole];
    let mut visited_holes = FxHashSet::default();
    while let Some(hole) = open_holes.pop() {
        for (diff_x, diff_y, diff_z) in diff_iter() {
            let (x, y, z) = (hole.0 + diff_x, hole.1 + diff_y, hole.2 + diff_z);
            if cubes.contains(&(x, y, z)) {
                continue;
            }
            if x <= min_x || x >= max_x || y <= min_y || y >= max_y || z <= min_z || z >= max_z {
                return false;
            }
            if !visited_holes.contains(&(x, y, z)) {
                open_holes.push((x, y, z));
            }
        }
        visited_holes.insert(hole);
    }

    true
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> FxHashSet<(isize, isize, isize)> {
    utils::file_to_lines(file_name)
        .map(|line| line.split(',').map(utils::str_to).collect_tuple().unwrap())
        .collect()
}

fn count_unconnected_faces(cubes: &FxHashSet<(isize, isize, isize)>) -> usize {
    let mut unconnected_faces = 0;
    for (cube_x, cube_y, cube_z) in cubes.iter() {
        for (diff_x, diff_y, diff_z) in diff_iter() {
            let (x, y, z) = (cube_x + diff_x, cube_y + diff_y, cube_z + diff_z);
            if (x, y, z) != (*cube_x, *cube_y, *cube_z) && !cubes.contains(&(x, y, z)) {
                unconnected_faces += 1;
            }
        }
    }
    unconnected_faces
}

////////////////////////////////////////////////////////////////////////////////////

fn get_minmax(
    cubes: &FxHashSet<(isize, isize, isize)>,
) -> ((isize, isize, isize), (isize, isize, isize)) {
    let minmax = cubes.iter().fold(
        (
            (isize::MAX, isize::MAX, isize::MAX),
            (isize::MIN, isize::MIN, isize::MIN),
        ),
        |((min_x, min_y, min_z), (max_x, max_y, max_z)), (x, y, z)| {
            (
                (min_x.min(*x), min_y.min(*y), min_z.min(*z)),
                (max_x.max(*x), max_y.max(*y), max_z.max(*z)),
            )
        },
    );
    minmax
}

fn diff_iter() -> impl Iterator<Item = (isize, isize, isize)> {
    vec![
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .into_iter()
}

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 64);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 3586);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 58);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 2072);
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
