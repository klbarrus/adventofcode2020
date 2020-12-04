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

fn parse_passport_1(ppt_str: &str) -> Option<HashMap<&str, &str>> {
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

fn parse_passport_2(passport: &HashMap<&str, &str>) -> bool {
  passport.iter().all(|(&k, v)| match k {
    "byr" => {
      let year = v.parse().unwrap_or(0);
      1920 <= year && year <= 2002
    },
    "iyr" => {
      let year = v.parse().unwrap_or(0);
      2010 <= year && year <= 2020
    },
    "eyr" => {
      let year = v.parse().unwrap_or(0);
      2020 <= year && year <= 2030
    },
    "hgt" => {
      let height = v[0..(v.len()-2)].parse().unwrap_or(0);
      match &v[(v.len()-2)..] {
        "cm" => 150 <= height && height <= 193,
        "in" => 59 <= height && height <= 76,
        _ => false
      }
    }
    "hcl" => {
      v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|ch| ch.is_ascii_hexdigit())
    },
    "ecl" => {
      ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)
    },
    "pid" => {
      v.len() == 9 && v.chars().all(|ch| ch.is_ascii_digit())
    },
    "cid" => true,
    _ => true,
  })
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-04.txt")?;

    // collapse passport into to a single line per passport
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

    // parse each passport for validity
    let valid_passports_1: Vec<_> = all_ppt_info
        .iter()
        .filter_map(|p| parse_passport_1(&p))
        .collect();

    // println!("{:#?}", valid_passports_1);
    println!("Part 1: {}", valid_passports_1.len());

    // extra checking on valid passports
    let valid_passports_2: Vec<_> = valid_passports_1
    .iter()
    .filter(|p| parse_passport_2(p))
    .collect();

    println!("Part 2: {}", valid_passports_2.len());

    Ok(())
}
