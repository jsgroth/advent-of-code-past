//! Day 25: Combo Breaker
//!
//! <https://adventofcode.com/2020/day/25>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u64, SimpleError> {
    let mut lines = input.lines();

    let public_key_1: u64 = lines
        .next()
        .ok_or_else(|| SimpleError::new(String::from("input is empty")))?
        .parse()?;

    let public_key_2: u64 = lines
        .next()
        .ok_or_else(|| SimpleError::new(String::from("input only has one line, expected two")))?
        .parse()?;

    let loop_size_1 = find_loop_size(public_key_1)?;

    let mut encryption_key = public_key_2;
    for _ in 0..loop_size_1 {
        encryption_key = (encryption_key * public_key_2) % 20201227;
    }

    Ok(encryption_key)
}

fn find_loop_size(public_key: u64) -> Result<u32, SimpleError> {
    let mut transformed_number = 7;
    for loop_size in 1.. {
        transformed_number = (transformed_number * 7) % 20201227;

        if transformed_number == public_key {
            return Ok(loop_size);
        }
    }

    Err(SimpleError::new(format!(
        "no loop size found for public key {public_key}"
    )))
}

pub fn solve(input: &str) -> Result<(u64, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;

    Ok((solution1, String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        assert_eq!(Ok(14897079), solve_part_1("5764801\n17807724"));
    }
}
