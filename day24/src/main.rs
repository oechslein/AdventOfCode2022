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

use fxhash::FxHashSet;
use grid::grid_types::{Coor2DMut, Direction};
use itertools::Itertools;
use pathfinding::prelude::astar;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day24/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day24/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let mut valley = parse(file_name);
    valley.coor = valley.start_pos();

    let (goal_valley, steps_to_goal) = valley.find_path(&valley.goal_pos());
    debug_assert_eq!(goal_valley.coor, valley.goal_pos());

    steps_to_goal
}

pub fn solve_part2(file_name: &str) -> usize {
    let mut valley = parse(file_name);
    valley.coor = valley.start_pos();

    // way to goal
    let (goal_valley, steps_to_goal) = valley.find_path(&valley.goal_pos());
    debug_assert_eq!(goal_valley.coor, valley.goal_pos());

    // way back to start
    let (start_valley, steps_to_start) = goal_valley.find_path(&valley.start_pos());
    debug_assert_eq!(start_valley.coor, valley.start_pos());

    // way back to goal
    let (goal_valley, steps_to_goal2) = start_valley.find_path(&valley.goal_pos());
    debug_assert_eq!(goal_valley.coor, valley.goal_pos());

    steps_to_goal + steps_to_start + steps_to_goal2
}

////////////////////////////////////////////////////////////////////////////////////

type CoorTyp = u8;
type Coor = Coor2DMut<CoorTyp>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    coor: Coor,
    dir: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valley {
    coor: Coor,
    width: CoorTyp,
    height: CoorTyp,
    blizzards: Vec<Blizzard>,
}

#[must_use]
fn to_isize_coor(coor: &Coor) -> Coor2DMut<isize> {
    Coor2DMut::new(coor.x.try_into().unwrap(), coor.y.try_into().unwrap())
}

fn from_isize_coor(coor: &Coor2DMut<isize>) -> Coor {
    Coor::new(coor.x.try_into().unwrap(), coor.y.try_into().unwrap())
}

impl Valley {
    fn find_path(&self, goal_pos: &Coor) -> (Valley, usize) {
        let result = astar(
            self,
            |curr_valley| {
                curr_valley
                    .clone()
                    .possible_valleys()
                    .map(|valley| (valley, 1))
                    .collect_vec()
            },
            |valley| valley.min_steps_to(goal_pos),
            |valley| valley.coor == *goal_pos,
        );
        let (mut path, steps_to_goal) = result.expect("no path found");

        let final_valley = path.pop().unwrap();
        debug_assert_eq!(final_valley.coor, *goal_pos);
        (final_valley, steps_to_goal)
    }

    // move all blizzards and return for all possible new positions a clone of the valley with that position
    fn possible_valleys(&mut self) -> impl Iterator<Item = Valley> + '_ {
        self.go_blizzards().map(|coor| {
            let mut new_valley = self.clone();
            new_valley.coor = coor;
            new_valley
        })
    }

    // moves all blizzards are returns all new possible positions (including the current one)
    fn go_blizzards(&mut self) -> impl Iterator<Item = Coor> {
        let mut possible_next_positions = self._get_all_possible_next_positions();

        let width: isize = self.width.try_into().unwrap();
        let height: isize = self.height.try_into().unwrap();
        for blizzard in &mut self.blizzards {
            let mut new_coor = to_isize_coor(&blizzard.coor) + blizzard.dir.diff_coor();
            if new_coor.x <= 0 {
                new_coor.x = width - 2;
            } else if new_coor.x >= width - 1 {
                new_coor.x = 1;
            }
            if new_coor.y <= 0 {
                new_coor.y = height - 2;
            } else if new_coor.y >= height - 1 {
                new_coor.y = 1;
            }
            let new_coor = from_isize_coor(&new_coor);
            possible_next_positions.remove(&new_coor);
            blizzard.coor = new_coor;
        }

        possible_next_positions.into_iter()
    }

    fn _get_all_possible_next_positions(&self) -> FxHashSet<Coor> {
        let mut possible_next_positions: FxHashSet<Coor> = [
            Direction::East,
            Direction::West,
            Direction::South,
            Direction::North,
        ]
        .into_iter()
        .map(|dir| to_isize_coor(&self.coor) + dir.diff_coor())
        .filter(|new_coor| (new_coor.x >= 0 && new_coor.y >= 0)) // filter our negatives
        .map(|new_coor| from_isize_coor(&new_coor))
        .filter(|new_coor| self.is_valid_pos(new_coor))
        .collect();
        possible_next_positions.insert(self.coor.clone()); // add current position in addition
        possible_next_positions
    }

    #[allow(clippy::unused_self)]
    fn start_pos(&self) -> Coor {
        Coor::new(1, 0)
    }

    fn goal_pos(&self) -> Coor {
        Coor::new(self.width - 2, self.height - 1)
    }

    fn is_valid_pos(&self, coor: &Coor) -> bool {
        // now filter border but not goal and start
        (coor.x > 0 && coor.x < self.width - 1 && coor.y > 0 && coor.y < self.height - 1)
            || *coor == self.goal_pos()
            || *coor == self.start_pos()
    }

    fn min_steps_to(&self, goal_pos: &Coor) -> usize {
        self.coor.manhattan_distance(goal_pos)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coor = Coor::new(x, y);
                let all_blizzards = self
                    .blizzards
                    .iter()
                    .filter(|b| b.coor == coor)
                    .collect_vec();
                if coor == self.coor {
                    print!("E");
                    debug_assert!(all_blizzards.is_empty());
                } else if all_blizzards.len() == 1 {
                    let blizzard = all_blizzards[0];
                    print!(
                        "{}",
                        match blizzard.dir {
                            Direction::East => '>',
                            Direction::West => '<',
                            Direction::North => '^',
                            Direction::South => 'v',
                            _ => panic!("Unknown direction"),
                        }
                    );
                } else if all_blizzards.len() > 1 {
                    print!("{:x}", all_blizzards.len());
                } else if !self.is_valid_pos(&coor) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> Valley {
    let lines = utils::file_to_lines(file_name).collect_vec();
    let width: CoorTyp = lines[0].len().try_into().unwrap();
    let height: CoorTyp = lines.len().try_into().unwrap();
    let mut valley = Valley {
        coor: Coor::new(0, 0),
        width,
        height,
        blizzards: Vec::new(),
    };
    for (y, line) in lines.into_iter().enumerate().skip(1) {
        for (x, c) in line.chars().enumerate() {
            let x = x.try_into().unwrap();
            let y = y.try_into().unwrap();
            if let Some(dir) = match c {
                '#' | '.' => None,
                '>' => Some(Direction::East),
                '<' => Some(Direction::West),
                '^' => Some(Direction::North),
                'v' => Some(Direction::South),
                _ => panic!("Unknown char: '{c}'"),
            } {
                valley.blizzards.push(Blizzard {
                    coor: Coor::new(x, y),
                    dir,
                });
            }
        }
    }
    valley
}

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        debug_assert_eq!(solve_part1("test.txt"), 18);
    }

    #[test]
    fn verify1() {
        debug_assert_eq!(solve_part1("input.txt"), 279);
    }

    #[test]
    fn test2() {
        debug_assert_eq!(solve_part2("test.txt"), 54);
    }

    #[test]
    fn verify2() {
        debug_assert_eq!(solve_part2("input.txt"), 762);
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
