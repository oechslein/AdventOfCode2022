//! Types grids

/// CellIndexCoorType
pub type CoorIndex = usize;
/// CellIndexType
pub type Coor = (CoorIndex, CoorIndex);

/*
use derive_more::{Add, AddAssign, Constructor, Display, Sub, SubAssign};
use std::convert::AsRef;


#[derive(
    Eq,
    PartialEq,
    Hash,
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
struct CoorMut {
    pub x: CoorIndex,
    pub y: CoorIndex,
}

impl From<(CoorIndex, CoorIndex)> for CoorMut {
    fn from(t: (CoorIndex, CoorIndex)) -> Self {
        CoorMut { x: t.0, y: t.1 }
    }
}

impl<T: ?Sized> AsRef<(T,T)> for CoorMut where T: From<CoorIndex> {
    fn as_ref(&self) -> &(T, T) {
        &(self.x.into(), self.y.into())
    }
}
 */

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
