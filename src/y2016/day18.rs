//! Day 18: Like a Rogue
//! https://adventofcode.com/2016/day/18

use std::error::Error;
use std::iter;
use crate::SimpleError;

fn solve_part(input: &str, row_target: usize) -> Result<usize, SimpleError> {
    let first_line_traps = parse_input(input)?;

    let mut prev_line = first_line_traps.clone();
    let mut safe_count = first_line_traps.iter().filter(|&&b| !b).count();
    let mut row_count = 1;
    while row_count < row_target {
        prev_line = generate_new_line(&prev_line);
        safe_count += prev_line.iter().filter(|&&b| !b).count();
        row_count += 1;
    }

    Ok(safe_count)
}

fn generate_new_line(line: &[bool]) -> Vec<bool> {
    let extended_line: Vec<_> = iter::once(false)
        .chain(line.iter().copied())
        .chain(iter::once(false))
        .collect();

    extended_line.windows(3).map(|window| {
        matches!(window, [true, true, false] | [false, true, true] | [true, false, false] | [false, false, true])
    })
        .collect()
}

fn parse_input(input: &str) -> Result<Vec<bool>, SimpleError> {
    let first_line = crate::read_single_line(input)?;
    first_line.chars().map(|c| {
        match c {
            '^' => Ok(true),
            '.' => Ok(false),
            _ => Err(SimpleError::new(format!("invalid char '{c}' in line: {first_line}")))
        }
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, 40)?;
    let solution2 = solve_part(input, 400000)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(38), solve_part(".^^.^.^^^^", 10));
    }
}