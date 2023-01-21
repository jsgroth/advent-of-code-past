//! Day 10: Adapter Array
//!
//! <https://adventofcode.com/2020/day/10>

use crate::SimpleError;
use std::error::Error;
use std::num::ParseIntError;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let mut numbers = parse_input(input)?;

    numbers.push(0);
    numbers.sort();

    let mut one_diffs = 0;
    let mut three_diffs = 1;
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        match diff {
            1 => one_diffs += 1,
            3 => three_diffs += 1,
            _ => {}
        }
    }

    Ok(one_diffs * three_diffs)
}

fn solve_part_2(input: &str) -> Result<u64, SimpleError> {
    let mut numbers = parse_input(input)?;

    numbers.push(0);
    numbers.push(*numbers.iter().max().unwrap() + 3);
    numbers.sort();

    Ok(count_valid_combinations(&numbers))
}

fn count_valid_combinations(numbers: &[u32]) -> u64 {
    let mut sublist_counts = vec![0; numbers.len()];

    *sublist_counts.last_mut().unwrap() = 1;

    for i in (0..numbers.len() - 1).rev() {
        let mut combinations = 0;
        for j in 1..=3 {
            if i + j < numbers.len() && numbers[i + j] <= numbers[i] + 3 {
                combinations += sublist_counts[i + j];
            }
        }
        sublist_counts[i] = combinations;
    }

    sublist_counts[0]
}

fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|line| line.parse::<u32>()).collect()
}

pub fn solve(input: &str) -> Result<(u32, u64), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample10.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample10-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(35), solve_part_1(SAMPLE_INPUT));
        assert_eq!(Ok(220), solve_part_1(SAMPLE_INPUT_2));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(8), solve_part_2(SAMPLE_INPUT));
        assert_eq!(Ok(19208), solve_part_2(SAMPLE_INPUT_2));
    }
}
