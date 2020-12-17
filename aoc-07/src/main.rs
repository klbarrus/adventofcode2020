// Advent of Code 2020 Day 07

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-07.txt")?;

    let part1_data = lines
        .iter()
        .flat_map(|ln| ln.splitn(2, " contain "))
        .tuples()
        .map(|(a, b)| {
            (
                a.strip_suffix(" bags").unwrap(),
                b.split(", ")
                    .flat_map(|x| x.rsplitn(2, " ").skip(1).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<(_, _)>>();

    println!("{:#?}", part1_data);

    Ok(())
}
