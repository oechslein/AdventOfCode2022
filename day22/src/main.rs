#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
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

use grid::{
    grid_array::{GridArray, GridArrayBuilder},
    grid_types::{Coor2D, Direction, Neighborhood, Topology},
};
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day22/input.txt"));
    //utils::with_measure("Part 2", || solve_part2("day22/test.txt", false));
    //utils::with_measure("Part 2", || solve_part2("day22/input.txt", true));
}

////////////////////////////////////////////////////////////////////////////////////
///
///
///

type UInt = u16;

pub fn solve_part1(file_name: &str) -> usize {
    let (grid, path) = parse(file_name);
    let mut turtle = Turtle::new(grid);

    for (command, amount) in &path {
        turtle.apply_command(command);
        for _index in 0..*amount {
            turtle.move_forward();
        }
    }

    turtle.calc_result()
}

pub fn solve_part2(file_name: &str, _is_input: bool) -> usize {
    let (grid, path) = parse(file_name);
    let mut turtle = Turtle::new(grid);

    for (command, amount) in &path {
        turtle.apply_command(command);
        for _index in 0..*amount {
            turtle.move_forward_part2();
        }
    }

    turtle.calc_result()
}

////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Command {
    Left,
    Right,
    Straight,
}

#[derive(Debug, Clone)]
struct Turtle {
    curr_pos: Coor2D,
    curr_dir: Direction,
    grid: GridArray<char>,
}

impl Turtle {
    fn new(grid: GridArray<char>) -> Self {
        let mut turtle = Turtle {
            curr_pos: Coor2D::new(0, 0),
            curr_dir: Direction::East,
            grid,
        };

        if turtle.grid.get_unchecked(0, 0) == &' ' {
            turtle.move_forward(); // move to valid position
        }

        turtle.print_grid_blocks();

        turtle
    }

    fn get_cube_width(&self) -> usize {
        if self.grid.get_width() > self.grid.get_height() {
            self.grid.get_width() / 4
        } else {
            self.grid.get_height() / 3
        }
    }

    fn get_grid_block(&self, pos: &Coor2D) -> (usize, usize) {
        let x = pos.x / self.get_cube_width();
        let y = pos.y / self.get_cube_width();
        (x, y)
    }

    fn get_cube_side(&self, pos: &Coor2D) -> Option<usize> {
        match (pos.x / 4, pos.y / 3) {
            (0, 0) => None,
            (1, 0) => None,
            (2, 0) => Some(3),
            (3, 0) => None,

            (0, 1) => Some(6),
            (1, 1) => Some(2),
            (2, 1) => Some(1),
            (3, 1) => None,

            (0, 2) => None,
            (1, 2) => None,
            (2, 2) => Some(4),
            (3, 2) => Some(5),

            _ => None,
        }
    }
    fn get_cube_side_helper(&self, from_side: usize, dir: Direction) -> (usize, isize) {
        match (from_side, dir) {
            (1, Direction::East) => (5, 90),
            (1, Direction::South) => (4, 0),
            (1, Direction::West) => (2, 0),
            (1, Direction::North) => (3, 0),

            (2, Direction::East) => (1, 0),
            (2, Direction::South) => (4, -90),
            (2, Direction::West) => (6, 0),
            (2, Direction::North) => (3, 0),

            (3, Direction::East) => (1, 180),
            (3, Direction::South) => (5, 0),
            (3, Direction::West) => (2, 90),
            (3, Direction::North) => (4, 0),

            (4, Direction::East) => (5, 0),
            (4, Direction::South) => (6, 180),
            (4, Direction::West) => (2, -00),
            (4, Direction::North) => (1, 0),

            (5, Direction::East) => (3, 180),
            (5, Direction::South) => (6, -90),
            (5, Direction::West) => (4, 0),
            (5, Direction::North) => (1, 90),

            (6, Direction::East) => (2, 0),
            (6, Direction::South) => (4, 180),
            (6, Direction::West) => (5, 90),
            (6, Direction::North) => (3, 180),

            _ => unreachable!(),
        }
    }

    fn get_real_cube_side(&self, pos: &Coor2D) -> Option<usize> {
        match (pos.x / 3, pos.y / 4) {
            (0, 0) => None,
            (1, 0) => Some(2),
            (2, 0) => Some(4),

            (0, 1) => None,
            (1, 1) => Some(1),
            (2, 1) => None,

            (0, 2) => Some(3),
            (1, 2) => Some(5),
            (2, 2) => None,

            (0, 3) => Some(6),
            (1, 3) => None,
            (2, 3) => None,

            _ => None,
        }
    }

    fn get_real_cube_side_helper(&self, from_side: usize, dir: Direction) -> (usize, isize) {
        match (from_side, dir) {
            _ => unimplemented!(),
        }
    }

    fn calc_result(&self) -> usize {
        1000 * (self.curr_pos.y as usize + 1)
            + 4 * (self.curr_pos.x as usize + 1)
            + match self.curr_dir {
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 2,
                Direction::North => 3,
                _ => unreachable!(),
            }
    }

    fn move_forward_part2(&mut self) {
        let next_pos = self._get_next_pos_part2();

        // check for wall
        if self.grid.get_unchecked(next_pos.x, next_pos.y) != &'#' {
            self.curr_pos = next_pos;
        }

        debug_assert!(self.get_cube_side(&self.curr_pos).is_some());

        debug_assert!(
            self.grid.get_unchecked(self.curr_pos.x, self.curr_pos.y) == &'.',
            "AFTER MOVE '{}' {:?}",
            self.grid.get_unchecked(self.curr_pos.x, self.curr_pos.y),
            self
        );
    }

    fn _get_next_pos_part2(&mut self) -> Coor2D {
        // if we are on the same cube side all fine
        // if not we need to get the next cube side and the direction to turn it

        let mut next_pos = self.curr_pos.clone();
        let curr_side = self.get_cube_side(&next_pos).unwrap();
        next_pos = self
            .grid
            .adjacent_cell(next_pos.x, next_pos.y, self.curr_dir)
            .unwrap();
        let next_side = self.get_cube_side(&next_pos).unwrap();
        if curr_side == next_side {
            return next_pos;
        }

        let (next_side, rotation) = self.get_cube_side_helper(curr_side, self.curr_dir);
        let _new_dir = self.curr_dir.rotate(rotation);

        // now we need the new position on the new cube side
        // get x,y index on current cube side
        let cube_index_x = next_pos.x % self.get_cube_width();
        let cube_index_y = next_pos.y % self.get_cube_width();

        // now we have to transform back into the new cube sides indexes
        // I'm lazy just search the upper left coordinate
        let new_grid_side_upper_left = (0..self.grid.get_width())
            .cartesian_product(0..self.grid.get_height())
            .find(|(x, y)| self.get_cube_side(&Coor2D::new(*x, *y)) == Some(next_side))
            .unwrap();

        match (self.curr_dir, rotation) {
            (_, 0) => {
                // no rotation just go ahead
                // rotating by 0 coming from west (if direction is east) ...
                //next_pos.x = next_pos.x;
                //next_pos.y = next_pos.y;
            }
            (Direction::East, -90) => {
                // cube_index_x must be self.get_cube_width()-1
                assert_eq!(cube_index_x, self.get_cube_width() - 1);
                // cube_index_y could be anything
                // rotating by -90 coming from north
                // => new_cube_index_x = cube_index_y
                // => new_cube_index_y = 0
                next_pos.x = new_grid_side_upper_left.0 + cube_index_y;
                next_pos.y = new_grid_side_upper_left.1;
                assert_eq!(next_pos.x % self.get_cube_width(), cube_index_y);
                assert_eq!(next_pos.y % self.get_cube_width(), 0);
            }
            (Direction::East, 90) => {
                // cube_index_x must be self.get_cube_width()-1
                assert_eq!(cube_index_x, self.get_cube_width() - 1);
                // cube_index_y could be anything
                // rotating by 90 coming from south
                // => new_cube_index_x = cube_index_y
                // => new_cube_index_y = self.get_cube_width()-1
                next_pos.x = new_grid_side_upper_left.0 + cube_index_y;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - 1;
                assert_eq!(next_pos.x % self.get_cube_width(), cube_index_y);
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
            }
            (Direction::East, 180) => {
                // cube_index_x must be self.get_cube_width()-1
                assert_eq!(cube_index_x, self.get_cube_width() - 1);
                // cube_index_y could be anything
                // rotating by 90 coming from east
                // => new_cube_index_x = self.get_cube_width()-1
                // => new_cube_index_y = self.get_cube_width()-cube_index_y+1
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width();
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - cube_index_y + 1;
                assert_eq!(next_pos.x % self.get_cube_width(), cube_index_y);
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
            }
            (Direction::West, -90) => {
                // cube_index_x must be 0
                assert_eq!(cube_index_x, 0);
                // cube_index_y could be anything
                // rotating by -90 coming from south
                // => new_cube_index_x = self.get_cube_width()-cube_index_y
                // => new_cube_index_y = self.get_cube_width()-1
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - cube_index_y;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - 1;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - cube_index_y
                );
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
            }
            (Direction::West, 90) => {
                // cube_index_x must be 0
                assert_eq!(cube_index_x, 0);
                // cube_index_y could be anything
                // rotating by 90 coming from north
                // => new_cube_index_x = self.get_cube_width()-cube_index_y
                // => new_cube_index_y = 0
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - cube_index_y;
                next_pos.y = new_grid_side_upper_left.1 + 0;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - cube_index_y
                );
                assert_eq!(next_pos.y % self.get_cube_width(), 0);
            }
            (Direction::West, 180) => {
                // cube_index_x must be 0
                assert_eq!(cube_index_x, 0);
                // cube_index_y could be anything
                // rotating by 180 coming from west
                // => new_cube_index_x = self.get_cube_width()-1
                // => new_cube_index_y = self.get_cube_width()-cube_index_y+1
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - 1;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - cube_index_y + 1;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - cube_index_y + 1
                );
            }
            (Direction::North, -90) => {
                // cube_index_x could be anything
                // cube_index_y must be 0
                assert_eq!(cube_index_y, 0);
                // rotating by -90 coming from west
                // => new_cube_index_x = self.get_cube_width()-1
                // => new_cube_index_y = self.get_cube_width()-cube_index_x
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - 1;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - cube_index_x;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - cube_index_x
                );
            }
            (Direction::North, 90) => {
                // cube_index_x could be anything
                // cube_index_y must be 0
                assert_eq!(cube_index_y, 0);
                // rotating by 90 coming from east
                // => new_cube_index_x = 0
                // => new_cube_index_y = self.get_cube_width()-cube_index_x
                next_pos.x = new_grid_side_upper_left.0 + 0;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - cube_index_x;
                assert_eq!(next_pos.x % self.get_cube_width(), 0);
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - cube_index_x
                );
            }
            (Direction::North, 180) => {
                // cube_index_x could be anything
                // cube_index_y must be 0
                assert_eq!(cube_index_y, 0);
                // rotating by 180 coming from north
                // => new_cube_index_x = self.get_cube_width()-cube_index_x+1
                // => new_cube_index_y = self.get_cube_width()-1
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - cube_index_x + 1;
                next_pos.y = new_grid_side_upper_left.1 + self.get_cube_width() - 1;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - cube_index_x + 1
                );
                assert_eq!(
                    next_pos.y % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
            }
            (Direction::South, -90) => {
                // cube_index_x could be anything
                // cube_index_y must be self.get_cube_width()-1
                assert_eq!(cube_index_y, self.get_cube_width() - 1);
                // rotating by -90 coming from east
                // => new_cube_index_x = 0
                // => new_cube_index_y = cube_index_x
                next_pos.x = new_grid_side_upper_left.0 + 0;
                next_pos.y = new_grid_side_upper_left.1 + cube_index_x;
                assert_eq!(next_pos.x % self.get_cube_width(), 0);
                assert_eq!(next_pos.y % self.get_cube_width(), cube_index_x);
            }
            (Direction::South, 90) => {
                // cube_index_x could be anything
                // cube_index_y must be self.get_cube_width()-1
                assert_eq!(cube_index_y, self.get_cube_width() - 1);
                // rotating by 90 coming from west
                // => new_cube_index_x = self.get_cube_width()-1
                // => new_cube_index_y = cube_index_x
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - 1;
                next_pos.y = new_grid_side_upper_left.1 + cube_index_x;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - 1
                );
                assert_eq!(next_pos.y % self.get_cube_width(), cube_index_x);
            }
            (Direction::South, 180) => {
                // cube_index_x could be anything
                // cube_index_y must be self.get_cube_width()-1
                assert_eq!(cube_index_y, self.get_cube_width() - 1);
                // rotating by 180 coming from south
                // => new_cube_index_x = self.get_cube_width()-cube_index_x+1
                // => new_cube_index_y = 0
                next_pos.x = new_grid_side_upper_left.0 + self.get_cube_width() - cube_index_x + 1;
                next_pos.y = new_grid_side_upper_left.1 + 0;
                assert_eq!(
                    next_pos.x % self.get_cube_width(),
                    self.get_cube_width() - cube_index_x + 1
                );
                assert_eq!(next_pos.y % self.get_cube_width(), 0);
            }
            (Direction::South, rotation) => {
                panic!("Invalid rotation degree: {}", rotation);
            }
            (Direction::North, rotation) => {
                panic!("Invalid rotation degree: {}", rotation);
            }
            (Direction::West, rotation) => {
                panic!("Invalid rotation degree: {}", rotation);
            }
            (Direction::East, rotation) => {
                panic!("Invalid rotation degree: {}", rotation);
            }
            (dir, rot) => {
                panic!("Invalid direction: {:?} and rotation: {}", dir, rot);
            }
        }

        next_pos

        // I need to correct the coordinates to the other cube side
        // and I need to correct the direction I'm facing
        // both based on the rotation degree I'm getting
        // rotation means: rotate the target cube side
    }

    fn move_forward(&mut self) {
        let next_pos = self._get_next_pos();

        // check for wall
        if self.grid.get_unchecked(next_pos.x, next_pos.y) != &'#' {
            self.curr_pos = next_pos;
        }

        debug_assert!(
            self.grid.get_unchecked(self.curr_pos.x, self.curr_pos.y) == &'.',
            "AFTER MOVE '{}' {:?}",
            self.grid.get_unchecked(self.curr_pos.x, self.curr_pos.y),
            self
        );
    }

    // going over the edge of the grid will wrap around and also ignoring empty cells
    fn _get_next_pos(&mut self) -> Coor2D {
        let mut next_pos = self.curr_pos.clone();
        next_pos = self
            .grid
            .adjacent_cell(next_pos.x, next_pos.y, self.curr_dir)
            .unwrap();
        while self.grid.get_unchecked(next_pos.x, next_pos.y) == &' ' {
            next_pos = self
                .grid
                .adjacent_cell(next_pos.x, next_pos.y, self.curr_dir)
                .unwrap();
        }
        next_pos
    }

    fn apply_command(&mut self, cmd: &Command) {
        match (self.curr_dir, cmd) {
            (Direction::North, Command::Left) => self.curr_dir = Direction::West,
            (Direction::North, Command::Right) => self.curr_dir = Direction::East,
            (Direction::South, Command::Left) => self.curr_dir = Direction::East,
            (Direction::South, Command::Right) => self.curr_dir = Direction::West,
            (Direction::East, Command::Left) => self.curr_dir = Direction::North,
            (Direction::East, Command::Right) => self.curr_dir = Direction::South,
            (Direction::West, Command::Left) => self.curr_dir = Direction::South,
            (Direction::West, Command::Right) => self.curr_dir = Direction::North,
            (_, Command::Straight) => {}
            (Direction::NorthEast, _) => unreachable!("NorthEast is not a valid direction"),
            (Direction::SouthEast, _) => unreachable!("SouthEast is not a valid direction"),
            (Direction::NorthWest, _) => unreachable!("NorthWest is not a valid direction"),
            (Direction::SouthWest, _) => unreachable!("SouthWest is not a valid direction"),
        }
    }

    fn print_grid(&self) {
        for y in 0..self.grid.get_height() {
            print!("|");
            for x in 0..self.grid.get_width() {
                if self.curr_pos.to_tuple() == (x, y) {
                    print!(
                        "{}",
                        match self.curr_dir {
                            Direction::North => '^',
                            Direction::South => 'v',
                            Direction::East => '>',
                            Direction::West => '<',
                            _ => unreachable!(),
                        }
                    );
                } else {
                    print!("{}", self.grid.get_unchecked(x, y));
                }
            }
            println!("|");
        }
    }

    fn print_grid_blocks(&self) {
        for y in 0..self.grid.get_height() {
            print!("|");
            for x in 0..self.grid.get_width() {
                let (x, y) = self.get_grid_block(&Coor2D::new(x, y));
                print!("{:x}", y * self.get_cube_width() + x);
            }
            println!("|");
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

fn parse(file_name: &str) -> (GridArray<char>, Vec<(Command, u16)>) {
    let input = utils::file_to_string(file_name).replace("\r\n", "\n");
    let (board, path_str) = input.split_once("\n\n").unwrap();
    let grid_vec = board
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut grid: GridArray<char> = GridArrayBuilder::default()
        .topology(Topology::Torus)
        .neighborhood(Neighborhood::Orthogonal)
        .width(grid_vec.iter().map(|x| x.len()).max().unwrap())
        .height(grid_vec.len())
        .build()
        .unwrap();
    for y in 0..grid.get_height() {
        for x in 0..grid.get_width() {
            grid.set(x, y, *grid_vec[y].get(x).unwrap_or(&' '));
        }
    }
    let alpha_indexes = path_str
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .map(|(index, _)| index)
        .chain(vec![path_str.len()].into_iter())
        .collect_vec();
    let mut path: Vec<(Command, UInt)> = alpha_indexes
        .iter()
        .tuple_windows()
        .map(|(index_start, index_end)| {
            (
                match path_str.chars().skip(*index_start).next().unwrap() {
                    'L' => Command::Left,
                    'R' => Command::Right,
                    _ => panic!("Unknown direction"),
                },
                utils::str_to(&path_str[index_start + 1..*index_end]),
            )
        })
        .collect_vec();
    path.insert(
        0,
        (
            Command::Straight,
            path_str[0..alpha_indexes[0]].parse::<UInt>().unwrap(),
        ),
    );
    (grid, path)
}

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 6032);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 27492);
    }

    #[ignore]
    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt", false), 5031);
    }

    #[ignore]
    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt", true), 78291);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[ignore]
    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt", true));
    }
}
