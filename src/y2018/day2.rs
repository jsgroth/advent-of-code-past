//! Day 2: Inventory Management System
//! https://adventofcode.com/2018/day/2

use crate::SimpleError;
use std::collections::HashMap;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut two_counts = 0;
    let mut three_counts = 0;
    for line in input.lines() {
        let mut char_counts = HashMap::new();
        for c in line.chars() {
            if let Some(value) = char_counts.get_mut(&c) {
                *value += 1;
            } else {
                char_counts.insert(c, 1);
            }
        }

        if char_counts.values().any(|&count| count == 2) {
            two_counts += 1;
        }
        if char_counts.values().any(|&count| count == 3) {
            three_counts += 1;
        }
    }

    Ok(two_counts * three_counts)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    for (i, line) in input.lines().enumerate() {
        for other_line in input.lines().skip(i + 1) {
            if line.len() == other_line.len() {
                let diff_count = line
                    .chars()
                    .zip(other_line.chars())
                    .filter(|&(a, b)| a != b)
                    .count();

                if diff_count == 1 {
                    for (i, (c, other_c)) in line.chars().zip(other_line.chars()).enumerate() {
                        if c != other_c {
                            return Ok(format!("{}{}", &line[..i], &line[i + 1..]));
                        }
                    }
                }
            }
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
