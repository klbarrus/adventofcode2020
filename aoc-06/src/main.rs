// Advent of Code 2020 Day 06

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn one_line(info: &Vec<String>) -> String {
    let response = info.join(" ");
    response
}

// turn the incoming string into bytes, drop the space between groups,
// sort, and remove duplicates
// the result is the questions someone answered yes to
// (i.e. unique occurences of letters/bytes)
fn remove_duplicates(s: &str) -> Vec<u8> {
    let mut b = s.bytes().filter(|&b| !(b == b' ')).collect::<Vec<u8>>();
    b.sort();
    b.dedup();

    b
}

// turn the incoming string into vectors of hashsets
// i.e. "abc" -> HashSet with a,b,c inserted
//      "ab ac" -> HashSet with a,b inserted, HashSet with a,c inserted
fn make_set(s: &str) -> Vec<HashSet<char>> {
    let vhsc = s
        .split(' ')
        .map(|x| {
            let mut hs = HashSet::new();
            for y in x.chars() {
                hs.insert(y);
            }
            // doesn't fully work due to iterator laziness
            //x.chars().map(|y| hs.insert(y));
            hs
        })
        .collect();

    vhsc
}

// perform set intersection of the hashsets of the vector
fn count_yes(vhsc: &Vec<HashSet<char>>) -> u32 {
    1
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-06.txt")?;

    // collapse responses into to a single line per group
    let mut surveys: Vec<String> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for l in lines {
        if l.is_empty() {
            surveys.push(one_line(&group));
            group.clear();
        } else {
            group.push(l.trim().to_string());
        }
    }
    if !group.is_empty() {
        surveys.push(one_line(&group));
    }

    let part1_data = surveys
        .iter()
        .map(|s| remove_duplicates(&s))
        .collect::<Vec<Vec<u8>>>();
    let part1 = part1_data.iter().map(|vu| vu.len()).fold(0, |acc, x| acc + x);
    println!("Part 1: {}", part1);

    let part2_data: Vec<Vec<HashSet<char>>> = surveys.iter().map(|s| make_set(&s)).collect();
    //println!("{:#?}", part2_data);
    let part2 = part2_data.iter().map(|vhsc| count_yes(&vhsc)).fold(0, |acc, x| acc + x);
    println!("Part 2: {} (placeholder, not finished)", part2);

    Ok(())
}
