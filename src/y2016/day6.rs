//! Day 6: Signals and Noise
//! https://adventofcode.com/2016/day/6

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

fn solve_part(input: &str, find_min: bool) -> Result<String, SimpleError> {
    let chars = parse_input(input);
    if chars.is_empty() {
        return Err(SimpleError::new(String::from("input is empty")));
    }

    let mut result = String::new();
    for j in 0..chars[0].len() {
        let mut char_counts = HashMap::new();
        for row in &chars {
            let c = row[j];
            if let Some(count) = char_counts.get_mut(&c) {
                *count += 1;
            } else {
                char_counts.insert(c, 1);
            }
        }

        let (c, _) = if find_min {
            char_counts.into_iter().min_by_key(|&(_, count)| count).unwrap()
        } else {
            char_counts.into_iter().max_by_key(|&(_, count)| count).unwrap()
        };
        result.push(c);
    }

    Ok(result)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| {
        line.chars().collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample6.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("easter")), solve_part(SAMPLE_INPUT, false));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(String::from("advent")), solve_part(SAMPLE_INPUT, true));
    }
}