// Advent of Code 2020 Day 08

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn run_prog(prog_data: &Vec<(&str, i64)>, start_inst: usize) -> (bool, i64) {
    let mut done: bool = false;
    let mut looping: bool = true;
    let mut curr_inst: usize = start_inst;
    let mut acc_val: i64 = 0;

    let mut already_run = HashSet::<usize>::new();
    already_run.insert(curr_inst);

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

        done = !already_run.insert(curr_inst);

        if curr_inst > prog_data.len() - 1 {
            done = true;
            looping = false;
        }
    }

    (looping, acc_val)
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-08.txt")?;

    let prog_data = lines
        .iter()
        .flat_map(|l| l.split(" "))
        .tuples()
        .map(|(a, b)| (a, b.parse::<i64>().unwrap()))
        .collect::<Vec<(_, _)>>();

    //println!("{:#?}", prog_data);

    let (_, acc_val) = run_prog(&prog_data, 0);
    println!("Part 1: {}", acc_val);

    Ok(())
}
