#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(test)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]

use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::{Path, PathBuf},
    rc::Rc,
};

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day07/test.txt"));
    utils::with_measure("Part 2", || solve_part2("day07/test.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    let all_folder_sizes = collect_folder_sizes(file_name);
    println!("{:?}", all_folder_sizes);
    all_folder_sizes
        .into_iter()
        .map(|(_, size)| size)
        .filter(|size| size < &100000)
        .sum()
}

pub fn solve_part2(file_name: &str) -> isize {
    let total_disk_space = 70000000;
    let unused_space_limit = 30000000;
    let all_folder_sizes = collect_folder_sizes(file_name);

    let used_space: isize = all_folder_sizes[""] as isize;
    let free_space: isize = total_disk_space - used_space;
    let missing_free_space: isize = unused_space_limit - free_space;

    all_folder_sizes
        .into_iter()
        .map(|(_, size)| size as isize)
        .filter(|size| size >= &missing_free_space)
        .min()
        .unwrap()
}

fn collect_folder_sizes(file_name: &str) -> HashMap<String, usize> {
    let file_size_map = collect_filename_sizes(file_name);
    let mut all_folders = HashSet::new();
    for file_path in file_size_map.keys() {
        let x = file_path.split('/').collect_vec();
        for i in 0..x.len() - 1 {
            all_folders.insert(x[0..=i].iter().join("/"));
        }
    }

    let mut all_folders = HashSet::new();
    for file_path in file_size_map.keys() {
        let x = file_path.split('/').collect_vec();
        for i in 0..x.len() - 1 {
            all_folders.insert(x[0..=i].iter().join("/"));
        }
    }

    let mut all_folder_sizes: HashMap<String, usize> = HashMap::new();
    for (filepath, size) in file_size_map {
        for folder in all_folders.iter() {
            if filepath.starts_with(folder) {
                *all_folder_sizes.entry(folder.clone()).or_insert(0) += size;
            }
        }
    }

    all_folder_sizes
}

////////////////////////////////////////////////////////////////////////////////////

fn collect_filename_sizes(file_name: &str) -> HashMap<String, usize> {
    let mut folder_size_map = HashMap::new();
    let folder = &mut VecDeque::new();
    let lines = &mut utils::file_to_lines(file_name).skip(1);
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            folder.pop_back();
        } else if line.starts_with("$ cd ") {
            let new_folder = line.get("$ cd ".len()..).unwrap().to_string();
            folder.push_back(new_folder);
        } else if line.starts_with("dir ") {
            // skip
        } else if line.starts_with("$ ls") {
            // skip
        } else {
            let (size_str, filename) = line.split_whitespace().collect_tuple().unwrap();
            let file_size = size_str.parse::<usize>().unwrap();
            let folder_string = to_folder_string(folder) + "/" + filename;
            folder_size_map.insert(
                folder_string.clone(),
                file_size + folder_size_map.get(&folder_string).unwrap_or(&0),
            );
        }
    }
    folder_size_map
}

fn to_folder_string(folder: &VecDeque<String>) -> String {
    folder.iter().join("/")
}

////////////////////////////////////////////////////////////////////////////////////
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 95437);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1792222);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 24933642);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 1112963);
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
}
