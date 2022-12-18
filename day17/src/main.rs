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

use itertools::Itertools;
use rock::*;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day17/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day17/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

// test: 1*41=41
// input: 2*2*3*29*29 = 10092

const WIDTH: usize = 7;

struct Floor {
    _rock_number: usize,
    _z_height_array: [usize; WIDTH],
}

impl Floor {
    fn new() -> Self {
        Self {
            _rock_number: 0,
            _z_height_array: [0; WIDTH],
        }
    }
    fn get_z_height(&self, x: usize) -> usize {
        self._z_height_array[x]
    }

    fn set_z_height(&mut self, x: usize, new_z: usize) {
        self._z_height_array[x] = new_z;
    }

    fn get_z_height_max(&self) -> usize {
        self._z_height_array.iter().max().unwrap().clone()
    }

    fn generate_rock(&mut self, z_min: usize) -> RockEnum {
        let result = RockEnum::new(self._rock_number, z_min);
        self._rock_number = (self._rock_number + 1) % 5;
        result
    }

    fn would_touch_floor(&self, rock: &RockEnum) -> bool {
        for x in 0..WIDTH {
            if let Some(z_height_min) = rock.get_z_height_min(x) {
                if self.get_z_height(x) + 1 >= z_height_min {
                    return true;
                }
            }
        }
        return false;
    }

    fn draw(&self) {
        let max_z = self.get_z_height_max();
        for z in (1..=max_z).rev() {
            print!("|");
            for x in 0..WIDTH {
                if self.get_z_height(x) >= z {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+_______+");
    }

    fn draw_falling_rock(&self, rock: &RockEnum) {
        for z in (rock.get_total_z_height_min()..=rock.get_total_z_height_max()).rev() {
            print!("|");
            for x in 0..WIDTH {
                if rock.get_z_height_min(x) <= Some(z) && Some(z) <= rock.get_z_height_max(x) {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        let max_z = rock.get_total_z_height_min() - 1;
        for z in (1..=max_z).rev() {
            print!("|");
            for x in 0..WIDTH {
                if self.get_z_height(x) >= z {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
        println!("+_______+");
    }
}

pub fn solve_part1(file_name: &str) -> usize {
    let input = utils::file_to_string(file_name);
    let mut directions = input.chars().cycle();
    let mut floor = Floor::new();

    let limit = 2022;

    const DRAW_FALLING_ROCKS: bool = true;

    for _index in 0..limit {
        floor.draw();
        println!("");

        let curr_z_height = &floor.get_z_height_max();
        let mut current_rock = floor.generate_rock(curr_z_height + 4);
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

            match directions.next() {
                Some('>') => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push right");
                    }
                    // todo check if would bump into floor sideways!
                    current_rock.push_right()
                }
                Some('<') => {
                    if DRAW_FALLING_ROCKS {
                        println!("Push left");
                    }
                    // todo check if would bump into floor sideways!
                    current_rock.push_left()
                }
                ch => panic!("Unexpected end of input or wrong character: {:?}", ch),
            }
            if DRAW_FALLING_ROCKS {
                floor.draw_falling_rock(&current_rock);
                println!("");
            }

            if floor.would_touch_floor(&current_rock) {
                if DRAW_FALLING_ROCKS {
                    println!("Would touch floor!\n");
                }
                // save rock to floor
                for x in 0..WIDTH {
                    if let Some(z_height_max_rock) = current_rock.get_z_height_max(x) {
                        floor.set_z_height(x, z_height_max_rock);
                    }
                }
                if _index >= 4 {
                    return 42;
                }
                break;
            }

            if DRAW_FALLING_ROCKS {
                println!("Let fall");
            }
            current_rock.do_fall();
        }
    }

    42
}

pub fn solve_part2(file_name: &str) -> usize {
    let _input = utils::file_to_lines(file_name);
    42
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
        assert_eq!(solve_part1("test.txt"), 42);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 42);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 42);
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
