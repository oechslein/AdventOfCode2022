//! Types grids

use std::fmt::Display;

use derive_more::{Add, AddAssign, Constructor, Display, Sub, SubAssign};

/// CellIndexCoorType
pub type Coor2DIndex = usize;
/// CellIndexType
pub type Coor2D = Coor2DMut<Coor2DIndex>;

/// Coor
#[derive(
    Eq,
    PartialEq,
    Hash,
    // Ord,
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
#[display(fmt = "({x},{y})")]
pub struct Coor2DMut<T: Clone + Ord + Eq + Display> {
    /// x
    pub x: T,
    /// y
    pub y: T,
}

impl<T: Clone + Ord + Eq + Display> From<(T, T)> for Coor2DMut<T> {
    fn from(t: (T, T)) -> Self {
        Coor2DMut { x: t.0, y: t.1 }
    }
}

impl<T: Clone + Ord + Eq + Display> Coor2DMut<T> {
    /// to tuples
    pub fn to_tuple(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
    /// from tuples
    #[must_use]
    pub fn from_tuple(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }

    /// to array
    pub fn to_array(&self) -> [T; 2] {
        [self.x.clone(), self.y.clone()]
    }

    /// min
    #[must_use]
    pub fn min(&self, other: &Self) -> Self {
        Self::new(
            self.x.clone().min(other.x.clone()),
            self.y.clone().min(other.y.clone()),
        )
    }

    /// max
    #[must_use]
    pub fn max(&self, other: &Self) -> Self {
        Self::new(
            self.x.clone().max(other.x.clone()),
            self.y.clone().max(other.y.clone()),
        )
    }

    /// Returns abs
    pub fn abs(&self) -> usize
    where
        T: TryInto<isize>,
        <T as TryInto<isize>>::Error: std::fmt::Debug,
    {
        #![allow(clippy::cast_sign_loss)]
        ((self.x.clone().try_into().unwrap()).abs() + (self.y.clone().try_into().unwrap()).abs())
            as usize
    }

    /// Returns manhattan distance
    pub fn manhattan_distance(&self, other: &Coor2DMut<T>) -> usize
    where
        T: TryInto<isize>,
        <T as TryInto<isize>>::Error: std::fmt::Debug,
    {
        #![allow(clippy::cast_sign_loss)]
        ((self.x.clone().try_into().unwrap() - other.clone().x.try_into().unwrap()).abs()
            + (self.y.clone().try_into().unwrap() - other.clone().y.try_into().unwrap()).abs())
            as usize
    }
}

/// A type of topology
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topology {
    /// A bounded grid, with no wrap-around
    Bounded = 0,
    /// A grid that wraps around, preserving the axis not moved in. e.g. Pacman
    Torus = 1,
}

/// All eight directions (Orthogonal+Diagonal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    /// North
    North = 0,
    /// NorthEast
    NorthEast = 1,
    /// East
    East = 2,
    /// SouthEast
    SouthEast = 3,
    /// South
    South = 4,
    /// SouthWest
    SouthWest = 5,
    /// West
    West = 6,
    /// NorthWest
    NorthWest = 7,
}

impl Direction {
    /// Returns the direction rotated by the given number of degrees
    #[must_use]
    pub fn rotate(&self, rotation: isize) -> Self {
        let new_dir = (*self as isize + rotation * 8 / 360).rem_euclid(8);
        match new_dir {
            0 => Direction::North,
            1 => Direction::NorthEast,
            2 => Direction::East,
            3 => Direction::SouthEast,
            4 => Direction::South,
            5 => Direction::SouthWest,
            6 => Direction::West,
            7 => Direction::NorthWest,
            _ => unreachable!(),
        }
    }

    #[must_use]
    /// Returns the difference vector for direction
    pub fn diff_coor(&self) -> Coor2DMut<isize> {
        match self {
            Direction::North => Coor2DMut::new(0, -1),
            Direction::NorthEast => Coor2DMut::new(1, -1),
            Direction::East => Coor2DMut::new(1, 0),
            Direction::SouthEast => Coor2DMut::new(1, 1),
            Direction::South => Coor2DMut::new(0, 1),
            Direction::SouthWest => Coor2DMut::new(-1, 1),
            Direction::West => Coor2DMut::new(-1, 0),
            Direction::NorthWest => Coor2DMut::new(-1, -1),
        }
    }
}

/// Neighborhoods around a point. They do not contain the point itself
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Neighborhood {
    /// The neighborhood consisting of the points directly North, South, East, and West of a point.
    Orthogonal,
    /// The neighborhood consisting of the points directly diagonal to a point.
    Diagonal,
    /// The neighborhood consisting of the square directly around the point.
    Square,
}
