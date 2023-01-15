//! Day 9: Encoding Error
//! https://adventofcode.com/2020/day/9

use std::cmp;
use std::error::Error;
use std::num::ParseIntError;
use crate::SimpleError;

fn solve_part_1(input: &str, preamble_size: usize) -> Result<u64, SimpleError> {
    let numbers = parse_input(input)?;

    let (_, first_invalid_number) = find_first_invalid_number(&numbers, preamble_size)?;

    Ok(first_invalid_number)
}

fn solve_part_2(input: &str, preamble_size: usize) -> Result<u64, SimpleError> {
    let numbers = parse_input(input)?;

    let (first_invalid_index, first_invalid_number) = find_first_invalid_number(&numbers, preamble_size)?;

    for i in 0..first_invalid_index {
        let mut sum = numbers[i];
        let mut smallest_in_range = numbers[i];
        let mut largest_in_range = numbers[i];
        for j in i + 1..first_invalid_index {
            sum += numbers[j];
            smallest_in_range = cmp::min(smallest_in_range, numbers[j]);
            largest_in_range = cmp::max(largest_in_range, numbers[j]);
            if sum >= first_invalid_number {
                break;
            }
        }

        if sum == first_invalid_number {
            return Ok(smallest_in_range + largest_in_range);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn find_first_invalid_number(numbers: &[u64], preamble_size: usize) -> Result<(usize, u64), SimpleError> {
    for (i, window) in numbers.windows(preamble_size + 1).enumerate() {
        let preamble = &window[..window.len() - 1];
        let target = *window.last().unwrap();
        if !is_valid_sequence(preamble, target) {
            return Ok((i + preamble_size, target));
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn is_valid_sequence(preamble: &[u64], target: u64) -> bool {
    for (i, &n) in preamble.iter().enumerate() {
        for &m in &preamble[i + 1..] {
            if n + m == target {
                return true;
            }
        }
    }

    false
}

fn parse_input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines()
        .map(|line| line.parse::<u64>())
        .collect()
}

pub fn solve(input: &str) -> Result<(u64, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 25)?;
    let solution2 = solve_part_2(input, 25)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample9.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(127), solve_part_1(SAMPLE_INPUT, 5));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(62), solve_part_2(SAMPLE_INPUT, 5));
    }
}