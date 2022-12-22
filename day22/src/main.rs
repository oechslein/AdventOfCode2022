use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use num_integer::gcd;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

use Facing::*;

impl Facing {
    fn movement(&self, pos: Pos) -> Pos {
        match self {
            Right => Pos {
                row: pos.row,
                col: pos.col + 1,
            },
            Down => Pos {
                row: pos.row + 1,
                col: pos.col,
            },
            Left => Pos {
                row: pos.row,
                col: pos.col - 1,
            },
            Up => Pos {
                row: pos.row - 1,
                col: pos.col,
            },
        }
    }

    fn turn_left(&self) -> Facing {
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }

    fn turn_right(&self) -> Facing {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn opposite(&self) -> Facing {
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
        }
    }
}

enum Direction {
    TurnLeft,
    TurnRight,
    Forward(usize),
}

use Direction::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tile {
    Nothing,
    Air,
    Wall,
}

use Tile::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Pos {
    row: usize,
    col: usize,
}

type Grid = HashMap<Pos, Tile>;

pub struct Map {
    grid: Grid,
    start: Pos,
    facing: Facing,
    directions: Vec<Direction>,
    cube_stitchings: HashMap<(Pos, Facing), (Pos, Facing)>,
}

const DEBUG_CUBE: bool = true;
const DEBUG_FACES: bool = false;
const DEBUG_FOLDING: bool = false;
const DEBUG_STITCHING: bool = false;

impl Map {
    pub fn at(&self, pos: &Pos) -> Tile {
        *self.grid.get(pos).unwrap_or(&Nothing)
    }

    pub fn warp_flat(&self, pos: Pos, facing: Facing) -> (Pos, Facing) {
        assert_eq!(self.at(&pos), Air);
        assert_eq!(self.at(&facing.movement(pos)), Nothing);
        let direction = facing.opposite();
        let mut pos = pos;
        let mut next = direction.movement(pos);
        while self.at(&next) != Nothing {
            pos = next;
            next = direction.movement(pos);
        }
        (pos, facing)
    }

    pub fn warp_cube(&self, pos: Pos, facing: Facing) -> (Pos, Facing) {
        assert_eq!(self.at(&pos), Air);
        assert_eq!(self.at(&facing.movement(pos)), Nothing);
        *self.cube_stitchings.get(&(pos, facing)).unwrap()
    }

    fn build_cube(&mut self, width: usize, height: usize) {
        // I think there might be foldings where this will give the wrong size (double the real
        // one), but it works on both my example and my sample input.
        let cube_size = gcd(width, height);
        if DEBUG_CUBE {
            dbg!(cube_size);
        }
        let mut cube = 0;
        let mut faces: Vec<Face> = Vec::new();
        let mut face_at: HashMap<Pos, usize> = HashMap::new();
        for row in 0..height / cube_size {
            for col in 0..width / cube_size {
                let pos = Pos {
                    row: 1 + row * cube_size,
                    col: 1 + col * cube_size,
                };
                if self.at(&pos) == Nothing {
                    if DEBUG_FACES {
                        print!("   ");
                    }
                } else {
                    face_at.insert(pos, cube);
                    faces.push(Face {
                        pos,
                        size: cube_size,
                        which: None,
                        flat_edges: HashMap::new(),
                    });
                    cube += 1;
                    if DEBUG_FACES {
                        print!("[{}]", cube);
                    }
                }
            }
            if DEBUG_FACES {
                println!();
            }
        }
        assert_eq!(cube, 6);
        faces[0].which = Some(CubeFace::Front);
        faces[0].flat_edges.insert(Right, CubeFace::Right);
        faces[0].flat_edges.insert(Left, CubeFace::Left);
        faces[0].flat_edges.insert(Up, CubeFace::Top);
        faces[0].flat_edges.insert(Down, CubeFace::Bottom);

        let next_left: HashMap<(CubeFace, CubeFace), CubeFace> = HashMap::from([
            // I'm sure this can be derived from the vertices somehow
            ((CubeFace::Front, CubeFace::Top), CubeFace::Left),
            ((CubeFace::Front, CubeFace::Left), CubeFace::Bottom),
            ((CubeFace::Front, CubeFace::Bottom), CubeFace::Right),
            ((CubeFace::Front, CubeFace::Right), CubeFace::Top),
            ((CubeFace::Left, CubeFace::Top), CubeFace::Back),
            ((CubeFace::Left, CubeFace::Back), CubeFace::Bottom),
            ((CubeFace::Left, CubeFace::Bottom), CubeFace::Front),
            ((CubeFace::Left, CubeFace::Front), CubeFace::Top),
            ((CubeFace::Right, CubeFace::Top), CubeFace::Front),
            ((CubeFace::Right, CubeFace::Front), CubeFace::Bottom),
            ((CubeFace::Right, CubeFace::Bottom), CubeFace::Back),
            ((CubeFace::Right, CubeFace::Back), CubeFace::Top),
            ((CubeFace::Back, CubeFace::Top), CubeFace::Right),
            ((CubeFace::Back, CubeFace::Right), CubeFace::Bottom),
            ((CubeFace::Back, CubeFace::Bottom), CubeFace::Left),
            ((CubeFace::Back, CubeFace::Left), CubeFace::Top),
            ((CubeFace::Top, CubeFace::Front), CubeFace::Right),
            ((CubeFace::Top, CubeFace::Right), CubeFace::Back),
            ((CubeFace::Top, CubeFace::Back), CubeFace::Left),
            ((CubeFace::Top, CubeFace::Left), CubeFace::Front),
            ((CubeFace::Bottom, CubeFace::Front), CubeFace::Left),
            ((CubeFace::Bottom, CubeFace::Left), CubeFace::Back),
            ((CubeFace::Bottom, CubeFace::Back), CubeFace::Right),
            ((CubeFace::Bottom, CubeFace::Right), CubeFace::Front),
        ]);

        let mut queue = VecDeque::from([0]);
        while let Some(u) = queue.pop_front() {
            let which_u = faces[u].which.unwrap();
            for direction in [Right, Down, Left, Up] {
                if let Some(v) = face_at.get(&faces[u].flat_sibling(direction)) {
                    if faces[*v].which.is_some() {
                        continue;
                    }
                    if !faces[u].flat_edges.contains_key(&direction) {
                        continue;
                    }
                    let which_v = faces[u].flat_edges[&direction];
                    faces[*v].which = Some(which_v);
                    if DEBUG_FOLDING {
                        println!(
                            "{} ({:?}) -> {} ({:?}) going {:?}",
                            u + 1,
                            which_u,
                            v + 1,
                            which_v,
                            direction
                        );
                    }
                    let mut insert = |direction, face| {
                        if DEBUG_FOLDING {
                            println!("  go {:?} to get to {:?}", direction, face);
                        }
                        faces[*v].flat_edges.insert(direction, face);
                    };
                    insert(direction, which_u.opposite());
                    insert(direction.turn_left(), next_left[&(which_u, which_v)]);
                    insert(
                        direction.turn_right(),
                        next_left[&(which_u, which_v)].opposite(),
                    );
                    insert(direction.opposite(), which_u);
                    queue.push_back(*v);
                }
            }
        }
        // A cube has 6 sides, 8 vertices and 12 edges
        // In a folding, 5 edges stay connected, the other 7 get split into 14 unconnected square
        // edges.

        if DEBUG_CUBE {
            for row in 0..height / cube_size {
                for col in 0..width / cube_size {
                    let pos = Pos {
                        row: 1 + row * cube_size,
                        col: 1 + col * cube_size,
                    };
                    if let Some(f) = face_at.get(&pos) {
                        print!(
                            "[{}]",
                            match faces[*f].which {
                                None => "?",
                                Some(CubeFace::Front) => "F",
                                Some(CubeFace::Back) => "K",
                                Some(CubeFace::Left) => "L",
                                Some(CubeFace::Right) => "R",
                                Some(CubeFace::Top) => "T",
                                Some(CubeFace::Bottom) => "B",
                            }
                        );
                    } else {
                        print!("   ");
                    }
                }
                println!();
            }
        }

        let mut stitchings: HashMap<CubeEdge, (Facing, CubeVertex, Vec<Pos>)> = HashMap::new();
        for face in faces {
            for direction in [Right, Down, Left, Up] {
                if !face_at.contains_key(&face.flat_sibling(direction)) {
                    let (vertex, tiles) = face.edge_tiles(direction);
                    let pos = tiles[0];
                    let other_face = face.flat_edges[&direction];
                    if face.common_edge(other_face).is_none() {
                        println!(
                            "No common edge between {:?} and {:?}!",
                            face.which.unwrap(),
                            other_face
                        );
                        panic!();
                    }
                    let edge = face.common_edge(other_face).unwrap();
                    if DEBUG_STITCHING {
                        println!(
                            "The {:?} edge of {:?} wants {:?} for {:?} (with {:?} at {:?})",
                            direction,
                            face.which.unwrap(),
                            other_face,
                            edge,
                            vertex,
                            pos,
                        );
                    }
                    if let Some((other_direction, other_vertex, other_tiles)) =
                        stitchings.get(&edge)
                    {
                        let pairs: Vec<_> = if vertex == *other_vertex {
                            if DEBUG_STITCHING {
                                println!(
                                    "  stiching straight {:?}..{:?} with {:?}..{:?}",
                                    tiles[0],
                                    tiles.last().unwrap(),
                                    other_tiles[0],
                                    other_tiles.last().unwrap()
                                );
                            }
                            tiles.iter().zip(other_tiles).collect()
                        } else {
                            if DEBUG_STITCHING {
                                println!(
                                    "  stiching reverse {:?}..{:?} with {:?}..{:?}",
                                    tiles.last().unwrap(),
                                    tiles[0],
                                    other_tiles[0],
                                    other_tiles.last().unwrap()
                                );
                            }
                            tiles.iter().rev().zip(other_tiles).collect()
                        };
                        for (pos, other) in pairs {
                            self.cube_stitchings
                                .insert((*pos, direction), (*other, other_direction.opposite()));
                            self.cube_stitchings
                                .insert((*other, *other_direction), (*pos, direction.opposite()));
                        }
                    } else {
                        stitchings.insert(edge, (direction, vertex, tiles));
                    }
                }
            }
        }
    }
}

// Let's number the vertices of the front face:
//
//    0 --- 1
//    |     |
//    |     |
//    3 --- 2
//
// back face:
//
//    4 --- 5
//    |     |
//    |     |
//    7 --- 6
//
// A set of vertices determines the face.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, IntoPrimitive)]
enum CubeFace {
    Front = 0b0000_1111,
    Top = 0b0011_0011,
    Bottom = 0b1100_1100,
    Left = 0b1001_1001,
    Right = 0b0110_0110,
    Back = 0b1111_0000,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
enum CubeEdge {
    FrontTop = 0b0000_0011,
    FrontRight = 0b0000_0110,
    FrontBottom = 0b0000_1100,
    FrontLeft = 0b0000_1001,
    BackTop = 0b0011_0000,
    BackRight = 0b0110_0000,
    BackBottom = 0b1100_0000,
    BackLeft = 0b1001_0000,
    LeftTop = 0b0001_0001,
    LeftBottom = 0b1000_1000,
    RightTop = 0b0010_0010,
    RightBottom = 0b0100_0100,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, TryFromPrimitive)]
enum CubeVertex {
    FrontTopLeft = 0b0000_0001,
    FrontTopRight = 0b0000_0010,
    FrontBottomRight = 0b0000_0100,
    FrontBottomLeft = 0b0000_1000,
    BackTopLeft = 0b0001_0000,
    BackTopRight = 0b0010_0000,
    BackBottomRight = 0b0100_0000,
    BackBottomLeft = 0b1000_0000,
}

impl CubeFace {
    fn opposite(self) -> CubeFace {
        use CubeFace::*;
        match self {
            Front => Back,
            Back => Front,
            Left => Right,
            Right => Left,
            Top => Bottom,
            Bottom => Top,
        }
    }

    #[allow(dead_code)]
    fn touching(self) -> [CubeFace; 4] {
        use CubeFace::*;
        match self {
            Front => [Top, Left, Bottom, Right],
            Left => [Top, Back, Bottom, Front],
            Right => [Top, Front, Bottom, Back],
            Back => [Top, Right, Bottom, Left],
            Top => [Front, Right, Back, Left],
            Bottom => [Front, Left, Back, Right],
        }
    }

    fn common_edge(self, other: CubeFace) -> Option<CubeEdge> {
        let a: u8 = self.into();
        let b: u8 = other.into();
        (a & b).try_into().ok()
    }

    fn common_vertex(self, other1: CubeFace, other2: CubeFace) -> Option<CubeVertex> {
        let a: u8 = self.into();
        let b: u8 = other1.into();
        let c: u8 = other2.into();
        (a & b & c).try_into().ok()
    }
}

struct Face {
    pos: Pos,
    size: usize,
    which: Option<CubeFace>,
    flat_edges: HashMap<Facing, CubeFace>,
}

impl Face {
    fn common_edge(&self, other: CubeFace) -> Option<CubeEdge> {
        self.which?.common_edge(other)
    }

    fn common_vertex(&self, other1: CubeFace, other2: CubeFace) -> Option<CubeVertex> {
        self.which?.common_vertex(other1, other2)
    }

    fn edge_tiles(&self, direction: Facing) -> (CubeVertex, Vec<Pos>) {
        let n = self.size;
        let vertex = self
            .common_vertex(
                self.flat_edges[&direction],
                self.flat_edges[&direction.turn_left()],
            )
            .unwrap();
        let tiles = match direction {
            Right => (0..n)
                .map(|n| Pos {
                    row: self.pos.row + n,
                    col: self.pos.col + self.size - 1,
                })
                .collect(),
            Down => (0..n)
                .rev()
                .map(|n| Pos {
                    row: self.pos.row + self.size - 1,
                    col: self.pos.col + n,
                })
                .collect(),
            Left => (0..n)
                .rev()
                .map(|n| Pos {
                    row: self.pos.row + n,
                    col: self.pos.col,
                })
                .collect(),
            Up => (0..n)
                .map(|n| Pos {
                    row: self.pos.row,
                    col: self.pos.col + n,
                })
                .collect(),
        };
        (vertex, tiles)
    }

    fn flat_sibling(&self, direction: Facing) -> Pos {
        match direction {
            Right => Pos {
                row: self.pos.row,
                col: self.pos.col + self.size,
            },
            Down => Pos {
                row: self.pos.row + self.size,
                col: self.pos.col,
            },
            Left => Pos {
                row: self.pos.row,
                col: self.pos.col.saturating_sub(self.size),
            },
            Up => Pos {
                row: self.pos.row.saturating_sub(self.size),
                col: self.pos.col,
            },
        }
    }
}

fn parse_grid(input: &[&str]) -> (Grid, Pos) {
    let mut grid: Grid = HashMap::new();
    let mut start = None;
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            let pos = Pos {
                row: i + 1,
                col: j + 1,
            };
            match cell {
                '#' => {
                    grid.insert(pos, Wall);
                }
                '.' => {
                    grid.insert(pos, Air);
                    if start.is_none() {
                        start = Some(pos);
                    }
                }
                _ => (),
            }
        }
    }
    (
        grid,
        start.expect("there should be at least one air tile in the map"),
    )
}

fn parse_directions(input: &str) -> Vec<Direction> {
    lazy_static! {
        static ref TOKEN_RE: Regex = Regex::new(r"\d+|[LR]|.").unwrap();
    }
    TOKEN_RE
        .find_iter(input)
        .map(|m| match m.as_str() {
            "L" => TurnLeft,
            "R" => TurnRight,
            _ => Forward(m.as_str().parse().unwrap()),
        })
        .collect()
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() - 2;
        let width = lines[0..height].iter().map(|l| l.len()).max().unwrap();
        let (grid, start) = parse_grid(&lines[0..height]);
        let directions = parse_directions(lines.last().unwrap());
        let mut map = Map {
            grid,
            start,
            facing: Right,
            directions,
            cube_stitchings: HashMap::new(),
        };
        map.build_cube(width, height);
        map
    }

    pub fn walk(self: &Map, warp: impl Fn(&Map, Pos, Facing) -> (Pos, Facing)) -> usize {
        let map = self;
        let mut pos = map.start;
        let mut facing = map.facing;
        for direction in map.directions.iter() {
            match direction {
                TurnLeft => facing = facing.turn_left(),
                TurnRight => facing = facing.turn_right(),
                Forward(n) => {
                    for _ in 0..*n {
                        let mut next = facing.movement(pos);
                        let mut next_facing = facing;
                        if map.at(&next) == Nothing {
                            (next, next_facing) = warp(map, pos, facing);
                        }
                        match map.at(&next) {
                            Wall => break,
                            Air => {
                                pos = next;
                                facing = next_facing;
                            }
                            Nothing => unreachable!(),
                        }
                    }
                }
            }
        }
        1000 * pos.row + 4 * pos.col + facing as usize
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::parse(&input);
    println!("{}", map.walk(Map::warp_flat));
    println!("{}", map.walk(Map::warp_cube));
}
