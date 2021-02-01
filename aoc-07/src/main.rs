// Advent of Code 2020 Day 07

use aoclib;
use itertools::Itertools;
use std::collections::HashMap;
use std::vec::Vec;
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
                    //.iter()
                    .map(|a| {
                        let v = a.splitn(2, " ").collect::<Vec<_>>();
                        if v[0] == "no" {
                            (0, "")
                        } else {
                            (v[0].parse::<i32>().unwrap(), v[1])
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    println!("{:#?}", part1_data);

    Ok(())
}
