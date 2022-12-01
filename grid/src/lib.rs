//! 2D Grid implementations

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![deny(missing_docs)]

#[macro_use]
extern crate derive_builder;
extern crate num_derive;

pub mod grid_array;
pub mod grid_iteration;
pub mod grid_types;
