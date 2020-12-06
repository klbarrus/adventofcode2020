// Advent of Code 2020 Day 05

use aoclib;
use itertools::Itertools;
use itertools::MinMaxResult;
use std::io;

// seat row and column are disguised binary numbers
// where F/L are 0 and B/R are 1
fn parse_pass(s: &str) -> (u16, u16, u16) {
    let row: Vec<u16> = s[0..7]
        .chars()
        .map(|c| if c == 'F' { 0 } else { 1 })
        .collect();
    let col: Vec<u16> = s[7..10]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    let r = row.iter().fold(0, |acc, x| acc * 2 + x);
    let c = col.iter().fold(0, |acc, x| acc * 2 + x);
    let id = r * 8 + c;

    (r, c, id)
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-05.txt")?;

    let passes: Vec<(u16, u16, u16)> = lines.iter().map(|s| parse_pass(s)).collect();

    let minmax = passes.iter().minmax_by_key(|(_r, _c, id)| id);
    let (minid, maxid) = match minmax {
        MinMaxResult::MinMax(a, b) => (a.2, b.2),
        _ => (0, 0),
    };

    println!("Part 1: {}", maxid);

    let mut seatids = passes.iter().map(|(_r, _c, id)| id).collect::<Vec<&u16>>();
    seatids.sort();

    // search from the first seat
    let mut myseat = minid;
    loop {
        myseat = myseat + 1;
        if !seatids.contains(&&myseat) {
            println!("Part 2: {}", myseat);
            break;
        }
    }

    Ok(())
}
