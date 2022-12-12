//! Manipulating 2d grids

use super::grid_types::{
    Coor2D, Coor2DIndex, Direction, Direction::*, Neighborhood, Neighborhood::*, Topology,
    Topology::*,
};

/// Get the adjacent point to a point in a given direction
pub(crate) fn adjacent_cell(
    t: Topology,
    width: Coor2DIndex,
    height: Coor2DIndex,
    index: Coor2D,
    d: Direction,
) -> Option<Coor2D> {
    let (x, y) = (index.x, index.y);
    match d {
        NorthEast => adjacent_cell(t, width, height, index, North)
            .and_then(|new_coor| adjacent_cell(t, width, height, new_coor, East)),
        NorthWest => adjacent_cell(t, width, height, index, North)
            .and_then(|new_coor| adjacent_cell(t, width, height, new_coor, West)),
        SouthEast => adjacent_cell(t, width, height, index, South)
            .and_then(|new_coor| adjacent_cell(t, width, height, new_coor, East)),
        SouthWest => adjacent_cell(t, width, height, index, South)
            .and_then(|new_coor| adjacent_cell(t, width, height, new_coor, West)),

        _ => match t {
            Bounded => match d {
                North => Some(Coor2D::new(x, y.checked_sub(1)?)),
                South => {
                    if y + 1 < height {
                        Some(Coor2D::new(x, y + 1))
                    } else {
                        None
                    }
                }
                East => {
                    if x + 1 < width {
                        Some(Coor2D::new(x + 1, y))
                    } else {
                        None
                    }
                }
                West => Some(Coor2D::new(x.checked_sub(1)?, y)),

                _ => panic!(), // already handled above
            },
            Torus => match d {
                North => Some(Coor2D::new(x, y.checked_sub(1).unwrap_or(height - 1))),
                South => Some(Coor2D::new(x, (y + 1) % height)),
                East => Some(Coor2D::new((x + 1) % width, y)),
                West => Some(Coor2D::new(x.checked_sub(1).unwrap_or(width - 1), y)),

                _ => unreachable!(), // already handled above
            },
        },
    }
}

/// Is a given point on an edge of a grid
pub(crate) fn is_edge(t: Topology, width: Coor2DIndex, height: Coor2DIndex, index: Coor2D) -> bool {
    let (x, y) = (index.x, index.y);
    t == Topology::Bounded && (x == 0 || x + 1 == width || y == 0 || y + 1 == height)
}

/// Is a given point a corner of a grid
pub(crate) fn is_corner(
    t: Topology,
    width: Coor2DIndex,
    height: Coor2DIndex,
    index: Coor2D,
) -> bool {
    let (x, y) = (index.x, index.y);
    t == Topology::Bounded && (x == 0 || x + 1 == width) && (y == 0 || y + 1 == height)
}

/// Returns an iterator over the points of a grid
pub(crate) fn all_cells(width: Coor2DIndex, height: Coor2DIndex) -> impl Iterator<Item = Coor2D> {
    (0..width).flat_map(move |x| (0..height).map(move |y| Coor2D::new(x, y)))
}

/// Returns an iterator over the directions for given neighborhood type
fn all_adjacent_cells(n: Neighborhood) -> impl Iterator<Item = Direction> {
    match n {
        Orthogonal => vec![North, South, East, West],
        Diagonal => vec![NorthWest, NorthEast, SouthEast, SouthWest],
        Square => vec![
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ],
    }
    .into_iter()
}

/// Returns an iterator over the points in a neighborhood around a point
pub(crate) fn neighborhood_cells(
    t: Topology,
    width: Coor2DIndex,
    height: Coor2DIndex,
    index: Coor2D,
    n: Neighborhood,
) -> impl Iterator<Item = Coor2D> {
    all_adjacent_cells(n)
        .filter_map(move |direction| adjacent_cell(t, width, height, index.clone(), direction))
}

/// Returns manhattan distance
pub fn manhattan_distance(index1: Coor2D, index2: Coor2D) -> usize {
    #![allow(clippy::cast_sign_loss)]
    ((isize::try_from(index1.x).unwrap() - isize::try_from(index2.x).unwrap()).abs()
        + (isize::try_from(index1.y).unwrap() - isize::try_from(index2.y).unwrap()).abs())
        as usize
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn adjacent_bounded() {
        assert_eq!(adjacent_cell(Bounded, 3, 3, Coor2D::new(1, 0), North), None);
        assert_eq!(
            adjacent_cell(Bounded, 3, 3, Coor2D::new(1, 1), North),
            Some(Coor2D::new(1, 0))
        );
        assert_eq!(adjacent_cell(Bounded, 3, 3, Coor2D::new(2, 2), South), None);
        assert_eq!(
            adjacent_cell(Bounded, 3, 3, Coor2D::new(0, 0), South),
            Some(Coor2D::new(0, 1))
        );
        assert_eq!(adjacent_cell(Bounded, 3, 3, Coor2D::new(2, 2), East), None);
        assert_eq!(
            adjacent_cell(Bounded, 3, 3, Coor2D::new(1, 1), East),
            Some(Coor2D::new(2, 1))
        );
        assert_eq!(adjacent_cell(Bounded, 3, 3, Coor2D::new(0, 0), West), None);
        assert_eq!(
            adjacent_cell(Bounded, 3, 3, Coor2D::new(1, 1), West),
            Some(Coor2D::new(0, 1))
        );
    }

    #[test]
    fn adjacent_torus() {
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(1, 0), North),
            Some(Coor2D::new(1, 2))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(1, 1), North),
            Some(Coor2D::new(1, 0))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(2, 2), South),
            Some(Coor2D::new(2, 0))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(0, 0), South),
            Some(Coor2D::new(0, 1))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(2, 2), East),
            Some(Coor2D::new(0, 2))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(1, 1), East),
            Some(Coor2D::new(2, 1))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(0, 0), West),
            Some(Coor2D::new(2, 0))
        );
        assert_eq!(
            adjacent_cell(Torus, 3, 3, Coor2D::new(1, 1), West),
            Some(Coor2D::new(0, 1))
        );
    }

    #[test]
    fn edge() {
        assert!(is_edge(Bounded, 3, 3, Coor2D::new(1, 0)));
        assert!(is_edge(Bounded, 3, 3, Coor2D::new(0, 1)));
        assert!(is_edge(Bounded, 3, 3, Coor2D::new(1, 2)));
        assert!(is_edge(Bounded, 3, 3, Coor2D::new(2, 1)));
        assert!(!is_edge(Bounded, 3, 3, Coor2D::new(1, 1)));
        assert!(!is_edge(Torus, 3, 3, Coor2D::new(2, 1)));
    }

    #[test]
    fn pts() {
        assert_eq!(all_cells(3, 3).count(), 9);
    }

    #[test]
    fn neighborino() {
        assert_eq!(
            neighborhood_cells(Torus, 5, 5, Coor2D::new(0, 0), Square).collect::<HashSet<Coor2D>>(),
            HashSet::from([
                Coor2D::new(0, 4),
                Coor2D::new(0, 1),
                Coor2D::new(1, 0),
                Coor2D::new(4, 0),
                Coor2D::new(1, 4),
                Coor2D::new(1, 1),
                Coor2D::new(4, 4),
                Coor2D::new(4, 1)
            ]),
        );
        assert_eq!(
            neighborhood_cells(Bounded, 5, 5, Coor2D::new(0, 0), Square)
                .collect::<HashSet<Coor2D>>(),
            HashSet::from([Coor2D::new(0, 1), Coor2D::new(1, 0), Coor2D::new(1, 1)]),
        );
    }

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(
            manhattan_distance(Coor2D::new(11, 13), Coor2D::new(11, 13)),
            0
        );
        assert_eq!(
            manhattan_distance(Coor2D::new(11, 13), Coor2D::new(11, 12)),
            1
        );
        assert_eq!(
            manhattan_distance(Coor2D::new(11, 13), Coor2D::new(11, 14)),
            1
        );
        assert_eq!(
            manhattan_distance(Coor2D::new(11, 13), Coor2D::new(10, 13)),
            1
        );
        assert_eq!(
            manhattan_distance(Coor2D::new(11, 13), Coor2D::new(10, 12)),
            2
        );
    }
}
