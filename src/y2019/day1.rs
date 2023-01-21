//! Day 1: The Tyranny of the Rocket Equation
//!
//! <https://adventofcode.com/2019/day/1>

use crate::SimpleError;
use std::cmp;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<i32, SimpleError> {
    let masses = parse_input(input)?;

    Ok(masses.into_iter().map(|mass| mass / 3 - 2).sum())
}

fn solve_part_2(input: &str) -> Result<i32, SimpleError> {
    let masses = parse_input(input)?;

    let fuel_required = masses
        .into_iter()
        .map(|mass| {
            let mut total_fuel = 0;
            let mut mass = mass;
            while mass > 0 {
                mass = mass / 3 - 2;
                total_fuel += cmp::max(mass, 0);
            }
            total_fuel
        })
        .sum();

    Ok(fuel_required)
}

fn parse_input(input: &str) -> Result<Vec<i32>, SimpleError> {
    input
        .lines()
        .map(|line| line.parse::<i32>().map_err(SimpleError::from))
        .collect()
}

pub fn solve(input: &str) -> Result<(i32, i32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
