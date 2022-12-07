//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(unused_must_use)]
#![feature(test)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]

use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
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
    let root = create_tree(file_name);
    println!("{}", root);

    root.bfs()
        .filter(|f| f.is_folder())
        .map(FileSystemObject::size)
        .filter(|size| size < &100000)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let root = create_tree(file_name);
    println!("{:?}", root);

    let total_disk_space = 70000000;
    let unused_space_limit = 30000000;

    let used_space = root.size();
    let free_space = total_disk_space - used_space;
    let missing_free_space = unused_space_limit - free_space;

    root.bfs()
        .filter(|f| f.is_folder())
        .map(FileSystemObject::size)
        .filter(|size| size >= &missing_free_space)
        .min()
        .unwrap()
}

////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum FileSystemObject {
    Directory {
        name: String,
        children: Vec<FileSystemObject>,
        cached_size: Option<usize>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl Display for FileSystemObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemObject::Directory { name, children, .. } => {
                writeln!(f, "Directory: '{}'", name)?;
                for child in children {
                    write!(f, "  {}", child)?;
                }
                Ok(())
            }
            FileSystemObject::File { name, size } => {
                writeln!(f, "File: '{}' ({})", name, size)
            }
        }
    }
}

impl FileSystemObject {
    fn new_file(name: String, size: usize) -> Self {
        FileSystemObject::File { name, size }
    }

    fn new_folder(name: String) -> Self {
        FileSystemObject::Directory {
            name,
            children: vec![],
            cached_size: None,
        }
    }

    fn add_child(&mut self, child: FileSystemObject) {
        match self {
            FileSystemObject::Directory {
                children,
                cached_size,
                ..
            } => {
                children.push(child);
                *cached_size = None
            }
            FileSystemObject::File { .. } => panic!("Cannot add child to file"),
        }
    }

    fn size(&self) -> usize {
        match self {
            FileSystemObject::Directory { children, .. } => {
                children.iter().map(|child| child.size()).sum()
            }
            FileSystemObject::File { size, .. } => *size,
        }
    }

    /*
    fn size_cached(&self) -> usize {
        match self {
            FileSystemObject::Directory { children, cached_size, .. } => {
                if cached_size.is_none() {
                    let folder_size = children.iter_mut().map(|child| child.size()).sum();
                    *cached_size = Some(folder_size);
                }
                cached_size.unwrap()
            }
            FileSystemObject::File { size, .. } => *size,
        }
    }
    */

    fn bfs(&self) -> impl Iterator<Item = &FileSystemObject> {
        let mut queue = VecDeque::new();
        let mut result = vec![];
        queue.push_back(self);
        while let Some(curr) = queue.pop_front() {
            result.push(curr);
            if let FileSystemObject::Directory { children, .. } = curr {
                for child in children {
                    queue.push_back(child);
                }
            }
        }
        result.into_iter()
    }

    fn is_file(&self) -> bool {
        match self {
            FileSystemObject::Directory { .. } => false,
            FileSystemObject::File { .. } => true,
        }
    }

    fn is_folder(&self) -> bool {
        !self.is_file()
    }
}

fn create_tree(file_name: &str) -> FileSystemObject {
    fn create_tree_rec(lines: &mut impl Iterator<Item = String>, curr_tree: &mut FileSystemObject) {
        while let Some(line) = lines.next() {
            if line.starts_with("$ cd ..") {
                return;
            } else if line.starts_with("$ cd ") {
                let new_folder = line.get("$ cd ".len()..).unwrap().to_string();
                let mut new_tree = FileSystemObject::new_folder(new_folder);
                create_tree_rec(lines, &mut new_tree);
                curr_tree.add_child(new_tree);
            } else if line.starts_with("dir ") {
                // skip
            } else if line.starts_with("$ ls") {
                // skip
            } else {
                let (size_str, filename) = line.split_whitespace().collect_tuple().unwrap();
                let file_size = size_str.parse::<usize>().unwrap();

                let new_tree = FileSystemObject::new_file(filename.to_string(), file_size);
                curr_tree.add_child(new_tree);
            }
        }
    }
    let lines = &mut utils::file_to_lines(file_name);
    let mut root = FileSystemObject::new_folder("".to_string());
    create_tree_rec(lines, &mut root);
    root
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
