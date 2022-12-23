//! Grid based on a hash map

use fxhash::FxHashMap;

use crate::grid_types::{Coor2DMut};

use super::grid_iteration;
use super::grid_types::Neighborhood;

use itertools::Itertools;

/// GridArray
#[allow(missing_docs)]
#[derive(Builder, Clone, PartialEq, Debug)]
pub struct GridHashMap<T>
where
    T: Default + Clone + std::fmt::Display,
{
    #[builder(default = "Neighborhood::Square")]
    neighborhood: Neighborhood,

    #[builder(default = "FxHashMap::default()")]
    data: FxHashMap<Coor2DMut<isize>, T>,
}

impl GridHashMap<char> {
    /// from newline separated string
    /// #Panics panics if the string is not a rectangle
    pub fn from_newline_separated_string(neighborhood: Neighborhood, input: &str) -> Self {
        let width = input
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n' || *x == '\r')
            .take(1)
            .next()
            .unwrap()
            .0;
        let data = input
            .chars()
            .filter(|x| *x != '\n' && *x != '\r')
            .collect_vec();
        GridHashMap::from_1d_vec(neighborhood, width, data)
    }
}

impl<T> GridHashMap<T>
where
    T: Default + Clone + std::fmt::Display,
{
    /// from 1d vector
    pub fn from_1d_vec(neighborhood: Neighborhood, width: usize, data: Vec<T>) -> Self {
        debug_assert_eq!(
            (data.len()) % width,
            0,
            "data.len()={} width={}",
            data.len(),
            width
        );
        GridHashMap {
            neighborhood,
            data: data
                .into_iter()
                .enumerate()
                .map(|(index, cell)| {
                    let index: isize = index.try_into().unwrap();
                    let width: isize = width.try_into().unwrap();
                    let x = Coor2DMut::new(index % width, index / width);
                    (x, cell)
                })
                .collect(),
        }
    }

    /// get_neighborhood
    pub fn get_neighborhood(&self) -> Neighborhood {
        self.neighborhood
    }

    /// get min coor
    pub fn get_min_max(&self) -> (Coor2DMut<isize>, Coor2DMut<isize>) {
        self.data.keys().fold(
            (
                Coor2DMut::new(isize::MAX, isize::MAX),
                Coor2DMut::new(isize::MIN, isize::MIN),
            ),
            |(min_coor, max_coor), coor| (min_coor.min(coor), max_coor.max(coor)),
        )
    }

    /// get reference to element on x, y
    pub fn get(&self, coor: &Coor2DMut<isize>) -> Option<&T> {
        self.data.get(coor)
    }

    /// get mutable reference element on x, y
    pub fn get_mut(&mut self, coor: &Coor2DMut<isize>) -> Option<&mut T> {
        self.data.get_mut(coor)
    }

    /// set new element on x, y and return old element
    pub fn set(&mut self, coor: Coor2DMut<isize>, new_value: T) -> Option<T> {
        self.data.insert(coor, new_value)
    }

    /// clear element on x, y and return old element
    pub fn remove(&mut self, coor: &Coor2DMut<isize>) -> Option<T> {
        self.data.remove(coor)
    }

    /// return all indexes
    pub fn all_indexes(&self) -> impl Iterator<Item = Coor2DMut<isize>> + '_ {
        self.data.keys().cloned()
    }

    /// return all neighbor indexes (based on neighborhood)
    pub fn neighborhood_cell_indexes<'a>(
        &self,
        coor: &'a Coor2DMut<isize>,
    ) -> impl Iterator<Item = Coor2DMut<isize>> + 'a {
        grid_iteration::all_adjacent_cells(self.neighborhood)
            .map(|direction| coor.clone() + direction.diff_coor())
    }

    fn map_indexes_to_cells(
        &self,
        it: impl Iterator<Item = Coor2DMut<isize>>,
    ) -> impl Iterator<Item = (Coor2DMut<isize>, Option<&T>)> {
        it.map(|coor| (coor.clone(), self.get(&coor)))
    }

    /// all data
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.values()
    }

    /// return all elements
    pub fn all_cells(&self) -> impl Iterator<Item = (Coor2DMut<isize>, Option<&T>)> {
        self.map_indexes_to_cells(self.all_indexes())
    }

    /// return all neighbor elements (based on topology and neighborhood)
    pub fn neighborhood_cells<'a>(
        &'a self,
        coor: &'a Coor2DMut<isize>,
    ) -> impl Iterator<Item = (Coor2DMut<isize>, Option<&T>)> + 'a {
        self.map_indexes_to_cells(self.neighborhood_cell_indexes(coor))
    }

    /// Print grid
    pub fn print(&self, default: &char) {
        let min_max = self.get_min_max();
        //println!("min: {:?}, max: {:?}", min_max.0, min_max.1);
        for y in min_max.0.y..=min_max.1.y {
            for x in min_max.0.x..=min_max.1.x {
                if let Some(ch) = self.get(&Coor2DMut::new(x, y)) {
                    print!("{ch}");
                } else {
                    print!("{default}");
                }
            }
            println!();
        }
    }

    /// Print grid
    pub fn print_with_mapping(&self, map_fn: impl Fn(Option<&T>) -> char) {
        let min_max = self.get_min_max();
        //println!("min: {:?}, max: {:?}", min_max.0, min_max.1);
        for y in min_max.0.y..=min_max.1.y {
            for x in min_max.0.x..=min_max.1.x {
                let cell = map_fn(self.get(&Coor2DMut::new(x, y)));
                print!("{cell}");
            }
            println!();
        }
    }
}
