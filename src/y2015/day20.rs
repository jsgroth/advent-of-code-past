//! Day 20: Infinite Elves and Infinite Houses
//!
//! <https://adventofcode.com/2015/day/20>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;

    let target = line.parse::<u32>()? / 10;

    let mut houses = vec![1; target as usize];

    for elf in 2..target {
        for i in (elf..target).step_by(elf as usize) {
            houses[i as usize] += elf;
        }
    }

    for (i, &house) in houses.iter().enumerate() {
        if house >= target {
            return Ok(i);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;

    let target = line.parse::<u32>()?;

    let mut houses = vec![0; target as usize];

    for elf in 1..(target / 11) {
        for i in (elf..target / 11).step_by(elf as usize).take(50) {
            houses[i as usize] += 11 * elf;
        }
    }

    for (i, &house) in houses.iter().enumerate() {
        if house >= target {
            return Ok(i);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
