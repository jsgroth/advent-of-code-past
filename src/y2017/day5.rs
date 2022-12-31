//! Day 5: A Maze of Twisty Trampolines, All Alike
//! https://adventofcode.com/2017/day/5

use std::error::Error;
use std::num::ParseIntError;
use crate::SimpleError;

fn solve_part(input: &str, part_two: bool) -> Result<usize, SimpleError> {
    let mut jumps = parse_input(input)?;

    let mut i = 0;
    let mut steps = 0;
    while i < jumps.len() as i32 {
        let offset = jumps[i as usize];

        if part_two && offset >= 3 {
            jumps[i as usize] -= 1;
        } else {
            jumps[i as usize] += 1;
        }

        i += offset;
        steps += 1;
    }

    Ok(steps)
}

fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}