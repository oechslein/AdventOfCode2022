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
    collections::{HashMap, HashSet},
    iter::repeat,
};

use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Coor, CoorIndex, Neighborhood, Topology},
};
use itertools::{chain, Itertools};
use utils::printlnit;

use gif::{Encoder, Frame, Repeat};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::fs::File;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day09/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day09/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

type Position = (isize, isize);

const DEBUG_PRINT: bool = if cfg!(test) { false } else { false };
const SAVE_IMAGE: bool = if cfg!(test) { false } else { true };
const GET_MINMAX: bool = if cfg!(test) { false } else { true };

// minmax_x:  (-103, 55), minmax_y: (-6, 274)
const MINMAX_X: (isize, isize) = (-103, 55);
const MINMAX_Y: (isize, isize) = (-6, 274);
const IMAGE_X_OFFSET: isize = -MINMAX_X.0;
const IMAGE_Y_OFFSET: isize = -MINMAX_Y.0;
const IMAGE_WIDTH: usize = (IMAGE_X_OFFSET + MINMAX_X.1 + 1) as usize;
const IMAGE_HEIGHT: usize = (IMAGE_Y_OFFSET + MINMAX_Y.1 + 1) as usize;
const VISIT_POINTS_LENGTH: usize = 500;
const MIN_COLOR_RED: usize = 100;

pub fn solve_part1(file_name: &str) -> usize {
    solve(2, file_name, false)
}

pub fn solve_part2(file_name: &str) -> usize {
    solve(10, file_name, SAVE_IMAGE)
}

fn solve(amount_of_knots: usize, file_name: &str, save_image: bool) -> usize {
    let mut frame_vec: Vec<Frame> = Vec::new();

    let mut pos_knots_vec: Vec<Position> = Vec::with_capacity(amount_of_knots);
    for _ in 0..amount_of_knots {
        pos_knots_vec.push((0, 0));
    }
    let mut visited_positions_head: Vec<Position> = Vec::new();
    visited_positions_head.push((0, 0));
    let mut visited_positions_tail: Vec<Position> = Vec::new();
    visited_positions_tail.push((0, 0));
    for (_line_index, (direction, distance)) in parse_input(file_name).enumerate() {
        for _index in 0..distance {
            apply_step(&mut pos_knots_vec[0], direction);
            for (index_heads, index_tail) in (0..pos_knots_vec.len()).zip(1..pos_knots_vec.len()) {
                fix_tail_position(pos_knots_vec[index_heads], &mut pos_knots_vec[index_tail]);

                if DEBUG_PRINT {
                    print_grid(&pos_knots_vec, (0, -6), (6, 0));
                }
            }
            if DEBUG_PRINT {
                println!("{}. {} -> {:?}", _index, direction, pos_knots_vec);
                print_grid(&pos_knots_vec, (0, -4), (5, 0));
                println!("");
            }

            if save_image {
                add_frame_to_image(
                    &visited_positions_tail,
                    &pos_knots_vec,
                    amount_of_knots,
                    &mut frame_vec,
                );
            }

            if GET_MINMAX || save_image {
                visited_positions_head.push(pos_knots_vec[0]);
            }
            visited_positions_tail.push(pos_knots_vec[pos_knots_vec.len() - 1]);
        }
    }

    if GET_MINMAX {
        println!(
            "minmax_x: {:?}, minmax_y: {:?}",
            visited_positions_head.iter().map(|pos| pos.0).minmax(),
            visited_positions_head.iter().map(|pos| pos.1).minmax()
        );
    }

    if save_image {
        save_gif(frame_vec);
    }

    visited_positions_tail.into_iter().unique().count()
}

fn add_frame_to_image(
    visited_positions_tail: &Vec<(isize, isize)>,
    pos_knots_vec: &Vec<(isize, isize)>,
    amount_of_knots: usize,
    frame_vec: &mut Vec<Frame>,
) {
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for (index, pos) in visited_positions_tail.iter().rev().enumerate() {
        let color: u8 = if index > VISIT_POINTS_LENGTH {
            MIN_COLOR_RED as u8
        } else {
            ((VISIT_POINTS_LENGTH - index) * (255 - MIN_COLOR_RED) / VISIT_POINTS_LENGTH
                + MIN_COLOR_RED) as u8
        };
        img.put_pixel(
            (IMAGE_X_OFFSET + pos.0) as u32,
            (IMAGE_Y_OFFSET + pos.1) as u32,
            image::Rgb([color, color, color]),
        );
    }
    for (index, knot_pos) in pos_knots_vec.iter().rev().enumerate() {
        let color_value = ((index + 1) * 255 / amount_of_knots) as u8;
        img.put_pixel(
            (IMAGE_X_OFFSET + knot_pos.0) as u32,
            (IMAGE_Y_OFFSET + knot_pos.1) as u32,
            image::Rgb([color_value, 0, 0]),
        );
    }
    let frame = Frame::from_rgb(IMAGE_WIDTH as u16, IMAGE_HEIGHT as u16, &img.into_raw());
    frame_vec.push(frame);
}

fn save_gif(frame_vec: Vec<Frame>) {
    println!("Saving image ....");
    let mut image = File::create(r"c:\temp\day09.gif").unwrap();
    let mut encoder = Encoder::new(
        &mut image,
        IMAGE_WIDTH as u16,
        IMAGE_HEIGHT as u16,
        &[0xFF, 0xFF, 0xFF, 0, 0, 0],
    )
    .unwrap();
    encoder.set_repeat(Repeat::Finite(1)).unwrap();
    for frame in frame_vec {
        encoder.write_frame(&frame).unwrap();
    }
}

fn print_grid(pos_knots_vec: &Vec<Position>, min_pos: Position, max_pos: Position) {
    for y in min_pos.1..=max_pos.1 {
        for x in min_pos.0..=max_pos.0 {
            if let Some((index, _)) = pos_knots_vec
                .iter()
                .enumerate()
                .filter(|(_, p)| **p == (x, y))
                .next()
            {
                print!("{}", index);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn apply_step(pos_head: &mut Position, direction: char) {
    match direction {
        'R' => pos_head.0 += 1,
        'L' => pos_head.0 -= 1,
        'U' => pos_head.1 -= 1,
        'D' => pos_head.1 += 1,
        _ => panic!("Unknown direction {}", direction),
    }
}

fn fix_tail_position(pos_head: Position, pos_tail: &mut Position) {
    if is_touching(pos_head, *pos_tail) {
        // no changes needed if touching
        return;
    }

    *pos_tail = {
        if pos_head.0 == pos_tail.0
            || pos_head.1 == pos_tail.1
            || (pos_head.0 - pos_tail.0).abs() == (pos_head.1 - pos_tail.1).abs()
        {
            // same row or same column or diagonal
            (
                pos_head.0 - (pos_head.0 - pos_tail.0).signum(),
                pos_head.1 - (pos_head.1 - pos_tail.1).signum(),
            )
        } else if (pos_head.0 - pos_tail.0).abs() < (pos_head.1 - pos_tail.1).abs() {
            // farer away in y direction
            (pos_head.0, pos_head.1 - (pos_head.1 - pos_tail.1).signum())
        } else {
            // farer away in x direction
            assert!(
                (pos_head.0 - pos_tail.0).abs() > (pos_head.1 - pos_tail.1).abs(),
                "pos_head: {:?}, pos_tail: {:?}",
                pos_head,
                pos_tail
            );
            (pos_head.0 - (pos_head.0 - pos_tail.0).signum(), pos_head.1)
        }
    };
}

fn is_touching(pos_head: Position, pos_tail: Position) -> bool {
    (pos_head.0 - pos_tail.0).abs() <= 1 && (pos_head.1 - pos_tail.1).abs() <= 1
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input(file_name: &str) -> impl Iterator<Item = (char, usize)> {
    utils::file_to_lines(file_name).map(|line| {
        let (direction, distance) = line.split_at(1);
        (
            direction.chars().next().unwrap(),
            distance.trim().parse::<usize>().unwrap(),
        )
    })
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 13);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 6236);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test2.txt"), 36);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 2449);
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
