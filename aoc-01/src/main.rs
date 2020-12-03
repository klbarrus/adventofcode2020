// Advent of Code 2020 Day 01

use aoclib;
use std::io;

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-01.txt")?;

    let mut lines_int: Vec<u32> = Vec::new();
    for num in lines.iter() {
        lines_int.push(num.parse::<u32>().unwrap());
    }
    lines_int.sort();

    let mut found = false;
    for &n1 in lines_int.iter() {
        for &n2 in lines_int.iter() {
            if n1 + n2 > 2020 {
                break;
            }
            if n1 + n2 == 2020 {
                found = true;
                println!("found pair: {} {}", n1, n2);
                println!("Part 1: {}\n", n1 * n2);
                break;
            }
        }

        if found {
            break;
        }
    }

    let mut found = false;
    for &n1 in lines_int.iter() {
        for &n2 in lines_int.iter() {
            for &n3 in lines_int.iter() {
                if n1 + n2 + n3 > 2020 {
                    break;
                }
                if n1 + n2 + n3 == 2020 {
                    found = true;
                    println!("found triplet: {} {} {}", n1, n2, n3);
                    println!("Part 2: {}", n1 * n2 * n3);
                    break;
                }
            }

            if found {
                break;
            }
        }

        if found {
            break;
        }
    }

    Ok(())
}
