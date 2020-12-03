use aoclib;
use regex::Regex;
use std::io;
use std::string::String;

fn validate1(lo: usize, hi: usize, ch: char, pwd: &String) -> bool {
    let count = pwd.matches(ch).count();
    lo <= count && count <= hi
}

fn validate2(lo: usize, hi: usize, ch: char, pwd: &String) -> bool {
    let mut count = 0;

    let mut idx = 0;
    for cc in pwd.chars() {
        idx = idx + 1;
        if idx == lo && cc == ch {
            count = count + 1;
        }
        if idx == hi && cc == ch {
            count = count + 1;
        }
    }

    if count == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-02.txt")?;

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut part1 = 0;
    let mut part2 = 0;
    for l in lines.iter() {
        for cap in re.captures_iter(l) {
            let lo = cap[1].parse().unwrap();
            let hi = cap[2].parse().unwrap();
            let ch = cap[3].parse().unwrap();
            let pwd = cap[4].to_string();

            if validate1(lo, hi, ch, &pwd) {
                part1 += 1;
            }

            if validate2(lo, hi, ch, &pwd) {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}
