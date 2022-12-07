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

use std::{rc::Rc, collections::{HashMap, VecDeque}, path::{PathBuf, Path}};

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
    /*
    let root = Tree::new("root".to_string(), 0, None);
    let mut curr_directory = &mut Rc::new(root);
    for line in utils::file_to_lines(file_name) {
        if line.starts_with("$ cd ..") {
            let var_name = curr_directory.parent.as_mut();
            curr_directory = var_name.unwrap();
        } else if line.starts_with("$ cd ") {
            let new_directory = Rc::new(Tree::new(
                line.get("$ cd ".len()..).unwrap().to_string(),
                0,
                Some(*curr_directory),
            ));
            curr_directory.add_child(new_directory);
            curr_directory = &mut new_directory;
        } else if line.starts_with("dir ") {
            //skip
        } else if line.starts_with("$ ls") {
            //skip
        } else {
            let (size_str, filename) = line.split_whitespace().collect_tuple().unwrap();
            let size = size_str.parse::<usize>().unwrap();
            let new_directory =
                Rc::new(Tree::new(filename.to_string(), size, Some(*curr_directory)));
            curr_directory.add_child(new_directory);
        }
    }
     */

    let mut folder_size_map: HashMap<String, usize> = HashMap::new();
    solve(&mut VecDeque::new(), &mut utils::file_to_lines(file_name), &mut folder_size_map);
    println!("{:?}", folder_size_map);
    42
}

fn solve(folder: &mut VecDeque<String>, lines: &mut impl Iterator<Item = String>, result: &mut HashMap<String, usize>) {
    let mut sum_size = 0;
    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ..") {
            println!("Insert {:?} {}", folder, sum_size);
            result.insert(folder.iter().join("/"), sum_size);
            folder.pop_back();
        } else if line.starts_with("$ cd ") {
            let new_folder = line.get("$ cd ".len()..).unwrap().to_string();
            println!("Enter folder '{}'", new_folder);
            folder.push_back(new_folder);
            solve(folder, lines, result);
        } else if line.starts_with("dir ") {
            // skip
        } else if line.starts_with("$ ls") {
            // skip
        } else {
            let (size_str, _) = line.split_whitespace().collect_tuple().unwrap();
            let file_size = size_str.parse::<usize>().unwrap();
            sum_size += file_size;
        }
    }
    println!("Insert final {:?} {}", folder, sum_size);
    result.insert(folder.iter().join("/"), sum_size);
}

pub fn solve_part2(file_name: &str) -> usize {
    42
}

////////////////////////////////////////////////////////////////////////////////////

// Tree struct with parent pointers

struct Tree {
    name: String,
    size: usize,
    children: Vec<Rc<Tree>>,
    parent: Option<Rc<Tree>>,
}

impl Tree {
    fn new(name: String, size: usize, parent: Option<Rc<Tree>>) -> Self {
        Tree {
            name,
            size,
            children: Vec::new(),
            parent,
        }
    }

    fn add_child(&mut self, child: Rc<Tree>) {
        self.children.push(child);
    }

    fn size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.size()).sum::<usize>()
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
        assert_eq!(solve_part1("test.txt"), 7);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 1702);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 19);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 3559);
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
