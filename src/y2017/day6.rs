//! Day 6: Memory Reallocation
//! https://adventofcode.com/2017/day/6

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;
use std::iter;

fn solve_part(input: &str, return_cycle_len: bool) -> Result<usize, SimpleError> {
    let mut banks = parse_input(input)?;

    let mut cycles = 0;
    let mut seen_configurations: HashMap<_, _> = iter::once((banks.clone(), 0)).collect();
    loop {
        let max = banks.iter().copied().max().unwrap();
        let max_index = banks.iter().position(|&blocks| blocks == max).unwrap();

        banks[max_index] = 0;

        for i in 1..=max {
            let index = (max_index + (i as usize)) % banks.len();
            banks[index] += 1;
        }

        cycles += 1;

        if let Some(prev_seen_cycles) = seen_configurations.get(&banks) {
            return if return_cycle_len {
                Ok(cycles - prev_seen_cycles)
            } else {
                Ok(cycles)
            };
        }

        seen_configurations.insert(banks.clone(), cycles);
    }
}

fn parse_input(input: &str) -> Result<Vec<u32>, SimpleError> {
    let line = crate::read_single_line(input)?;
    let banks: Result<Vec<_>, _> = line
        .split('\t')
        .map(|blocks| blocks.parse::<u32>())
        .collect();

    let banks = banks?;

    if banks.is_empty() {
        return Err(SimpleError::new(String::from("input line is empty")));
    }

    Ok(banks)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(5), solve_part("0\t2\t7\t0", false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4), solve_part("0\t2\t7\t0", true));
    }
}
