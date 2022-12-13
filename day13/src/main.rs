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

use std::{cmp::Ordering, fmt::Display};

use derive_more::Display;
use itertools::Itertools;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() {
    utils::with_measure("Part 1", || solve_part1("day13/input.txt"));
    utils::with_measure("Part 2", || solve_part2("day13/input.txt"));
}

////////////////////////////////////////////////////////////////////////////////////

pub fn solve_part1(file_name: &str) -> usize {
    parse_pairs(file_name)
        .into_iter()
        .enumerate()
        .filter(|(_, (packet1, packet2))| packet1.cmp(&packet2) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_part2(file_name: &str) -> usize {
    let mut packet_vec = parse(file_name);

    let div_packet1 = parse_line("[[2]]".to_string());
    let div_packet2 = parse_line("[[6]]".to_string());
    packet_vec.push(div_packet1.clone());
    packet_vec.push(div_packet2.clone());

    packet_vec
        .into_iter()
        .sorted()
        .enumerate()
        .filter(|(_, packet)| packet == &div_packet1 || packet == &div_packet2)
        .map(|(i, _)| i + 1)
        .product()
}

////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Display)]
#[display(fmt = "{}", _data)]
struct Packet {
    _data: PacketContent,
}

#[derive(Debug, Clone)]
enum PacketContent {
    Number(usize),
    List(Vec<PacketContent>),
}

impl PacketContent {
    fn to_list(&self) -> PacketContent {
        match self {
            PacketContent::List(_) => self.clone(),
            PacketContent::Number(n) => PacketContent::List(vec![PacketContent::Number(*n)]),
        }
    }
}

impl Eq for PacketContent {}

impl PartialEq for PacketContent {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for PacketContent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketContent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketContent::Number(n1), PacketContent::Number(n2)) => n1.cmp(n2),
            (PacketContent::List(l1), PacketContent::List(l2)) => {
                for (item1, item2) in l1.iter().zip(l2.iter()) {
                    match item1.cmp(&item2) {
                        Ordering::Equal => {}
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                    }
                }
                l1.len().cmp(&l2.len())
            }
            (PacketContent::List(_), PacketContent::Number(_)) => self.cmp(&other.to_list()),
            (PacketContent::Number(_), PacketContent::List(_)) => self.to_list().cmp(other),
        }
    }
}

impl Display for PacketContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketContent::Number(n) => write!(f, "{}", n),
            PacketContent::List(l) => {
                write!(f, "[")?;
                for (i, item) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////
fn parse_pairs(file_name: &str) -> Vec<(Packet, Packet)> {
    utils::file_to_lines(file_name)
        .filter(|line| !line.is_empty())
        .chunks(2)
        .into_iter()
        .map(|x| x.map(parse_line).collect_tuple().unwrap())
        .collect_vec()
}

fn parse(file_name: &str) -> Vec<Packet> {
    utils::file_to_lines(file_name)
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect_vec()
}

fn parse_line(line: String) -> Packet {
    let mut index = 0;
    Packet {
        _data: parse_line_rec(&line.chars().collect_vec()[..], &mut index),
    }
}

fn parse_line_rec(line: &[char], index: &mut usize) -> PacketContent {
    // println!(
    //     "parse_line_rec: {}, {}",
    //     line.iter().skip(*index).join(""),
    //     index
    // );
    if line[*index] == '[' {
        parse_packet_list(line, index)
    } else {
        parse_packet_number(line, index)
    }
}

fn parse_packet_number(line: &[char], index: &mut usize) -> PacketContent {
    // print!(
    //     "parse_packet_number: {}, {}",
    //     line.iter().skip(*index).join(""),
    //     index
    // );
    let end_pos = line
        .iter()
        .enumerate()
        .skip(*index)
        .find_or_last(|(_, c)| !c.is_digit(10))
        .unwrap()
        .0;
    // print!(" end_pos: {}", end_pos);
    let num = line[*index..end_pos].iter().join("").parse().unwrap();
    // println!(" => {}", num);
    let result = PacketContent::Number(num);
    *index = end_pos;
    result
}

fn parse_packet_list(line: &[char], index: &mut usize) -> PacketContent {
    // println!(
    //     "parse_packet_list: {}, {}",
    //     line.iter().skip(*index).join(""),
    //     index
    // );
    let end_pos = find_closing_bracket(&line, index);
    *index += 1; // skip the opening bracket
    let mut content = Vec::new();
    while *index < end_pos {
        content.push(parse_line_rec(line, index));
        if *index < end_pos {
            assert_eq!(line[*index], ',');
            *index += 1; // skip the comma
        }
    }
    *index += 1; // skip the closing bracket
                 // println!("parse_packet_list => {:?}", content);
    PacketContent::List(content)
}

fn find_closing_bracket(line: &[char], index: &usize) -> usize {
    // print!(
    //     "find_closing_bracket: {}, {}",
    //     line.iter().skip(*index).join(""),
    //     index
    // );
    let mut open_bracket_count: usize = 0;
    for new_index in *index..line.len() {
        match line[new_index] {
            '[' => open_bracket_count += 1,
            ']' => {
                open_bracket_count -= 1;
                if open_bracket_count == 0 {
                    // println!(" => {}", new_index);
                    return new_index;
                }
            }
            _ => (),
        }
    }
    panic!("No matching closing bracket found");
}

////////////////////////////////////////////////////////////////////////////////////

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test1() {
        assert_eq!(solve_part1("test.txt"), 13);
    }

    #[test]
    fn verify1() {
        assert_eq!(solve_part1("input.txt"), 5555);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part2("test.txt"), 140);
    }

    #[test]
    fn verify2() {
        assert_eq!(solve_part2("input.txt"), 22852);
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
