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
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

use std::collections::VecDeque;

use fxhash::FxHashSet;
use grid::grid_types::Direction;
use itertools::Itertools;
use pathfinding::prelude::astar;
use pathfinding::prelude::dijkstra;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day24/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day24/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

type CoorTyp = u8;

pub fn solve_part1(file_name: &str) -> usize {
    let mut valley = parse(file_name);
    (valley.x, valley.y) = valley.start_pos();
    //valley.print();
    //println!();

    let (goal_valley, steps_to_goal) = valley.find_path(valley.goal_pos());
    assert_eq!((goal_valley.x, goal_valley.y), valley.goal_pos());

    steps_to_goal
}

pub fn solve_part2(file_name: &str) -> usize {
    let mut valley = parse(file_name);
    (valley.x, valley.y) = valley.start_pos();
    //valley.print();
    //println!();

    // way to goal
    let (goal_valley, steps_to_goal) = valley.find_path(valley.goal_pos());
    assert_eq!((goal_valley.x, goal_valley.y), valley.goal_pos());

    // way back to start
    let (start_valley, steps_to_start) = goal_valley.find_path(valley.start_pos());
    assert_eq!((start_valley.x, start_valley.y), valley.start_pos());

    // way back to goal
    let (goal_valley, steps_to_goal2) = start_valley.find_path(valley.goal_pos());
    assert_eq!((goal_valley.x, goal_valley.y), valley.goal_pos());

    steps_to_goal + steps_to_start + steps_to_goal2
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    x: CoorTyp,
    y: CoorTyp,
    dir: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valley {
    x: CoorTyp,
    y: CoorTyp,
    width: CoorTyp,
    height: CoorTyp,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    fn find_path(&self, goal_pos: (CoorTyp, CoorTyp)) -> (Valley, usize) {
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
            |valley| (valley.x, valley.y) == goal_pos,
        );
        //println!("{result:?}");
        let (path, steps_to_goal) = result.expect("no path found");

        let final_valley = path.last().unwrap().clone();
        assert_eq!((final_valley.x, final_valley.y), goal_pos);
        (final_valley, steps_to_goal)
    }

    // move all blizzards and return for all possible new positions a clone of the valley with that position
    fn possible_valleys(&mut self) -> impl Iterator<Item = Valley> + '_ {
        self.go_blizzards().map(|(x, y)| {
            let mut new_valley = self.clone();
            new_valley.x = x;
            new_valley.y = y;
            new_valley
        })
    }

    // moves all blizzards are returns all new possible positions (including the current one)
    fn go_blizzards(&mut self) -> impl Iterator<Item = (CoorTyp, CoorTyp)> {
        let mut possible_next_positions = self._get_all_possible_next_positions();

        let width: isize = self.width.try_into().unwrap();
        let height: isize = self.height.try_into().unwrap();
        for blizzard in &mut self.blizzards {
            let diff = blizzard.dir.diff_coor();
            let mut new_x: isize = isize::try_from(blizzard.x).unwrap() + diff.x;
            let mut new_y: isize = isize::try_from(blizzard.y).unwrap() + diff.y;
            if new_x <= 0 {
                new_x = width - 2;
            } else if new_x >= width - 1 {
                new_x = 1;
            }
            if new_y <= 0 {
                new_y = height - 2;
            } else if new_y >= height - 1 {
                new_y = 1;
            }
            let new_x = new_x.try_into().unwrap();
            let new_y = new_y.try_into().unwrap();
            possible_next_positions.remove(&(new_x, new_y));
            blizzard.x = new_x;
            blizzard.y = new_y;
        }

        possible_next_positions.into_iter()
    }

    fn _get_all_possible_next_positions(&self) -> FxHashSet<(CoorTyp, CoorTyp)> {
        let mut possible_next_positions: FxHashSet<(CoorTyp, CoorTyp)> = [
            Direction::East,
            Direction::West,
            Direction::South,
            Direction::North,
        ]
        .into_iter()
        .map(|dir| dir.diff_coor())
        .map(|diff| {
            (
                isize::try_from(self.x).unwrap() + diff.x,
                isize::try_from(self.y).unwrap() + diff.y,
            )
        })
        .filter(|(new_x, new_y)| (*new_x >= 0 && *new_y >= 0)) // filter our negatives
        .map(|(new_x, new_y)| {
            // now convert to unsigned
            (
                CoorTyp::try_from(new_x).unwrap(),
                CoorTyp::try_from(new_y).unwrap(),
            )
        })
        .filter(|(new_x, new_y)| {
            // now filter border but not goal and start
            (*new_x > 0 && *new_x < self.width - 1 && *new_y > 0 && *new_y < self.height - 1)
                || (*new_x, *new_y) == self.goal_pos()
                || (*new_x, *new_y) == self.start_pos()
        })
        .collect();
        possible_next_positions.insert((self.x, self.y)); // add current position
        possible_next_positions
    }

    #[allow(clippy::unused_self)]
    fn start_pos(&self) -> (CoorTyp, CoorTyp) {
        (1, 0)
    }

    fn goal_pos(&self) -> (CoorTyp, CoorTyp) {
        (self.width - 2, self.height - 1)
    }

    fn goal_reached(&self) -> bool {
        (self.x, self.y) == self.goal_pos()
    }

    fn min_steps_to(&self, goal_pos: (u8, u8)) -> usize {
        (goal_pos.0 as usize).abs_diff(self.x as usize)
            + (goal_pos.1 as usize).abs_diff(self.y as usize)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let all_blizzards = self
                    .blizzards
                    .iter()
                    .filter(|b| b.x == x && b.y == y)
                    .collect_vec();
                if x == self.x && y == self.y {
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
                } else if (x, y) == self.goal_pos() || (x, y) == (1, 0) {
                    print!(".");
                } else if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
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

fn _test(mut valley: Valley) {
    let test_vec: Vec<(isize, isize)> = vec![
        (0, 1), // 1
        (0, 1),
        (0, 0),
        (0, -1),
        (1, 0), // 5
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 0), // 10
        (0, 0),
        (0, 1),
        (0, 1),
        (1, 0),
        (1, 0), // 15
        (1, 0),
        (0, 1),
        (0, 1),
    ];
    for (index, (diff_x, diff_y)) in test_vec.into_iter().enumerate() {
        let possible_valleys = valley.possible_valleys().collect_vec();
        println!(
            "{} : ({diff_x}, {diff_y}) -> {}",
            index + 1,
            possible_valleys.len()
        );
        valley.x = (isize::try_from(valley.x).unwrap() + diff_x)
            .try_into()
            .unwrap();
        valley.y = (isize::try_from(valley.y).unwrap() + diff_y)
            .try_into()
            .unwrap();
        valley.print();
        println!();
        assert!(
            possible_valleys.contains(&valley) || valley.goal_reached(),
            "{:?}",
            valley._get_all_possible_next_positions()
        );
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> Valley {
    let lines = utils::file_to_lines(file_name).collect_vec();
    let width: CoorTyp = lines[0].len().try_into().unwrap();
    let height: CoorTyp = lines.len().try_into().unwrap();
    let mut valley = Valley {
        x: 0,
        y: 0,
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
                valley.blizzards.push(Blizzard { x, y, dir });
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
        assert_eq!(solve_part1("test.txt"), 18);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 279);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 54);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 762);
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
