// Advent of Code 2020 Day 08

use aoclib;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn run_prog(
    prog_data: &Vec<(&str, i64)>,
    start_inst: usize,
    error_inst: Option<usize>,
) -> (bool, i64) {
    let mut done: bool = false;
    let mut looping: bool = true;   // simulation exited due to loop detection
    let mut curr_inst: usize = start_inst;
    let mut acc_val: i64 = 0;

    let mut already_run = HashSet::<usize>::new();
    already_run.insert(curr_inst);

    while !done {
        let mut curr_op: &str = prog_data[curr_inst].0;
        let curr_val = prog_data[curr_inst].1;

        // if the current instruction is the simulate error, flip the instruction
        if Some(curr_inst) == error_inst {
            if curr_op == "jmp" {
                curr_op = "nop";
            } else {
                curr_op = "jmp";
            }
        }

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
            // simulation exited at the end of the program, set looping false
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

    // println!("{:#?}", prog_data);

    let (_, acc_val) = run_prog(&prog_data, 0, None);
    println!("Part 1: {}", acc_val);

    for i in 0..prog_data.len() {
        if prog_data[i].0 == "nop" || prog_data[i].0 == "jmp" {
            let (looping, acc_val) = run_prog(&prog_data, 0, Some(i));

            if !looping {
                println!("Part 2: {}", acc_val);
                println!("  error was instruction {}: {}", i, prog_data[i].0);
                break;
            }
        }
    }

    Ok(())
}
