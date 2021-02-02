// Advent of Code 2020 Day 07

use aoclib;
use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::vec::Vec;

fn contains_goldbag(bag: &str, allbags: &HashMap<&str, Vec<(i32, &str)>>) -> bool {
    //println!("looking into {}", bag);
    if bag == "shiny gold" {
        return true;
    }
    if bag.is_empty() {
        return false;
    }

    match allbags.get(bag) {
        Some(baglist) => {
            let mut any_goldbag = false;
            for bag_info in baglist {
                let has_goldbag = contains_goldbag(bag_info.1, allbags);
                if has_goldbag {
                    any_goldbag = true;
                }
            }
            any_goldbag
        }
        None => {
            println!("bag {} not present?!", bag);
            false
        }
    }
}

fn count_bags(bag: &str, allbags: &HashMap<&str, Vec<(i32, &str)>>) -> i32 {
    match allbags.get(bag) {
        Some(baglist) => {
            let mut count = 0;
            for bag_info in baglist {
                if bag_info.0 != 0 {
                    count = count + bag_info.0 * (1 + count_bags(bag_info.1, allbags));
                }
            }
            count
        }
        None => {
            println!("bag {} not present?!", bag);
            0
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
            //println!("top level {}", bag);
            if contains_goldbag(bag, &part1_data) {
                //println!("  yes");
                part1_count = part1_count + 1;
            }
        }
    }

    println!("Part 1: {}", part1_count);

    let part2_count = count_bags(&"shiny gold", &part1_data);

    println!("Part 2: {}", part2_count);

    Ok(())
}
