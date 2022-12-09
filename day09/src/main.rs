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

use itertools::Itertools;

use gif::{Encoder, Frame, Repeat};
use image::{ImageBuffer, RgbImage};
use std::{fs::File, iter::repeat};

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day09/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day09/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

const SAVE_IMAGE: bool = true;
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
    let mut wurm: Wurm = Wurm::new(amount_of_knots, (0, 0), save_image);
    wurm.apply_steps(parse_input_directions(file_name));
    wurm.unique_count()
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

type Position = (isize, isize);

struct Wurm<'a> {
    knots_vec: Vec<Position>,
    save_image: bool,
    visited_positions_head: Vec<Position>,
    visited_positions_tail: Vec<Position>,
    frame_vec: Vec<Frame<'a>>,
}

impl<'a> Wurm<'a> {
    fn new(amount_of_knots: usize, start_pos: Position, save_image: bool) -> Self {
        let knots_vec: Vec<Position> = vec![start_pos; amount_of_knots];
        let mut visited_positions_head = Vec::new();
        visited_positions_head.push(start_pos);
        let mut visited_positions_tail = Vec::with_capacity(20000);
        visited_positions_tail.push(start_pos);

        Self {
            knots_vec,
            save_image: save_image && cfg!(not(test)),
            visited_positions_head,
            visited_positions_tail,
            frame_vec: vec![],
        }
    }

    fn get_last_tail(&self) -> Position {
        self.knots_vec[self.knots_vec.len() - 1]
    }

    fn get_head(&self) -> Position {
        self.knots_vec[0]
    }

    fn apply_steps(&mut self, parse_input_directions: impl Iterator<Item = char>) {
        for direction in parse_input_directions {
            self.apply_step(direction);
        }
        self.print_minmax();
        self.save_gif();
    }

    fn apply_step(&mut self, direction: char) {
        self.knots_vec[0] = self.get_new_position(direction);

        self.fixup_tail_positions();
        self.visited_positions_tail.push(self.get_last_tail());

        self.save_frame();
    }

    fn get_new_position(&self, direction: char) -> Position {
        let (pos_head_x, pos_head_y) = self.get_head();
        match direction {
            'R' => (pos_head_x + 1, pos_head_y),
            'L' => (pos_head_x - 1, pos_head_y),
            'U' => (pos_head_x, pos_head_y - 1),
            'D' => (pos_head_x, pos_head_y + 1),
            _ => panic!("Unknown direction {}", direction),
        }
    }

    fn fixup_tail_positions(&mut self) {
        for index_head in 0..self.knots_vec.len() - 1 {
            let index_tail = index_head + 1;
            self.knots_vec[index_tail] =
                self.get_new_tail_pos(self.knots_vec[index_head], self.knots_vec[index_tail]);
        }
    }

    fn get_new_tail_pos(&self, head_pos: Position, tail_pos: Position) -> Position {
        if (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1 {
            // is touching
            tail_pos
        } else if head_pos.0 == tail_pos.0
            || head_pos.1 == tail_pos.1
            || (head_pos.0 - tail_pos.0).abs() == (head_pos.1 - tail_pos.1).abs()
        {
            // same row or same column or diagonal
            (
                head_pos.0 - (head_pos.0 - tail_pos.0).signum(),
                head_pos.1 - (head_pos.1 - tail_pos.1).signum(),
            )
        } else if (head_pos.0 - tail_pos.0).abs() < (head_pos.1 - tail_pos.1).abs() {
            // farer away in y direction
            (head_pos.0, head_pos.1 - (head_pos.1 - tail_pos.1).signum())
        } else {
            // farer away in x direction
            assert!(
                (head_pos.0 - tail_pos.0).abs() > (head_pos.1 - tail_pos.1).abs(),
                "pos_head: {:?}, pos_tail: {:?}",
                head_pos,
                tail_pos
            );
            (head_pos.0 - (head_pos.0 - tail_pos.0).signum(), head_pos.1)
        }
    }

    fn unique_count(&self) -> usize {
        self.visited_positions_tail.iter().unique().count()
    }

    fn should_save_image(&self) -> bool {
        cfg!(not(test)) && self.save_image
    }

    fn should_print_minmax(&self) -> bool {
        cfg!(not(test)) && GET_MINMAX
    }

    fn save_frame(&mut self) {
        if self.should_print_minmax() || self.should_save_image() {
            self.visited_positions_head.push(self.get_head());
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
            for (index, knot_pos) in (&self.knots_vec).iter().rev().enumerate() {
                let color_value = ((index + 1) * 255 / self.knots_vec.len()) as u8;
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
            for frame in self.frame_vec.iter() {
                encoder.write_frame(&frame).unwrap();
            }
        };
    }
}

fn parse_input_directions(file_name: &str) -> impl Iterator<Item = char> {
    parse_input(file_name).flat_map(|(direction, distance)| repeat(direction).take(distance))
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

#[allow(dead_code)]
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
