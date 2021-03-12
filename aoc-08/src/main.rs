// Advent of Code 2020 Day 08

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-08.txt")?;

    let prog_data = lines
        .iter()
        .flat_map(|l| l.split(" "))
        .tuples()
        .map(|(a, b)| (a, b.parse::<i64>().unwrap()))
        .collect::<Vec<(_, _)>>();

    let mut curr_inst: usize = 0;
    let mut acc_val: i64 = 0;
    let mut done: bool = false;
    let mut already_run = HashSet::<usize>::new();
    already_run.insert(0); // first line of program is always run

    //println!("{:#?}", prog_data);

    while !done {
        let curr_op: &str = prog_data[curr_inst].0;
        let curr_val = prog_data[curr_inst].1;

        match curr_op {
            "acc" => {
                acc_val += curr_val;
                curr_inst += 1;
            }
            "jmp" => {
                if curr_val >= 0 {
                    curr_inst += curr_val as usize;
                } else {
                    curr_inst -= curr_val.abs() as usize;
                }
            }
            "nop" => {
                curr_inst += 1;
            }
            _ => {
                done = true;
            }
        }

        // println!("inst: {}, acc_val: {}", curr_inst, acc_val);

        // HashSet::insert() returns true if set did not have the value
        done = !already_run.insert(curr_inst);
    }

    println!("Part 1: {}", acc_val);

    Ok(())
}
