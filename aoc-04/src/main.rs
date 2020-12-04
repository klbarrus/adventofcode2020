// Advent of Code 2020 Day 04

use aoclib;
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::string::String;

fn passport_info(info: &Vec<String>) -> String {
    let passport = info.join(" ");
    passport
}

fn parse_passport(ppt_str: &str) -> Option<HashMap<&str, &str>> {
    let passport: HashMap<_, _> = ppt_str
        .split_whitespace()
        .flat_map(|s| s.split(':'))
        .tuples()
        .collect();

    let valid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|f| passport.contains_key(f));

    if valid {
        return Some(passport);
    }

    None
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-04.txt")?;

    // convert passport into to a single line for each passport
    let mut all_ppt_info: Vec<String> = Vec::new();
    let mut one_ppt: Vec<String> = Vec::new();
    for l in lines {
        if l.is_empty() {
            all_ppt_info.push(passport_info(&one_ppt));
            one_ppt.clear();
        } else {
            one_ppt.push(l.trim().to_string());
        }
    }
    if !one_ppt.is_empty() {
        all_ppt_info.push(passport_info(&one_ppt));
    }

    // parse each passport line for validity
    let valid_passports: Vec<_> = all_ppt_info
        .iter()
        .filter_map(|p| parse_passport(&p))
        .collect();

//    println!("{:#?}", valid_passports);

    println!("Part 1: {}", valid_passports.len());

    Ok(())
}
