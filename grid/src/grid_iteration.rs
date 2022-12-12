//! Manipulating 2d grids

use super::grid_types::{
    Coor, CoorIndex, Direction, Direction::*, Neighborhood, Neighborhood::*, Topology, Topology::*,
};

/// Get the adjacent point to a point in a given direction
pub(crate) fn adjacent_cell(
    t: Topology,
    width: CoorIndex,
    height: CoorIndex,
    index: Coor,
    d: Direction,
) -> Option<Coor> {
    let (x, y) = index;
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
                North => Some((x, y.checked_sub(1)?)),
                South => {
                    if y + 1 < height {
                        Some((x, y + 1))
                    } else {
                        None
                    }
                }
                East => {
                    if x + 1 < width {
                        Some((x + 1, y))
                    } else {
                        None
                    }
                }
                West => Some((x.checked_sub(1)?, y)),

                _ => panic!(), // already handled above
            },
            Torus => match d {
                North => Some((x, y.checked_sub(1).unwrap_or(height - 1))),
                South => Some((x, (y + 1) % height)),
                East => Some(((x + 1) % width, y)),
                West => Some((x.checked_sub(1).unwrap_or(width - 1), y)),

                _ => panic!(), // already handled above
            },
        },
    }
}

/// Is a given point on an edge of a grid
pub(crate) fn is_edge(t: Topology, width: CoorIndex, height: CoorIndex, index: Coor) -> bool {
    let (x, y) = index;
    t == Topology::Bounded && (x == 0 || x + 1 == width || y == 0 || y + 1 == height)
}

/// Is a given point a corner of a grid
pub(crate) fn is_corner(t: Topology, width: CoorIndex, height: CoorIndex, index: Coor) -> bool {
    let (x, y) = index;
    t == Topology::Bounded && (x == 0 || x + 1 == width) && (y == 0 || y + 1 == height)
}

/// Returns an iterator over the points of a grid
pub(crate) fn all_cells(width: CoorIndex, height: CoorIndex) -> impl Iterator<Item = Coor> {
    (0..width).flat_map(move |x| (0..height).map(move |y| (x, y)))
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
    width: CoorIndex,
    height: CoorIndex,
    index: Coor,
    n: Neighborhood,
) -> impl Iterator<Item = Coor> {
    all_adjacent_cells(n)
        .filter_map(move |direction| adjacent_cell(t, width, height, index, direction))
}

/// Returns manhattan distance
pub fn manhattan_distance(index1: Coor, index2: Coor) -> usize {
    #![allow(clippy::cast_sign_loss)]
    ((isize::try_from(index1.0).unwrap() - isize::try_from(index2.0).unwrap()).abs()
        + (isize::try_from(index1.1).unwrap() - isize::try_from(index2.1).unwrap()).abs())
        as usize
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn adjacent_bounded() {
        assert_eq!(adjacent_cell(Bounded, 3, 3, (1, 0), North), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, (1, 1), North), Some((1, 0)));
        assert_eq!(adjacent_cell(Bounded, 3, 3, (2, 2), South), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, (0, 0), South), Some((0, 1)));
        assert_eq!(adjacent_cell(Bounded, 3, 3, (2, 2), East), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, (1, 1), East), Some((2, 1)));
        assert_eq!(adjacent_cell(Bounded, 3, 3, (0, 0), West), None);
        assert_eq!(adjacent_cell(Bounded, 3, 3, (1, 1), West), Some((0, 1)));
    }

    #[test]
    fn adjacent_torus() {
        assert_eq!(adjacent_cell(Torus, 3, 3, (1, 0), North), Some((1, 2)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (1, 1), North), Some((1, 0)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (2, 2), South), Some((2, 0)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (0, 0), South), Some((0, 1)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (2, 2), East), Some((0, 2)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (1, 1), East), Some((2, 1)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (0, 0), West), Some((2, 0)));
        assert_eq!(adjacent_cell(Torus, 3, 3, (1, 1), West), Some((0, 1)));
    }

    #[test]
    fn edge() {
        assert!(is_edge(Bounded, 3, 3, (1, 0)));
        assert!(is_edge(Bounded, 3, 3, (0, 1)));
        assert!(is_edge(Bounded, 3, 3, (1, 2)));
        assert!(is_edge(Bounded, 3, 3, (2, 1)));
        assert!(!is_edge(Bounded, 3, 3, (1, 1)));
        assert!(!is_edge(Torus, 3, 3, (2, 1)));
    }

    #[test]
    fn pts() {
        assert_eq!(all_cells(3, 3).count(), 9);
    }

    #[test]
    fn neighborino() {
        assert_eq!(
            neighborhood_cells(Torus, 5, 5, (0, 0), Square).collect::<HashSet<Coor>>(),
            HashSet::from([
                (0, 4),
                (0, 1),
                (1, 0),
                (4, 0),
                (1, 4),
                (1, 1),
                (4, 4),
                (4, 1)
            ]),
        );
        assert_eq!(
            neighborhood_cells(Bounded, 5, 5, (0, 0), Square).collect::<HashSet<Coor>>(),
            HashSet::from([(0, 1), (1, 0), (1, 1)]),
        );
    }

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(manhattan_distance((11, 13), (11, 13)), 0);
        assert_eq!(manhattan_distance((11, 13), (11, 12)), 1);
        assert_eq!(manhattan_distance((11, 13), (11, 14)), 1);
        assert_eq!(manhattan_distance((11, 13), (10, 13)), 1);
        assert_eq!(manhattan_distance((11, 13), (10, 12)), 2);
    }
}
