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

use std::{cell::RefCell, collections::VecDeque};

#[cfg(debug_assertions)]
use std::fmt::{Display, Formatter};

use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day07/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day07/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse(file_name)
        .bfs()
        .filter(|f| f.is_folder())
        .map(FileSystemObject::size)
        .filter(|size| size < &100000)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let root = parse(file_name);

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

fn parse(file_name: &str) -> FileSystemObject {
    FileSystemObject::new_root().parse_lines(&mut utils::file_to_lines(file_name))
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum FileSystemObject {
    Directory {
        #[cfg(debug_assertions)]
        name: String,
        children: Vec<FileSystemObject>,
        cached_size: RefCell<Option<usize>>,
    },
    File {
        #[cfg(debug_assertions)]
        name: String,
        size: usize,
    },
}

#[cfg(debug_assertions)]
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
    fn new_file(_name: &str, size: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            FileSystemObject::File {
                name: _name.to_string(),
                size,
            }
        }
        #[cfg(not(debug_assertions))]
        {
            FileSystemObject::File { size }
        }
    }
    fn new_root() -> FileSystemObject {
        FileSystemObject::new_folder("/")
    }

    fn new_folder(_name: &str) -> Self {
        #[cfg(debug_assertions)]
        {
            FileSystemObject::Directory {
                name: _name.to_string(),
                children: vec![],
                cached_size: RefCell::new(None),
            }
        }
        #[cfg(not(debug_assertions))]
        {
            FileSystemObject::Directory {
                children: vec![],
                cached_size: RefCell::new(None),
            }
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
                cached_size.replace(None);
            }
            FileSystemObject::File { .. } => panic!("Cannot add child to file"),
        }
    }

    fn size(&self) -> usize {
        match self {
            FileSystemObject::Directory {
                children,
                cached_size,
                ..
            } => {
                if cached_size.borrow().is_none() {
                    cached_size.replace(Some(children.iter().map(|child| child.size()).sum()));
                }
                cached_size.borrow().unwrap()
            }
            FileSystemObject::File { size, .. } => *size,
        }
    }

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

    fn bfs_fn(&self, visit_fn: fn(&FileSystemObject)) {
        visit_fn(self);
        match self {
            FileSystemObject::Directory { children, .. } => {
                children.iter().for_each(|child| child.bfs_fn(visit_fn));
            }
            FileSystemObject::File { .. } => (),
        };
    }

    fn parse_lines(mut self, lines: &mut impl Iterator<Item = String>) -> FileSystemObject {
        while let Some(line) = lines.next() {
            if line.starts_with("$ cd ..") {
                break;
            } else if line.starts_with("$ cd ") {
                let new_subfolder_name = line.get("$ cd ".len()..).unwrap();
                let new_subfolder = FileSystemObject::new_folder(new_subfolder_name);
                self.add_child(new_subfolder.parse_lines(lines));
            } else if line.starts_with("dir ") {
                // skip
            } else if line.starts_with("$ ls") {
                // skip
            } else {
                let (size_str, filename) = line.split_whitespace().collect_tuple().unwrap();
                let file_size = size_str.parse::<usize>().unwrap();
                let new_file = FileSystemObject::new_file(filename, file_size);
                self.add_child(new_file);
            }
        }
        self
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
