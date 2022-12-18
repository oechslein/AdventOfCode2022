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

mod rock;

use std::{collections::HashMap, time::Instant};

use grid::{
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{Coor2D, Coor2DMut, Neighborhood},
};
use itertools::Itertools;
use rock::*;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    //utils::with_measure("Part 1", || solve_part1("day17/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day17/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

// test: 1*41=41
// input: 2*2*3*29*29 = 10092

const WIDTH: usize = 7;

struct Floor {
    _grid: GridHashMap<bool>,
    _rock_number: usize,
}

impl Floor {
    fn new() -> Self {
        Self {
            _rock_number: 0,
            _grid: GridHashMapBuilder::default()
                .neighborhood(Neighborhood::Square)
                .build()
                .unwrap(),
        }
    }

    fn get_coor(&self, coor: &Coor2D) -> bool {
        self.get(coor.x, coor.y)
    }

    fn get(&self, x: usize, y: usize) -> bool {
        *self
            ._grid
            .get(&Coor2DMut::<isize>::new(x as isize, y as isize))
            .unwrap_or(&false)
    }

    fn set(&mut self, x: usize, y: usize) {
        self._grid
            .set(Coor2DMut::<isize>::new(x as isize, y as isize), true);
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self._grid
            .remove(Coor2DMut::<isize>::new(x as isize, y as isize));
    }

    fn generate_rock(&mut self, y_min: usize) -> RockEnum {
        let result = RockEnum::new(self._rock_number, 2, y_min);
        self._rock_number = (self._rock_number + 1) % 5;
        result
    }

    fn draw(&self) {
        let max_y = self.get_min_max().1.y;
        for y in (self.get_y_min() + 1..=max_y).rev() {
            print!("|");
            for x in 0..WIDTH {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }

    fn draw_falling_rock(&self, rock: &RockEnum) {
        for y in (rock.get_y_min()..=rock.get_y_max()).rev() {
            print!("|");
            for x in 0..WIDTH {
                if rock.get(x, y) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        let max_y = rock.get_y_min() - 1;
        for y in (self.get_y_min() + 1..=max_y).rev() {
            print!("|");
            for x in 0..WIDTH {
                if self.get(x, y) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }

    pub fn get_min_max(&self) -> (Coor2D, Coor2D) {
        let (min_coor, max_coor) = self._grid.get_min_max();
        (
            Coor2D::new(min_coor.x as usize, min_coor.y as usize),
            Coor2D::new(max_coor.x as usize, max_coor.y as usize),
        )
    }

    pub fn get_y_max(&self) -> usize {
        let (_min_coor, max_coor) = self._grid.get_min_max();
        max_coor.y as usize
    }

    pub fn get_y_min(&self) -> usize {
        let (min_coor, _max_coor) = self._grid.get_min_max();
        min_coor.y as usize
    }

    pub fn add_rock(&mut self, rock: &RockEnum) {
        for y in rock.get_y_min()..=rock.get_y_max() {
            for x in 0..WIDTH {
                if rock.get(x, y) {
                    self._grid.set(Coor2DMut::new(x as isize, y as isize), true);
                }
            }
        }
    }

    pub fn do_collide(&self, rock: &RockEnum, delta_x: isize, delta_y: isize) -> bool {
        for y in rock.get_y_min()..=rock.get_y_max() {
            for x in rock.get_x_min()..=rock.get_x_max() {
                if rock.get(x, y) {
                    let new_x = x as isize + delta_x;
                    let new_y = y as isize + delta_y;
                    if new_x == -1 || new_x == WIDTH as isize {
                        return true;
                    }
                    if self.get(new_x as usize, new_y as usize) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn remove_unreachable_lines(&mut self, current_rock: &RockEnum) -> Option<usize> {
        let map: HashMap<usize, bool> = (current_rock.get_y_min()..=current_rock.get_y_max())
            .map(|y| (y, (0..WIDTH).all(|x| self.get(x, y))))
            .collect();

        let y_max_full_line_opt = vec![current_rock.get_y_min()]
            .into_iter()
            .cartesian_product(current_rock.get_y_min()..=current_rock.get_y_max())
            .filter(|(y_min, y_max)| (*y_min..=*y_max).all(|y| *map.get(&y).unwrap_or(&false)))
            .map(|(_y_min, y_max)| y_max)
            .max();
        if let Some(y_max_full_line) = y_max_full_line_opt {
            for y in 0..y_max_full_line {
                for x in 0..WIDTH {
                    self.remove(x, y);
                }
            }
        }
        y_max_full_line_opt
    }
}

pub fn solve_part1(file_name: &str) -> usize {
    solve(file_name, 2022)
}

pub fn solve_part2(file_name: &str) -> usize {
    solve(file_name, 1_000_000_000_000)
}

fn solve(file_name: &str, limit: usize) -> usize {
    const REMOVE_UNREACHABLE_LINES: bool = false;

    let mut start = Instant::now();

    let input = utils::file_to_string(file_name);
    let input_len = input.len();
    let mut directions = input.chars().enumerate().cycle();
    let mut floor = Floor::new();
    for x in 0..WIDTH {
        floor.set(x, 0);
    }

    const ROCK_AMOUNT: usize = 5;
    let mut last_cycle_rows: Vec<Vec<bool>> = vec![];
    let mut cycle_count = 0;
    const DRAW_FALLING_ROCKS: bool = false;
    const DRAW_FLOOR: bool = false;
    for _index in 0..limit {
        if DRAW_FLOOR {
            floor.draw();
            println!("");
        }

        if (_index as usize) % ROCK_AMOUNT == 0 {
            let cycle_row = (0..WIDTH)
                .cartesian_product((1.max(floor.get_y_max() as isize-53) as usize)..=floor.get_y_max())
                .map(|(x, y)| floor.get(x, y))
                .collect_vec();
            if let Some(pos) = last_cycle_rows.iter().position(|row| row == &cycle_row) {
                println!(
                    "cycle detected. _index: {}, floor.get_y_max(): {}, pos: {}",
                    _index,
                    floor.get_y_max(),
                    pos
                );
                floor.draw();
                println!("");
                } else {
                last_cycle_rows.push(cycle_row);
            }
        }
        
        /*
        if _index % (input_len * ROCK_AMOUNT) == 0 {
            println!("_index: {}", _index);
            let max_y = floor.get_min_max().1.y;
            for y in (max_y - 10 + 1..=max_y).rev() {
                print!("|");
                for x in 0..WIDTH {
                    if floor.get(x, y) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("|");
            }
            println!("+-------+");
        }
         */

        let curr_y_height = &floor.get_y_max();

        /*
        if curr_y_height % 53 == 0 {
            println!("_index: {}, curr_y_height {}, _index/35: {}, curr_y_height/53 {}", _index, curr_y_height/35, _index, curr_y_height/53);
            let max_y = floor.get_min_max().1.y;
            for y in (max_y - 10 + 1..=max_y).rev() {
                print!("|");
                for x in 0..WIDTH {
                    if floor.get(x, y) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("|");
            }
            println!("+-------+");
        }
         */

        let mut current_rock = floor.generate_rock(curr_y_height + 4);

        if DRAW_FALLING_ROCKS {
            println!(
                "Start new rock ==============================================================="
            );
        }
        loop {
            if DRAW_FALLING_ROCKS {
                floor.draw_falling_rock(&current_rock);
                println!("");
            }

            let (index_direction, direction) = directions.next().unwrap();
            match direction {
                '>' => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push right");
                    }
                    if !floor.do_collide(&current_rock, 1, 0) {
                        current_rock.move_x(1);
                    }
                }
                '<' => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push left");
                    }
                    if !floor.do_collide(&current_rock, -1, 0) {
                        current_rock.move_x(-1);
                    }
                }
                ch => panic!("Unexpected end of input or wrong character: {:?}", ch),
            }
            if DRAW_FALLING_ROCKS {
                floor.draw_falling_rock(&current_rock);
                println!("");
            }

            if floor.do_collide(&current_rock, 0, -1) {
                if DRAW_FALLING_ROCKS {
                    println!("Would touch floor!\n");
                }
                floor.add_rock(&current_rock);

                if REMOVE_UNREACHABLE_LINES {
                    if let Some(y_max_full_line) = floor.remove_unreachable_lines(&current_rock) {
                        floor.draw();
                        println!(
                        "y_max_full_line: {} floor.get_y_max(): {}, index_direction: {}, _index: {}",
                        y_max_full_line,
                        floor.get_y_max(),
                        index_direction,
                        _index
                    );
                    }
                }

                if _index % 100_000 == 0 {
                    // 1_000_000_000_000
                    floor.draw();
                    println!("{}: {}", _index, floor.get_y_max());
                    let duration = start.elapsed();
                    println!(
                        "Elapsed time is: {:?}, Total in h: {}",
                        duration,
                        duration.as_secs_f64() * limit as f64 / 100_000.0 / 60.0 / 60.0
                    );

                    start = Instant::now();
                }
                break;
            }

            if DRAW_FALLING_ROCKS {
                println!("Let fall");
            }

            current_rock.move_y(-1);
        }
    }
    //floor.draw();
    //println!("");
    floor.get_y_max()
}

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 3068);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 3114);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 1514285714288);
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
