//! Day 3: Toboggan Trajectory
//!
//! <https://adventofcode.com/2020/day/3>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let map = parse_input(input)?;

    let mut tree_count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.len() {
        if map[i][j] {
            tree_count += 1;
        }

        i += 1;
        j = (j + 3) % map[0].len();
    }

    Ok(tree_count)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let map = parse_input(input)?;

    let mut tree_count_product = 1;
    for (di, dj) in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut tree_count = 0;
        let mut i = 0;
        let mut j = 0;
        while i < map.len() {
            if map[i][j] {
                tree_count += 1;
            }

            i += di;
            j = (j + dj) % map[0].len();
        }

        tree_count_product *= tree_count;
    }

    Ok(tree_count_product)
}

fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, SimpleError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    _ => Err(SimpleError::new(format!("unexpected char: {c}"))),
                })
                .collect()
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
