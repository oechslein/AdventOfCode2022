use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

fn correct_folder(file_name: &str) -> PathBuf {
    // Allows cargo run to be called in dayXY and in root folder
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
    fs::read_to_string(correct_folder(file_name)).unwrap()
}

pub fn file_to_lines(file_name: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(correct_folder(file_name)).unwrap())
        .lines()
        .map(|line| line.unwrap())
}

/// Splits given String, trim each lines, filters empty lines and parse each line into wished type
pub fn parse_input_items<T>(contents: String) -> Vec<T>
where
    T: std::str::FromStr,
    <T>::Err: Debug, // Not sure why this is needed
{
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

pub fn parse_input(contents: String) -> Vec<String> {
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub fn parse_input_vec_vecs(contents: String) -> Vec<Vec<String>> {
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|substring| substring.split(' ').map(String::from).collect())
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
