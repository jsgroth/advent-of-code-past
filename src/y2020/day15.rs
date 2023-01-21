//! Day 15: Rambunctious Recitation
//! https://adventofcode.com/2020/day/15

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str, nth_number: usize) -> Result<usize, SimpleError> {
    let starting_numbers = parse_input(input)?;

    let mut last_spoken = vec![0; nth_number];
    for (i, &n) in starting_numbers.iter().enumerate() {
        last_spoken[n] = i + 1;
    }

    let start = starting_numbers.len() + 1;
    let mut last_number = *starting_numbers.last().unwrap();
    for i in start..=nth_number {
        let next_number = match last_spoken[last_number] {
            0 => 0,
            prev_turn => i - 1 - prev_turn,
        };
        last_spoken[last_number] = i - 1;
        last_number = next_number;
    }

    Ok(last_number)
}

fn parse_input(input: &str) -> Result<Vec<usize>, SimpleError> {
    crate::read_single_line(input)?
        .split(',')
        .map(|s| s.parse::<usize>().map_err(SimpleError::from))
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 2020)?;
    let solution2 = solve_part_1(input, 30000000)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(436), solve_part_1("0,3,6", 2020));
    }
}
