// Advent of Code 2020 Day 07

use aoclib;
use itertools::Itertools;
use std::collections::HashMap;
use std::vec::Vec;
use std::io;

fn contain_goldbag(bag: &str, data: &HashMap<&str, Vec<(i32, &str)>>) -> bool {
    //println!("looking into {}", bag);
    if bag == "shiny gold" {
        return true;
    }
    if bag.is_empty() {
        return false;
    }

    match data.get(bag) {
        Some(v) => {
            let mut overall = false;
            for a in v {
                let res = contain_goldbag(a.1, data);
                if res {
                    overall = true;
                }
            };
            overall
        },
        None => {
            println!("bag {} not present?!", bag);
            false
        }
    }
}

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

    //println!("{:#?}", part1_data);

    let mut part1_count = 0;
    for bag in part1_data.keys() {
        if bag == &"shiny gold" {
            // shiny gold bag doesn't contain itself
        } else {
            println!("top level {}", bag);
            if contain_goldbag(bag, &part1_data) {
                println!("  yes");
                part1_count = part1_count + 1;
            }
        }
    }

    println!("Part 1: {}", part1_count);

    Ok(())
}
