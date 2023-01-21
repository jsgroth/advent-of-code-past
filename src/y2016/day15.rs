//! Day 15: Timing is Everything
//! https://adventofcode.com/2016/day/15

use crate::SimpleError;
use std::error::Error;

#[derive(Debug)]
struct Disc {
    num_positions: usize,
    initial_position: usize,
}

impl Disc {
    fn compute_position(&self, time: usize) -> usize {
        (self.initial_position + time) % self.num_positions
    }
}

fn solve_part(input: &str, add_last_disc: bool) -> Result<usize, SimpleError> {
    let mut discs = parse_input(input)?;
    if add_last_disc {
        discs.push(Disc {
            num_positions: 11,
            initial_position: 0,
        })
    }

    for t in 0.. {
        if discs
            .iter()
            .enumerate()
            .all(|(i, disc)| disc.compute_position(t + i) == 0)
        {
            return Ok(t - 1);
        }
    }

    Err(SimpleError::new(String::from("no solution found")))
}

fn parse_input(input: &str) -> Result<Vec<Disc>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(' ').collect();
            if split.len() != 12 {
                return Err(SimpleError::new(format!("invalid line format: {line}")));
            }

            let num_positions: usize = split[3].parse()?;
            let initial_position: usize = split[11][..split[11].len() - 1].parse()?;

            Ok(Disc {
                num_positions,
                initial_position,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, false)?;
    let solution2 = solve_part(input, true)?;

    Ok((solution1, solution2))
}
