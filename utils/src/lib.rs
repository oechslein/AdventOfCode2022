//! Various Utility functions

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![forbid(missing_docs)]

use std::cmp::Reverse;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use itertools::Itertools;

/// Allows cargo run to be called in dayXY and in root folder
fn correct_folder(file_name: &str) -> PathBuf {
    let mut file_path = PathBuf::from(file_name);
    if !file_path.exists() {
        if let Some(file_name) = file_path.file_name() {
            file_path = PathBuf::from(file_name);
        }
    }
    file_path
}

/// Reads a file and return its content as a string
pub fn file_to_string(file_name: &str) -> String {
    fs::read_to_string(correct_folder(file_name))
        .unwrap()
        .replace("\r\n", "\n")
}

/// Reads a file, splits per newline and returns an iterator
pub fn file_to_lines(file_name: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(correct_folder(file_name)).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

/// Converts an iterator with str to an iterator with "T"
pub fn convert_str_iter<'a, T>(
    input: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = T> + 'a
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    input.map(|x| str_to(x))
}

/// Converts an str to a type (and unwraps it)
pub fn str_to<T>(input: &str) -> T
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    str::parse::<T>(input).unwrap()
}

/// Converts item back from Reverse(item)
pub fn unreverse<T>(reversed_item: Reverse<T>) -> T {
    reversed_item.0
}

/// Splits given String split into chunks separated by empty lines
pub fn split_by_empty_lines<'a, T>(contents: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    convert_str_iter::<T>(contents.split("\n\n"))
}

/// Splits given String split into chunks separated by empty lines
pub fn split_by_newline<'a, T>(contents: &'a str) -> impl Iterator<Item = T> + 'a
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    convert_str_iter::<T>(contents.lines())
}

/// Splits given String, trim each lines, filters empty lines and parse each line into wished type
pub fn parse_input_items<T>(contents: String) -> Vec<T>
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

/// Runs given function, prints result and used duration
pub fn with_measure<T: Debug>(title: &str, f: fn() -> T) -> T {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();
    println!(
        "{} result: {:?} (elapsed time is: {:?})",
        title, res, duration
    );
    res
}

/*
pub fn k_largest_old<T>(input: impl Iterator<Item = T>, k: usize) -> impl Iterator<Item = T>
where T: Ord {
    use std::collections::BinaryHeap;
    let mut h = BinaryHeap::with_capacity(k);
    for item in input {
        h.push(std::cmp::Reverse(item));
        if h.len() > k {
             h.pop();
        }
    }

    h.into_iter().map(|rev| rev.0).rev()
}
*/

