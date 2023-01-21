//! Day 15: Dueling Generators
//! https://adventofcode.com/2017/day/15

use crate::SimpleError;
use std::error::Error;

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;

fn solve_part(input: &str, wait_for_multiples: bool, rounds: usize) -> Result<usize, SimpleError> {
    let start_values: Result<Vec<_>, _> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .last()
                .ok_or_else(|| SimpleError::new(String::from("line is empty")))?
                .parse::<u64>()
                .map_err(SimpleError::from)
        })
        .collect();
    let start_values = start_values?;
    if start_values.len() != 2 {
        return Err(SimpleError::new(format!(
            "input has {} lines, expected 2",
            start_values.len()
        )));
    }

    let mut gen_a = start_values[0];
    let mut gen_b = start_values[1];

    let mut match_count = 0;
    for _ in 0..rounds {
        gen_a = generate_next_value(gen_a, A_FACTOR);
        gen_b = generate_next_value(gen_b, B_FACTOR);

        if wait_for_multiples {
            while (gen_a % 4) != 0 {
                gen_a = generate_next_value(gen_a, A_FACTOR);
            }

            while (gen_b % 8) != 0 {
                gen_b = generate_next_value(gen_b, B_FACTOR);
            }
        }

        if (gen_a & 0xFFFF) == (gen_b & 0xFFFF) {
            match_count += 1;
        }
    }

    Ok(match_count)
}

#[inline]
fn generate_next_value(value: u64, factor: u64) -> u64 {
    (value * factor) % 2147483647
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false, 40_000_000)?;
    let solution2 = solve_part(input, true, 5_000_000)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(
            Ok(588),
            solve_part(
                "Generator A starts with 65\nGenerator B starts with 8921",
                false,
                40_000_000
            )
        );
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(
            Ok(309),
            solve_part(
                "Generator A starts with 65\nGenerator B starts with 8921",
                true,
                5_000_000
            )
        );
    }
}
