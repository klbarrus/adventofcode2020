// Advent of Code 2020 Day 03

use aoclib;
use std::io;

fn calc_trees(grid: &Vec<Vec<bool>>, col_delta: usize, row_delta: usize) -> usize {
    let mut col = 0;
    let mut trees = 0;
    let w = grid[0].len();

    for l in grid.iter().step_by(row_delta) {
        match l.get(col) {
            Some(true) => trees = trees + 1,
            _ => (),
        }
        col = (col + col_delta) % w;
    }

    trees
}

fn main() -> io::Result<()> {
    let lines = aoclib::read_file_lines(&"aoc-03.txt")?;

    // transform into boolean grid where true == tree present
    let grid: Vec<Vec<bool>> = lines
        .iter()
        .map(|row| row.chars().map(|ch| ch == '#').collect())
        .collect();

    println!("Part 1: {}", calc_trees(&grid, 3, 1));

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let trees: Vec<usize> = slopes
        .iter()
        .map(|(col_delta, row_delta)| calc_trees(&grid, *col_delta, *row_delta))
        .collect();

    let part2 = trees.iter().fold(1, |acc, x| acc * x);

    println!("Part 2: {}", part2);

    Ok(())
}
