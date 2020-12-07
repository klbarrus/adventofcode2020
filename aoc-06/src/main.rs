// Advent of Code 2020 Day 06

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn count_yes(vhs: &Vec<HashSet<char>>) -> usize {
    if vhs.len() == 1 {
        return vhs[0].len();
    }

    let mut first = vhs[0].clone();

    for hs in vhs.iter().skip(1) {
        let isect = first.intersection(hs).cloned().collect::<HashSet<_>>();
        first = isect;
    }

    first.len()
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_string(&"aoc-06.txt")?;

    let part1_data = lines
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&ch| !ch.is_whitespace())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:#?}", part1_data);

    let part1 = part1_data.iter().map(|v| v.len()).sum1::<usize>().unwrap();

    println!("Part 1: {}", part1);

    let part2_data = lines
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|vote| {
                    vote.chars()
                        .filter(|&ch| !ch.is_whitespace())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:#?}", part2_data);

    let part2 = part2_data
        .iter()
        .map(|v| count_yes(&v))
        .sum1::<usize>()
        .unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}
