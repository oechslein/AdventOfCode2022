//! Grid based on a vector

use std::mem::{replace, swap};

use itertools::Itertools;

use crate::grid_iteration::{is_corner, is_edge};

use super::grid_iteration;
use super::grid_types::{Coor, CoorIndex, Neighborhood, Topology};

/// GridArray
#[allow(missing_docs)]
#[derive(Builder, Clone, PartialEq, Debug)]
pub struct GridArray<T: Default + Clone + std::fmt::Display> {
    /// width of the grid
    width: CoorIndex,
    height: CoorIndex,

    #[builder(default = "Topology::Bounded")]
    topology: Topology,
    #[builder(default = "Neighborhood::Square")]
    neighborhood: Neighborhood,

    //    #[builder(setter(skip), default = "self.create_data_vec()")]
    #[builder(default = "self.create_data_vec()")]
    data: Vec<T>,
}

impl<T: Default + Clone + std::fmt::Display> GridArrayBuilder<T> {
    fn create_data_vec(&self) -> Vec<T> {
        vec![T::default(); self.width.unwrap() * self.height.unwrap()]
    }
}

impl GridArray<char> {
    /// from newline separated string 
    pub fn from_newline_separated_string(
        topology: Topology,
        neighborhood: Neighborhood,
        input: String,
    ) -> Self {
        let width = input
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n')
            .take(1)
            .next()
            .unwrap()
            .0
            - 1;
        let data = input
            .chars()
            .filter(|x| *x != '\n' && *x != '\r')
            .collect_vec();
        GridArray::from_1d_vec(topology, neighborhood, width, data)
    }
}

impl<T: Default + Clone + std::fmt::Display> GridArray<T> {
    #[allow(dead_code)]
    fn create_data_vec(&self) -> Vec<T> {
        vec![T::default(); self.width * self.height]
    }

    /// from 1d vector
    pub fn from_1d_vec(
        topology: Topology,
        neighborhood: Neighborhood,
        width: CoorIndex,
        data: Vec<T>,
    ) -> Self {
        assert_eq!(
            (data.len()) % width,
            0,
            "data.len()={} width={}",
            data.len(),
            width
        );
        GridArray {
            width,
            height: data.len() / width,
            topology,
            neighborhood,
            data,
        }
    }

    /// from 2d vector
    pub fn from_2d_vec(topology: Topology, neighborhood: Neighborhood, data: Vec<Vec<T>>) -> Self {
        assert!(data.len() > 0);
        GridArray {
            width: data[0].len(),
            height: data.len(),
            topology,
            neighborhood,
            data: data.into_iter().flatten().collect(),
        }
    }

    #[allow(unused_comparisons)]
    fn _check_index(x: CoorIndex, y: CoorIndex, width: usize, height: usize) -> bool {
        #![allow(clippy::absurd_extreme_comparisons)]
        (0 <= x && x < width) && (0 <= y && y < height)
    }

    #[allow(unused_comparisons)]
    fn check_index(&self, x: CoorIndex, y: CoorIndex) -> bool {
        GridArray::<T>::_check_index(x, y, self.width, self.height)
    }

    fn _index_to_vec_index(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    fn index_to_vec_index(&self, x: usize, y: usize) -> usize {
        assert!(self.check_index(x, y));
        GridArray::<T>::_index_to_vec_index(x, y, self.width)
    }

    /// get_width
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// get_height
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// get_topology
    pub fn get_topology(&self) -> Topology {
        self.topology
    }

    /// get_neighborhood
    pub fn get_neighborhood(&self) -> Neighborhood {
        self.neighborhood
    }

    /// is_edge
    pub fn is_edge(&self, x: CoorIndex, y: CoorIndex) -> bool {
        assert!(self.check_index(x, y));
        is_edge(self.topology, self.width, self.height, (x, y))
    }

    /// is_corner
    pub fn is_corner(&self, x: CoorIndex, y: CoorIndex) -> bool {
        assert!(self.check_index(x, y));
        is_corner(self.topology, self.width, self.height, (x, y))
    }

    /// get reference to element on x, y
    pub fn get(&self, x: CoorIndex, y: CoorIndex) -> Option<&T> {
        if self.check_index(x, y) {
            Some(&self.data[self.index_to_vec_index(x, y)])
        } else {
            None
        }
    }

    /// get reference to element on x, y
    pub fn get_unchecked(&self, x: CoorIndex, y: CoorIndex) -> &T {
        &self.data[self.index_to_vec_index(x, y)]
    }

    /// get mutable reference element on x, y
    pub fn get_mut(&mut self, x: CoorIndex, y: CoorIndex) -> Option<&mut T> {
        if self.check_index(x, y) {
            let vec_index = self.index_to_vec_index(x, y);
            Some(&mut self.data[vec_index])
        } else {
            None
        }
    }

    /// set new element on x, y and return old element
    pub fn set(&mut self, x: CoorIndex, y: CoorIndex, new_value: T) -> T {
        assert!(self.check_index(x, y));
        self.set_unchecked(x, y, new_value)
    }

    fn set_unchecked(&mut self, x: usize, y: usize, new_value: T) -> T {
        let vec_index = self.index_to_vec_index(x, y);
        replace(&mut self.data[vec_index], new_value)
    }

    /// set new element on x, y based on vector
    pub fn set_from_vec(&mut self, new_values: &Vec<Vec<T>>) {
        assert_eq!(new_values.len(), self.height);
        assert_eq!(new_values[0].len(), self.width);
        for (y, row) in new_values.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.set_unchecked(x, y, cell.clone());
            }
        }
    }

    /// return all indexes
    pub fn all_indexes(&self) -> impl Iterator<Item = Coor> {
        grid_iteration::all_cells(self.width, self.height)
    }

    /// return all neighbor indexes (based on topology and neighborhood)
    pub fn neighborhood_cell_indexes(
        &self,
        x: CoorIndex,
        y: CoorIndex,
    ) -> impl Iterator<Item = Coor> {
        grid_iteration::neighborhood_cells(
            self.topology,
            self.width,
            self.height,
            (x, y),
            self.neighborhood,
        )
    }

    fn map_indexes_to_cells(
        &self,
        it: impl Iterator<Item = Coor>,
    ) -> impl Iterator<Item = (Coor, &T)> {
        it.map(|coor| (coor, self.get_unchecked(coor.0, coor.1)))
    }

    // map_indexes_to_cells_mut not possible to implement (multiple borrows of self_data)

    /// all data
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// return all elements
    pub fn all_cells(&self) -> impl Iterator<Item = (Coor, &T)> {
        self.map_indexes_to_cells(self.all_indexes())
    }

    /// return all neighbor elements (based on topology and neighborhood)
    pub fn neighborhood_cells(
        &self,
        x: CoorIndex,
        y: CoorIndex,
    ) -> impl Iterator<Item = (Coor, &T)> {
        self.map_indexes_to_cells(self.neighborhood_cell_indexes(x, y))
    }

    /* The tests showed that this code isn#t stable
    pub fn all_cells_mut(
        &mut self,
    ) -> impl Iterator<Item = (CellIndexCoorType, CellIndexCoorType, &mut T)> {
        self.all_indexes()
            .zip(self._data.iter_mut())
            .map(|((x, y), cell)| (x, y, cell))
    }
    pub fn neighborhood_cells_mut(
        &mut self,
        x: CellIndexCoorType,
        y: CellIndexCoorType,
    ) -> impl Iterator<Item = (CellIndexCoorType, CellIndexCoorType, &mut T)> {
        // looping over the neighbor_cells and calling get_mut didn't work.
        let neighbor_cells: HashSet<CellIndexType> =
            HashSet::from_iter(self.neighborhood_cell_indexes(x, y));
        self.all_cells_mut()
            .filter(move |(x, y, _)| neighbor_cells.contains(&(*x, *y)))
    }
    */

    /// Print grid
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}*", self.get_unchecked(x, y));
            }
            println!();
        }
    }

    fn swap(&mut self, x1: CoorIndex, y1: CoorIndex, x2: CoorIndex, y2: CoorIndex) {
        if (x1, y1) != (x2, y2) {
            let vec_index1 = self.index_to_vec_index(x1, y1);
            let vec_index2 = self.index_to_vec_index(x2, y2);
            self.data.swap(vec_index1, vec_index2);
        }
    }

    /// flip_horizontal
    pub fn flip_horizontal(&mut self) {
        for x in 0..self.width / 2 {
            for y in 0..self.height {
                self.swap(x, y, self.width - x - 1, y);
            }
        }
    }

    /// flip_vertical
    pub fn flip_vertical(&mut self) {
        for y in 0..self.height / 2 {
            for x in 0..self.width {
                self.swap(x, y, x, self.height - y - 1);
            }
        }
    }

    fn _transform(&mut self, coors: impl Iterator<Item = Coor>, swap_width_height: bool) {
        let new_data = coors
            .map(|(x, y)| self.get_unchecked(x, y))
            .cloned()
            .collect_vec();
        if swap_width_height {
            swap(&mut self.width, &mut self.height);
        }
        self.data = new_data;
    }

    /// transpose
    pub fn transpose(&mut self) {
        self._transform((0..self.width).cartesian_product(0..self.height), true);
    }

    /// rotate_cw
    pub fn rotate_cw(&mut self) {
        // rotate clockwise by 90°
        self._transform(
            (0..self.width).cartesian_product((0..self.height).rev()),
            true,
        );
    }

    /// rotate_ccw
    pub fn rotate_ccw(&mut self) {
        // rotate counter clockwise by 90°
        self._transform(
            ((0..self.width).rev()).cartesian_product(0..self.height),
            true,
        );
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn build_common_array() -> GridArrayBuilder<isize> {
        GridArrayBuilder::default().width(4).height(5).to_owned()
    }

    fn build_common_bounded_array() -> GridArrayBuilder<isize> {
        build_common_array().topology(Topology::Bounded).to_owned()
    }

    fn build_common_torus_array() -> GridArrayBuilder<isize> {
        build_common_array().topology(Topology::Torus).to_owned()
    }

    fn populate_with_enumerated(a: &mut GridArray<isize>) {
        for (i, (x, y)) in a.all_indexes().enumerate() {
            a.set(x, y, i as isize);
        }
        assert_eq!(a.get(0, 0), Some(&0));
    }

    fn standard_tests(a: &mut GridArray<isize>) {
        assert_eq!(a.get(0, 0), Some(&0));
        assert_eq!(a.get(10, 11), None);

        assert_eq!(a.all_indexes().count(), a.width * a.height);
        assert_eq!(a.all_indexes().dedup().count(), a.width * a.height);
        assert_eq!(a.all_cells().count(), a.width * a.height);

        a.set(3, 2, -42);
        assert_eq!(a.get(3, 2), Some(&-42));

        {
            let mut new_value = 42;
            swap(a.get_mut(2, 3).unwrap(), &mut new_value);
            assert_eq!(a.get(2, 3), Some(&42));
            let (_, cell) = a
                .all_cells()
                .find(|((x, y), _)| (x, y) == (&2, &3))
                .unwrap();
            assert_eq!(cell, &42);
        }

        /* See above not stable
        a.print();
        {
            let mut new_value = -11;
            let (_, _, cell) = a
                .all_cells_mut()
                .find(|(x, y, _)| (x, y) == (&2, &3))
                .unwrap();
            swap(cell, &mut new_value);

            let cell_value: isize = *cell;
            println!("{} , {} , {}", a.get(2, 3).unwrap(), cell_value, new_value);
            a.print();
            assert_eq!(a.get(2, 3), Some(&-11));
        }
        */

        populate_with_enumerated(a);

        //a.print();
        a.flip_horizontal();
        assert_eq!(a.get(0, 0), Some(&15));
        //println!();
        //a.print();
        a.flip_vertical();
        assert_eq!(a.get(0, 0), Some(&19));
        //println!();

        a.transpose();
        assert_eq!(a.get(0, 0), Some(&19));
        assert_eq!(a.get(a.width - 1, 0), Some(&15));

        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.transpose();
        new_a.transpose();
        assert_eq!(&new_a, a);

        check_rotate_cw(a);
        check_rotate_ccw(a);
    }

    fn check_rotate_cw(a: &mut GridArray<isize>) {
        populate_with_enumerated(a);
        a.rotate_cw();
        assert_eq!(a.get(0, 0), Some(&3));
        assert_eq!(a.get(a.width - 1, a.height - 1), Some(&16));
        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.rotate_cw();
        new_a.rotate_cw();
        new_a.rotate_cw();
        new_a.rotate_cw();
        assert_eq!(&new_a, a);
    }

    fn check_rotate_ccw(a: &mut GridArray<isize>) {
        populate_with_enumerated(a);
        a.rotate_ccw();
        assert_eq!(a.get(0, 0), Some(&15));
        assert_eq!(a.get(a.width - 1, a.height - 1), Some(&4));
        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        assert_eq!(&new_a, a);
        new_a.rotate_cw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_cw();
        assert_eq!(&new_a, a);
    }

    #[test]
    fn grid_bounded_square_array_tests() {
        let mut a: GridArray<isize> = build_common_bounded_array()
            .neighborhood(Neighborhood::Square)
            .build()
            .unwrap();
        standard_tests(&mut a);

        assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 8);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 3);
            }
        }
    }

    #[test]
    fn grid_bounded_orthogonal_array_tests() {
        let mut a: GridArray<isize> = build_common_bounded_array()
            .neighborhood(Neighborhood::Orthogonal)
            .build()
            .unwrap();
        standard_tests(&mut a);
        assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 4);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 2);
            }
        }
    }

    #[test]
    fn grid_torus_square_array_tests() {
        let mut a: GridArray<isize> = build_common_torus_array()
            .neighborhood(Neighborhood::Square)
            .build()
            .unwrap();
        standard_tests(&mut a);

        assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 8);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 8);
            }
        }
    }

    #[test]
    fn grid_torus_orthogonal_array_tests() {
        let mut a: GridArray<isize> = build_common_torus_array()
            .neighborhood(Neighborhood::Orthogonal)
            .build()
            .unwrap();
        standard_tests(&mut a);

        assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 4);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 4);
            }
        }
    }
}
