//! Day 21: Fractal Art
//! https://adventofcode.com/2017/day/21

use std::collections::HashMap;
use std::error::Error;
use crate::SimpleError;

#[derive(Debug)]
struct Rule {
    from: Vec<Vec<bool>>,
    to: Vec<Vec<bool>>,
}

const START_PIXELS: [[bool; 3]; 3] = [
    [false, true, false],
    [false, false, true],
    [true, true, true],
];

fn solve_part(input: &str, iterations: usize) -> Result<usize, SimpleError> {
    let rules = parse_input(input)?;

    let rule_map = generate_rule_map(&rules);

    let mut pixels: Vec<Vec<_>> = START_PIXELS.iter()
        .map(|row| row.to_vec())
        .collect();

    for _ in 1..=iterations {
        let step = 2 + (pixels.len() % 2);

        let new_side_len = if step == 2 {
            pixels.len() * 3 / 2
        } else {
            pixels.len() * 4 / 3
        };

        let mut new_pixels = vec![vec![false; new_side_len]; new_side_len];

        for i in 0..(pixels.len() / step) {
            for j in 0..(pixels.len() / step) {
                let mut pixel_chunk = vec![vec![false; step]; step];
                for k in 0..step {
                    for l in 0..step {
                        pixel_chunk[k][l] = pixels[i * step + k][j * step + l];
                    }
                }

                let enhanced = rule_map.get(&pixel_chunk).ok_or_else(
                    || SimpleError::new(format!("no enhancement found for chunk: {pixel_chunk:?}"))
                )?;

                for k in 0..enhanced.len() {
                    for l in 0..enhanced.len() {
                        new_pixels[i * enhanced.len() + k][j * enhanced.len() + l] = enhanced[k][l];
                    }
                }
            }
        }

        pixels = new_pixels;
    }

    let pixel_count = pixels.iter().map(|row| {
        row.iter().filter(|&&b| b).count()
    })
        .sum();
    Ok(pixel_count)
}

fn generate_rule_map(rules: &Vec<Rule>) -> HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>> {
    let mut rule_map: HashMap<Vec<Vec<_>>, Vec<Vec<_>>> = HashMap::new();

    for Rule { from, to } in rules {
        let mut from = from.clone();
        for _ in 0..2 {
            for _ in 0..4 {
                rule_map.insert(from.clone(), to.clone());

                from = rotate_right(&from);
            }

            from = flip(&from);
        }
    }

    rule_map
}

fn rotate_right(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; grid.len()]; grid[0].len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            new_grid[j][grid.len() - 1 - i] = b;
        }
    }

    new_grid
}

fn flip(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![false; grid[0].len()]; grid.len()];

    for (i, row) in grid.iter().enumerate() {
        for (j, &b) in row.iter().enumerate() {
            new_grid[i][grid[0].len() - 1 - j] = b;
        }
    }

    new_grid
}

fn parse_input(input: &str) -> Result<Vec<Rule>, SimpleError> {
    input.lines().map(|line| {
        let (from, to) = line.split_once(" => ").ok_or_else(
            || SimpleError::new(String::from("line does not have a =>"))
        )?;

        let from = parse_rule_part(from);
        let to = parse_rule_part(to);
        Ok(Rule { from, to })
    })
        .collect()
}

fn parse_rule_part(s: &str) -> Vec<Vec<bool>> {
    s.split('/').map(|chunk| {
        chunk.chars().map(|c| c == '#').collect()
    })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part(input, 5)?;
    let solution2 = solve_part(input, 18)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample21.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(12), solve_part(SAMPLE_INPUT, 2));
    }
}