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
use fxhash::FxHashMap;
use grid::{
    grid_hashmap::{GridHashMap, GridHashMapBuilder},
    grid_types::{Coor2DMut, Topology},
    grid_types::{Direction, Neighborhood},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day23/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day23/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

const PRINT_DEBUG: bool = false;

pub fn solve_part1(file_name: &str) -> usize {
    let mut grid = Grid::new(parse(file_name));

    if PRINT_DEBUG {
        grid.print(14, 14);
        println!();
    }

    for round in 0..10 {
        grid.simulate_round(round);
    }

    grid.calc_result()
}

pub fn solve_part2(file_name: &str) -> usize {
    let mut grid = Grid::new(parse(file_name));

    if PRINT_DEBUG {
        grid.print(14, 14);
        println!();
    }

    let mut round = 0;
    while !grid.simulate_round(round) {
        round += 1;
    }

    round + 1
}

////////////////////////////////////////////////////////////////////////////////////

struct Grid {
    grid: GridHashMap<bool>,
    rules: Vec<(Vec<Direction>, Direction)>,
}

impl Grid {
    fn new(grid: GridHashMap<bool>) -> Self {
        /*
           If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
           If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
           If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
           If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
        */
        let rules = vec![
            (
                vec![Direction::North, Direction::NorthEast, Direction::NorthWest],
                Direction::North,
            ),
            (
                vec![Direction::South, Direction::SouthEast, Direction::SouthWest],
                Direction::South,
            ),
            (
                vec![Direction::West, Direction::NorthWest, Direction::SouthWest],
                Direction::West,
            ),
            (
                vec![Direction::East, Direction::NorthEast, Direction::SouthEast],
                Direction::East,
            ),
        ];
        Grid { grid, rules }
    }

    fn simulate_round(&mut self, round: usize) -> bool {
        let proposed_new_coor_map = self.get_proposed_coors(round);

        if proposed_new_coor_map.is_empty() {
            if PRINT_DEBUG {
                println!("round {} no elfes moved", round + 1);
            }
            return true;
        }

        for (proposed_new_coor, mut elf_positions) in proposed_new_coor_map {
            if elf_positions.len() == 1 {
                // move the elf
                let elf_coor = elf_positions.pop().unwrap();
                self.grid.remove(&elf_coor);
                self.grid.set(proposed_new_coor, true);
            }
        }

        if PRINT_DEBUG {
            println!("round {}", round + 1);
            self.print(14, 14);
            println!();
        }

        false
    }

    fn get_proposed_coors(
        &mut self,
        round: usize,
    ) -> FxHashMap<Coor2DMut<isize>, Vec<Coor2DMut<isize>>> {
        let mut proposed_new_coor_map = FxHashMap::default();
        let mut insert_fn = |proposed_new_coor: Coor2DMut<isize>, elf_coor: Coor2DMut<isize>| {
            proposed_new_coor_map
                .entry(proposed_new_coor)
                .or_insert(vec![])
                .push(elf_coor);
        };
        for elf_coor in self.grid.all_indexes() {
            if self.any_elves_in_neighboorhod(&elf_coor) {
                // no elfes in the neighborhood, do not move
                continue;
            }

            for (no_elf_directions, proposed_direction) in
                (0..4).map(|i| self.get_relevant_rule(round, i))
            {
                if self.no_elves_in_directions(&elf_coor, no_elf_directions) {
                    // no elf in any of the no_elf_directions
                    // propose to move in the proposed_direction
                    let proposed_new_coor = elf_coor.clone() + proposed_direction.diff_coor();
                    insert_fn(proposed_new_coor, elf_coor);
                    break;
                }
            }
        }
        proposed_new_coor_map
    }

    fn get_relevant_rule(&self, round: usize, index: usize) -> &(Vec<Direction>, Direction) {
        self.rules.get((round + index) % 4).unwrap()
    }

    fn any_elves_in_neighboorhod(&self, elf_coor: &Coor2DMut<isize>) -> bool {
        self.grid
            .neighborhood_cells(elf_coor)
            .filter(|(_coor, cell)| cell.is_some())
            .count()
            == 0
    }

    fn no_elves_in_directions(
        &self,
        elf_coor: &Coor2DMut<isize>,
        directions: &[Direction],
    ) -> bool {
        directions.iter().all(|no_elf_direction| {
            let coor = elf_coor.clone() + no_elf_direction.diff_coor();
            self.grid.get(&coor).is_none()
        })
    }

    fn calc_result(&self) -> usize {
        let (min_coor, max_coor) = self.grid.get_min_max();
        let count = self.grid.iter().count();
        if PRINT_DEBUG {
            println!(
                "min_coor: {:?}, max_coor: {:?}, count: {}",
                min_coor, max_coor, count
            );
        }
        usize::try_from((max_coor.x - min_coor.x + 1) * (max_coor.y - min_coor.y + 1)).unwrap()
            - count
    }
    fn print(&self, min_width: isize, min_height: isize) {
        let (min_coor, max_coor) = self.grid.get_min_max();
        let x_offset = (min_width - (max_coor.x - min_coor.x + 1)).max(0) / 2;
        let y_offset = (min_height - (max_coor.y - min_coor.y + 1)).max(0) / 2;
        for y in min_coor.y - y_offset..=max_coor.y + y_offset {
            for x in min_coor.x - x_offset..=max_coor.x + x_offset {
                match self.grid.get(&Coor2DMut::new(x, y)) {
                    Some(true) => print!("#"),
                    Some(false) => unreachable!("should not be false"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> GridHashMap<bool> {
    let mut grid: GridHashMap<bool> = GridHashMapBuilder::default()
        .neighborhood(Neighborhood::Square)
        .build()
        .unwrap();
    utils::file_to_lines(file_name)
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    grid.set(
                        Coor2DMut::<isize>::new(x.try_into().unwrap(), y.try_into().unwrap()),
                        true,
                    );
                }
            });
        });
    grid
}

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 110);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 3849);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 20);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 995);
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
