//! Types grids

use derive_more::{Add, AddAssign, Constructor, Display, Sub, SubAssign};

/// CellIndexCoorType
pub type Coor2DIndex = usize;
/// CellIndexType
//pub type Coor = (CoorIndex, CoorIndex);
pub type Coor2D = Coor2DMut<Coor2DIndex>;

/// Coor
#[derive(
    Eq,
    PartialEq,
    Hash,
    Ord,
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
#[display(fmt = "({},{})", x, y)]
pub struct Coor2DMut<T: Clone> {
    /// x
    pub x: T,
    /// y
    pub y: T,
}

impl<T: Clone> From<(T, T)> for Coor2DMut<T> {
    fn from(t: (T, T)) -> Self {
        Coor2DMut { x: t.0, y: t.1 }
    }
}

impl<T: Clone> Coor2DMut<T> {
    /// to tuples
    pub fn to_tuple(&self) -> (T, T) {
        (self.x.clone(), self.y.clone())
    }
    /// from tuples
    pub fn from_tuple(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
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
pub(crate) enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
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
