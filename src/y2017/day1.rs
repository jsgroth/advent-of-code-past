//! Day 1: Inverse Captcha
//!
//! <https://adventofcode.com/2017/day/1>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let mut captcha = parse_input(input)?;
    captcha.push(captcha[0]);

    let sum = captcha
        .windows(2)
        .filter_map(|window| {
            if window[0] == window[1] {
                Some(window[0])
            } else {
                None
            }
        })
        .sum();

    Ok(sum)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let captcha = parse_input(input)?;

    let mut sum = 0;
    for i in 0..captcha.len() {
        if captcha[i] == captcha[(i + captcha.len() / 2) % captcha.len()] {
            sum += captcha[i];
        }
    }

    Ok(sum)
}

fn parse_input(input: &str) -> Result<Vec<usize>, SimpleError> {
    Ok(crate::read_single_line(input)?
        .chars()
        .map(|c| c as usize - '0' as usize)
        .collect())
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
