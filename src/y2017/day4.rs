//! Day 4: High-Entropy Passphrases
//! https://adventofcode.com/2017/day/4

use crate::SimpleError;
use std::collections::HashSet;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let valid_count = input
        .lines()
        .filter(|line| {
            let mut words = HashSet::new();
            for word in line.split(' ') {
                if !words.insert(word) {
                    return false;
                }
            }
            true
        })
        .count();

    Ok(valid_count)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let valid_count = input
        .lines()
        .filter(|line| {
            let mut words = HashSet::new();
            for word in line.split(' ') {
                let mut word_chars: Vec<_> = word.chars().collect();
                word_chars.sort();
                if !words.insert(word_chars) {
                    return false;
                }
            }
            true
        })
        .count();

    Ok(valid_count)
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
