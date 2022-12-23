//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
//#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::unreadable_literal)]

use std::collections::HashMap;

use fxhash::FxHashMap;
use grid::{
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{Coor2D, Coor2DMut, Neighborhood},
};

use crate::{
    rock::RockEnum, DRAW_FALLING_ROCKS, DRAW_FLOOR, REMOVE_UNREACHABLE_LINES, ROCK_AMOUNT, WIDTH,
};

use itertools::Itertools;

pub struct Floor {
    grid: GridHashMap<bool>,
    rock_index: usize,
    directions: Vec<char>,
    jet_direction_index: usize,
}

impl Floor {
    pub fn new(input: String) -> Self {
        Self {
            rock_index: 0,
            directions: input.chars().collect_vec(),
            jet_direction_index: 0,
            grid: GridHashMapBuilder::default()
                .neighborhood(Neighborhood::Square)
                .build()
                .unwrap(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        *self
            .grid
            .get(&Coor2DMut::<isize>::new(x as isize, y as isize))
            .unwrap_or(&false)
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.grid
            .set(Coor2DMut::<isize>::new(x as isize, y as isize), true);
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self.grid
            .remove(&Coor2DMut::<isize>::new(x as isize, y as isize));
    }

    pub fn next_rock(&mut self, y_min: usize) -> RockEnum {
        let result = RockEnum::new(self.rock_index, 2, y_min);
        self.rock_index = (self.rock_index + 1) % ROCK_AMOUNT;
        result
    }

    pub fn get_rock_index(&self) -> usize {
        self.rock_index
    }

    pub fn next_jet_direction(&mut self) -> char {
        let result = self.directions[self.jet_direction_index];
        self.jet_direction_index = (self.jet_direction_index + 1) % self.directions.len();
        result
    }

    pub fn get_jet_direction_index(&self) -> usize {
        self.jet_direction_index
    }

    pub fn draw(&self) {
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

    pub fn draw_falling_rock(&self, rock: &RockEnum) {
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
        let (min_coor, max_coor) = self.grid.get_min_max();
        (
            Coor2D::new(min_coor.x as usize, min_coor.y as usize),
            Coor2D::new(max_coor.x as usize, max_coor.y as usize),
        )
    }

    pub fn get_y_max(&self) -> usize {
        let (_min_coor, max_coor) = self.grid.get_min_max();
        max_coor.y as usize
    }

    pub fn get_y_min(&self) -> usize {
        let (min_coor, _max_coor) = self.grid.get_min_max();
        min_coor.y as usize
    }

    pub fn add_rock(&mut self, rock: &RockEnum) {
        for y in rock.get_y_min()..=rock.get_y_max() {
            for x in 0..WIDTH {
                if rock.get(x, y) {
                    self.grid.set(Coor2DMut::new(x as isize, y as isize), true);
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

    pub fn drop_rock(&mut self) {
        let curr_y_height = self.get_y_max();
        let mut current_rock = self.next_rock(curr_y_height + 4);
        if DRAW_FALLING_ROCKS {
            println!(
                "Start new rock ==============================================================="
            );
        }
        loop {
            if DRAW_FALLING_ROCKS {
                self.draw_falling_rock(&current_rock);
                println!();
            }

            match self.next_jet_direction() {
                '>' => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push right");
                    }
                    if !self.do_collide(&current_rock, 1, 0) {
                        current_rock.move_x(1);
                    }
                }
                '<' => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push left");
                    }
                    if !self.do_collide(&current_rock, -1, 0) {
                        current_rock.move_x(-1);
                    }
                }
                ch => panic!("Unexpected end of input or wrong character: {ch:?}"),
            }
            if DRAW_FALLING_ROCKS {
                self.draw_falling_rock(&current_rock);
                println!();
            }

            if self.do_collide(&current_rock, 0, -1) {
                if DRAW_FALLING_ROCKS {
                    println!("Would touch floor!\n");
                }
                self.add_rock(&current_rock);

                if REMOVE_UNREACHABLE_LINES {
                    self.remove_unreachable_lines(&current_rock);
                }
                break;
            }

            if DRAW_FALLING_ROCKS {
                println!("Let fall");
            }

            current_rock.move_y(-1);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    ceiling: [usize; WIDTH],
    rock_index: usize,
    jet_direction_index: usize,
}

impl Floor {
    pub fn solve(&mut self, limit: usize) -> usize {
        let mut cycle_detection_map = FxHashMap::default();
        for x in 0..WIDTH {
            self.set(x, 0);
        }
        for total_rock_index in 0..limit {
            if DRAW_FLOOR {
                self.draw();
                println!();
            }

            self.drop_rock();

            let curr_state = State {
                ceiling: self.get_ceiling(),
                rock_index: self.get_rock_index(),
                jet_direction_index: self.get_jet_direction_index(),
            };

            if let Some(old_state) = cycle_detection_map.get(&curr_state) {
                return self.follow_pattern(old_state, total_rock_index, limit);
            }

            cycle_detection_map.insert(curr_state, (self.get_y_max(), total_rock_index + 1));
        }
        //floor.draw();
        //println!("");
        self.get_y_max()
    }

    pub fn get_ceiling(&self) -> [usize; WIDTH] {
        let y_max = self.get_y_max();
        let mut ceiling = [0; WIDTH];
        for x in 0..WIDTH {
            for y in (0..=y_max).rev() {
                if self.get(x, y) {
                    ceiling[x] = y_max - y;
                    break;
                }
            }
        }
        ceiling
    }

    fn follow_pattern(
        &mut self,
        old_state: &(usize, usize),
        total_rock_index: usize,
        limit: usize,
    ) -> usize {
        let (old_y_max, old_total_rock_index) = old_state;
        let original_y_max = self.get_y_max();
        let mut new_y_max = original_y_max;
        let mut new_total_rock_index = total_rock_index + 1;
        let diff_y = new_y_max - *old_y_max;
        let diff_rock_index = new_total_rock_index - old_total_rock_index;
        let amount_of_full_cycles = (limit - new_total_rock_index) / diff_rock_index;
        new_total_rock_index += amount_of_full_cycles * diff_rock_index;
        new_y_max += amount_of_full_cycles * diff_y;
        while new_total_rock_index < limit {
            new_total_rock_index += 1;
            self.drop_rock();
        }
        new_y_max + (self.get_y_max() - original_y_max)
    }
}
