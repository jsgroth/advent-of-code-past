//! Day 10: Elves Look, Elves Say
//! https://adventofcode.com/2015/day/10

use std::error::Error;
use crate::SimpleError;

fn solve_part(input: &str, iterations: usize) -> Result<usize, SimpleError> {
    let line = match input.lines().next() {
        Some(line) => line,
        None => return Err(SimpleError::new(String::from("input is empty"))),
    };

    if line.is_empty() {
        return Err(SimpleError::new(String::from("first line is empty")));
    }

    let mut current_sequence = String::from(line);
    for _ in 0..iterations {
        let mut next_sequence = String::new();

        let mut current_run: u8 = 1;
        let mut current_char = current_sequence.chars().next().unwrap();
        for c in current_sequence.chars().skip(1) {
            if current_char != c {
                next_sequence.push((current_run + ('0' as u8)) as char);
                next_sequence.push(current_char);

                current_run = 1;
                current_char = c;
            } else {
                current_run += 1;
            }
        }

        next_sequence.push((current_run + ('0' as u8)) as char);
        next_sequence.push(current_char);

        current_sequence = next_sequence;
    }

    Ok(current_sequence.len())
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, 40)?;
    let solution2 = solve_part(input, 50)?;

    Ok((solution1, solution2))
}