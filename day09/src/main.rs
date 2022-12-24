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

use fxhash::FxHashSet;
use itertools::Itertools;

use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, RgbImage};
use std::{fs::File, iter::repeat};

use derive_more::{Add, AddAssign, Constructor, Display, Sub, SubAssign};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day09/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day09/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

const SAVE_IMAGE: bool = false;
const GET_MINMAX: bool = true;

// minmax_x:  (-103, 55), minmax_y: (-6, 274)
const MINMAX_X: (isize, isize) = (-103, 55);
const MINMAX_Y: (isize, isize) = (-6, 274);
const VISIT_POINTS_LENGTH: usize = 500;
const MIN_COLOR_RED: usize = 100;

pub fn solve_part1(file_name: &str) -> usize {
    solve(file_name, 2, false)
}

pub fn solve_part2(file_name: &str) -> usize {
    solve(file_name, 10, SAVE_IMAGE)
}

fn solve(file_name: &str, amount_of_knots: usize, save_image: bool) -> usize {
    if cfg!(not(test)) && (save_image || GET_MINMAX) {
        let mut wurm = Wurm::new(amount_of_knots, Position::new(0, 0), save_image);
        wurm.apply_steps(parse_input_directions(file_name))
    } else {
        let mut wurm = WurmFast::new(amount_of_knots, Position::new(0, 0));
        wurm.apply_steps(parse_input_directions(file_name))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(
    Eq,
    PartialEq,
    Hash,
    PartialOrd,
    Clone,
    Debug,
    //    From,
    //    Into,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    //    Sum,
    Constructor,
    Display,
)]
//#[into(owned, ref, ref_mut)]
#[display(fmt = "({},{})", _0, _1)]
struct Position(isize, isize);

impl Position {
    fn abs(&self) -> Position {
        Position(self.0.abs(), self.1.abs())
    }

    fn signum(&self) -> Position {
        Position(self.0.signum(), self.1.signum())
    }
}

struct WurmFast {
    knots_vec: Vec<Position>,
    unique_visited_positions_tail: FxHashSet<Position>,
}

impl WurmFast {
    fn new(amount_of_knots: usize, start_pos: Position) -> Self {
        let mut unique_visited_positions_tail = FxHashSet::default();
        unique_visited_positions_tail.insert(start_pos.clone());
        Self {
            knots_vec: vec![start_pos; amount_of_knots],
            unique_visited_positions_tail,
        }
    }

    fn apply_steps(&mut self, parse_input_directions: impl Iterator<Item = Position>) -> usize {
        for direction in parse_input_directions {
            self.apply_step(direction);
        }
        self.unique_count()
    }

    fn apply_step(&mut self, direction: Position) {
        self.knots_vec[0] += direction;

        self.fixup_tail_positions();
        self.unique_visited_positions_tail
            .insert(self.knots_vec[self.knots_vec.len() - 1].clone());
    }

    fn fixup_tail_positions(&mut self) {
        for index_head in 0..self.knots_vec.len() - 1 {
            let index_tail = index_head + 1;
            let diff_pos = self.knots_vec[index_head].clone() - self.knots_vec[index_tail].clone();
            let diff_pos_abs = diff_pos.abs();
            if diff_pos_abs.0 > 1 || diff_pos_abs.1 > 1 {
                // not touching
                self.knots_vec[index_tail] += diff_pos.signum();
            }
        }
    }

    fn unique_count(&self) -> usize {
        self.unique_visited_positions_tail.len()
    }
}

struct Wurm<'a> {
    wurm: WurmFast,
    save_image: bool,
    visited_positions_head: Vec<Position>,
    visited_positions_tail: Vec<Position>,
    frame_vec: Vec<Frame<'a>>,
}

impl<'a> Wurm<'a> {
    fn new(amount_of_knots: usize, start_pos: Position, save_image: bool) -> Self {
        Self {
            wurm: WurmFast::new(amount_of_knots, start_pos.clone()),
            save_image: save_image && cfg!(not(test)),
            visited_positions_head: vec![start_pos.clone()],
            visited_positions_tail: vec![start_pos],
            frame_vec: vec![],
        }
    }

    fn apply_steps(&mut self, parse_input_directions: impl Iterator<Item = Position>) -> usize {
        for direction in parse_input_directions {
            self.apply_step(direction);
        }

        self.print_minmax();
        self.save_gif();

        self.wurm.unique_count()
    }

    fn apply_step(&mut self, direction: Position) {
        self.wurm.apply_step(direction);

        self.visited_positions_tail
            .push(self.wurm.knots_vec[self.wurm.knots_vec.len() - 1].clone());

        self.save_frame();
    }

    fn should_save_image(&self) -> bool {
        cfg!(not(test)) && self.save_image
    }

    fn should_print_minmax(&self) -> bool {
        cfg!(not(test)) && GET_MINMAX
    }

    fn save_frame(&mut self) {
        if self.should_print_minmax() || self.should_save_image() {
            self.visited_positions_head
                .push(self.wurm.knots_vec[0].clone());
        }

        if self.should_save_image() {
            let image_width = (-MINMAX_Y.0 + MINMAX_Y.1 + 1) as u32;
            let image_height = (-MINMAX_Y.0 + MINMAX_Y.1 + 1) as u32;

            let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
            for (index, pos) in self.visited_positions_tail.iter().rev().enumerate() {
                let color: u8 = if index > VISIT_POINTS_LENGTH {
                    MIN_COLOR_RED as u8
                } else {
                    ((VISIT_POINTS_LENGTH - index) * (255 - MIN_COLOR_RED) / VISIT_POINTS_LENGTH
                        + MIN_COLOR_RED) as u8
                };
                img.put_pixel(
                    (-MINMAX_X.0 + pos.0) as u32,
                    (-MINMAX_Y.0 + pos.1) as u32,
                    image::Rgb([color, color, color]),
                );
            }
            for (index, knot_pos) in self.wurm.knots_vec.iter().rev().enumerate() {
                let color_value = ((index + 1) * 255 / self.wurm.knots_vec.len()) as u8;
                img.put_pixel(
                    (-MINMAX_X.0 + knot_pos.0) as u32,
                    (-MINMAX_Y.0 + knot_pos.1) as u32,
                    image::Rgb([color_value, 0, 0]),
                );
            }

            self.frame_vec.push(Frame::from_rgb(
                image_width as u16,
                image_height as u16,
                &img.into_raw(),
            ));
        }
    }

    fn print_minmax(&self) {
        if self.should_print_minmax() {
            println!(
                "minmax_x: {:?}, minmax_y: {:?}",
                self.visited_positions_head.iter().map(|pos| pos.0).minmax(),
                self.visited_positions_head.iter().map(|pos| pos.1).minmax()
            );
        }
    }

    fn save_gif(&self) {
        if cfg!(not(test)) && self.save_image {
            println!("Saving image ....");
            let mut image = File::create(r"c:\temp\day09.gif").unwrap();
            let mut encoder = Encoder::new(
                &mut image,
                ((-MINMAX_X.0 + MINMAX_X.1 + 1) as usize) as u16,
                ((-MINMAX_Y.0 + MINMAX_Y.1 + 1) as usize) as u16,
                &[0xFF, 0xFF, 0xFF, 0, 0, 0],
            )
            .unwrap();
            encoder.set_repeat(Repeat::Finite(1)).unwrap();
            for frame in &self.frame_vec {
                encoder.write_frame(frame).unwrap();
            }
        };
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse_input(file_name: &str) -> impl Iterator<Item = (char, usize)> {
    let input = utils::file_to_string(file_name);
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            (
                direction.chars().next().unwrap(),
                distance.trim().parse::<usize>().unwrap(),
            )
        })
        .collect_vec()
        .into_iter()
}

fn parse_input_directions(file_name: &str) -> impl Iterator<Item = Position> {
    parse_input(file_name)
        .flat_map(|(direction, distance)| repeat(parse_direction(direction)).take(distance))
}

fn parse_direction(direction: char) -> Position {
    match direction {
        'R' => Position::new(1, 0),
        'L' => Position::new(-1, 0),
        'U' => Position::new(0, -1),
        'D' => Position::new(0, 1),
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn print_grid(pos_knots_vec: &[Position], min_pos: Position, max_pos: Position) {
    for y in min_pos.1..=max_pos.1 {
        for x in min_pos.0..=max_pos.0 {
            if let Some((index, _)) = pos_knots_vec
                .iter()
                .enumerate()
                .find(|(_, p)| **p == Position::new(x, y))
            {
                print!("{}", index);
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
