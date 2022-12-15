#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
#![feature(test)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]

use std::{collections::HashSet, fs::File};

use gif::{Encoder, Frame, Repeat};
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
    utils::with_measure("Part 2", || solve_part2("day14/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let sand_entry = Coor2D::new(500, 0);
    let (mut grid, rocks, max_rock_y) = parse(file_name, &sand_entry, None);
    //print_grid(&grid);

    let filename = create_image_filename(file_name, &sand_entry, 1);
    let sand_count = simulate_sands(
        &mut grid,
        rocks,
        &sand_entry,
        Some(max_rock_y),
        None,
        filename.as_str(),
    );
    //print_grid(&grid);

    sand_count
}

pub fn solve_part2(file_name: &str) -> usize {
    let floor_y_diff = 2;
    let sand_entry = Coor2D::new(500, 0);
    let (mut grid, rocks, max_rock_y) = parse(file_name, &sand_entry, Some(floor_y_diff));
    //print_grid(&grid);

    let filename = create_image_filename(file_name, &sand_entry, 2);
    let sand_count = simulate_sands(
        &mut grid,
        rocks,
        &sand_entry,
        None,
        Some(floor_y_diff + max_rock_y),
        filename.as_str(),
    );
    //print_grid(&grid);

    sand_count
}

////////////////////////////////////////////////////////////////////////////////////

fn simulate_sands(
    grid: &mut GridArray<char>,
    mut solid_coors_set: HashSet<Coor2D>,
    sand_entry: &Coor2D,
    max_rock_y: Option<usize>,
    floor_y: Option<usize>,
    file_path: &str,
) -> usize {
    let mut grid_vec = Vec::new();
    save_grid(grid, &mut grid_vec);
    let mut sand_count = 0;
    loop {
        let sandpos = let_sand_fall(
            grid,
            &sand_entry,
            &solid_coors_set,
            max_rock_y,
            floor_y,
            &mut grid_vec,
        );
        match sandpos {
            None => break,
            Some(sand_pos) if sand_pos == *sand_entry => {
                sand_count += 1;
                break;
            }
            Some(sand_pos) => {
                grid.set(sand_pos.x, sand_pos.y, 'o');
                solid_coors_set.insert(sand_pos);
                sand_count += 1;
            }
        }
        save_grid(grid, &mut grid_vec);
    }

    save_gif(&grid_vec, file_path);
    sand_count
}

fn let_sand_fall<'a>(
    grid: &mut GridArray<char>,
    start_coor: &Coor2D,
    solid_coors_set: &HashSet<Coor2D>,
    max_rock_y: Option<usize>,
    floor_y: Option<usize>,
    grid_vec: &mut Vec<GridArray<char>>,
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

    let mut _add_frame = |curr_coor: &Coor2D, next_coor: &Coor2D| {
        grid.set(curr_coor.x, curr_coor.y, '\0');
        grid.set(next_coor.x, next_coor.y, '+');
        grid.set(start_coor.x, start_coor.y, '+');
        save_grid(grid, grid_vec);
    };

    let mut curr_coor = start_coor.clone();
    loop {
        let next_coor = curr_coor.clone() + Coor2D::new(0, 1);
        if max_rock_y.is_some() && next_coor.y as usize > max_rock_y.unwrap() {
            return None;
        }
        if let Some(next_coor) = no_solid(next_coor.clone()) {
            #[cfg(not(test))]
            _add_frame(&curr_coor, &next_coor);

            curr_coor = next_coor;
            continue;
        }
        if let Some(next_coor) = no_solid(next_coor.clone() - Coor2D::new(1, 0)) {
            #[cfg(not(test))]
            _add_frame(&curr_coor, &next_coor);

            curr_coor = next_coor;
            continue;
        }
        if let Some(next_coor) = no_solid(next_coor.clone() + Coor2D::new(1, 0)) {
            #[cfg(not(test))]
            _add_frame(&curr_coor, &next_coor);

            curr_coor = next_coor;
            continue;
        }

        // all blocked
        return Some(curr_coor);
    }
}

fn get_minmax_nonempty(grid: &GridArray<char>) -> (Coor2D, Coor2D) {
    grid.all_cells().filter(|(_, ch)| ch != &&'\0').fold(
        (
            Coor2D::new(usize::MAX, usize::MAX),
            Coor2D::new(usize::MIN, usize::MIN),
        ),
        |(coor_min, coor_max), (coor, _)| (coor_min.min(&coor), coor_max.max(&coor)),
    )
}

///////////////////////////////////////////////////////////////////////////////////////

fn create_image_filename(file_name: &str, sand_entry: &Coor2D, part_number: usize) -> String {
    format!(
        r"C:\temp\{}_{}x{}_part{}.gif",
        file_name.to_string().replace("/", "_"),
        sand_entry.x,
        sand_entry.y,
        part_number
    )
}

fn parse(
    file_name: &str,
    sand_entry: &Coor2D,
    floor_y_diff: Option<usize>,
) -> (GridArray<char>, HashSet<Coor2D>, usize) {
    let rocks = parse_rock_data(file_name);
    let max_coor: Coor2D = rocks
        .iter()
        .fold(
            Coor2D::new(usize::MIN, usize::MIN),
            |acc: Coor2D, e: &Coor2D| acc.max(&e),
        )
        .max(&sand_entry);
    let mut grid: GridArray<char> = GridArrayBuilder::default()
        .topology(Topology::Bounded)
        .neighborhood(Neighborhood::Square)
        .width(max_coor.x + max_coor.y + 1)
        .height(max_coor.y + floor_y_diff.unwrap_or(0) + 1)
        .build()
        .unwrap();
    rocks.iter().for_each(|coor| {
        grid.set(coor.x, coor.y, '#');
    });
    grid.set(sand_entry.x, sand_entry.y, '+');
    (grid, rocks, max_coor.y)
}

fn parse_rock_data(file_name: &str) -> HashSet<Coor2D> {
    utils::file_to_lines(file_name)
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
                })
                .collect_vec()
        })
        .collect()
}

////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
fn print_grid(grid: &GridArray<char>) {
    if cfg!(test) {
        return;
    }
    let (min_coor, max_coor) = get_minmax_nonempty(grid);
    for y in min_coor.y..=max_coor.y {
        for x in min_coor.x..=max_coor.x {
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


fn save_grid(_grid: &GridArray<char>, _grid_vec: &mut Vec<GridArray<char>>) {
    #[cfg(not(test))]
    {
        _grid_vec.push(_grid.clone());
    }
}

fn save_gif<'a>(grid_vec: &Vec<GridArray<char>>, file_path: &str) {
    if cfg!(test) {
        return;
    }

    println!("Saving image to {} ....", file_path);

    let (total_min_coor, total_max_coor) = grid_vec.iter().map(get_minmax_nonempty).fold(
        (
            Coor2D::new(usize::MAX, usize::MAX),
            Coor2D::new(usize::MIN, usize::MIN),
        ),
        |(coor_min, coor_max), (min_coor, max_coor)| {
            (coor_min.min(&min_coor), coor_max.max(&max_coor))
        },
    );

    let image_width = (total_max_coor.x - total_min_coor.x + 1) as u16;
    let image_height = (total_max_coor.y - total_min_coor.y + 1) as u16;

    let mut image = File::create(file_path).unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        image_width,
        image_height,
        vec![
            vec![0, 0, 0],
            vec![160, 160, 160],
            vec![255, 217, 50],
            vec![255 / 2, 217 / 2, 50 / 2],
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>()
        .as_slice(),
    )
    .unwrap();
    encoder.set_repeat(Repeat::Finite(1)).unwrap();
    for grid in grid_vec {
        let (min_coor, max_coor) = get_minmax_nonempty(grid);
        let frame_width = (max_coor.x - min_coor.x + 1) as u16;
        let frame_height = (max_coor.y - min_coor.y + 1) as u16;

        let mut pixels: Vec<u8> = vec![0; (frame_width * frame_height) as usize];

        for y in min_coor.y..=max_coor.y {
            for x in min_coor.x..=max_coor.x {
                let image_x = x - min_coor.x;
                let image_y = y - min_coor.y;
                let index = image_x + image_y * frame_width as usize;
                let ch = grid.get_unchecked(x, y);
                if ch == &'#' || ch == &'o' || ch == &'+' {
                    (&mut pixels)[index] = match ch {
                        '#' => 1,
                        'o' => 2,
                        '+' => 3,
                        _ => unreachable!("ch: '{}'", ch),
                    };
                }
            }
        }

        let mut frame = Frame::from_indexed_pixels(frame_width, frame_height, &mut *pixels, None);
        frame.left = (min_coor.x - total_min_coor.x) as u16;
        frame.top = (min_coor.y - total_min_coor.y) as u16;
        encoder.write_frame(&frame).unwrap();
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
