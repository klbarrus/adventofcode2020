use aoclib;
use itertools::Itertools;
use std::io;

fn one_line(info: &Vec<String>) -> String {
    let response = info.join("");
    response
}

fn remove_duplicates(s: &str) -> Vec<u8> {
    let mut b = s.bytes().collect::<Vec<u8>>();
    b.sort();
    b.dedup();

    b
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

    let part1 = surveys
        .iter()
        .map(|s| remove_duplicates(&s))
        .collect::<Vec<Vec<u8>>>();
    let numyes = part1.iter().map(|r| r.len()).fold(0, |acc, x| acc + x);
    println!("Part 1: {}", numyes);

    Ok(())
}
