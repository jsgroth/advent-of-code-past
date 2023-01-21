//! Day 8: Space Image Format
//! https://adventofcode.com/2019/day/8

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let line = crate::read_single_line(input)?;
    let chars: Vec<_> = line.chars().collect();

    let mut fewest_zero_digits = usize::MAX;
    let mut fewest_zero_digits_layer = Vec::new();
    for layer in chars.chunks(25 * 6) {
        let zero_digits = layer.iter().copied().filter(|&c| c == '0').count();

        if zero_digits < fewest_zero_digits {
            fewest_zero_digits = zero_digits;
            fewest_zero_digits_layer = Vec::from(layer);
        }
    }

    let ones = fewest_zero_digits_layer
        .iter()
        .copied()
        .filter(|&c| c == '1')
        .count();
    let twos = fewest_zero_digits_layer
        .iter()
        .copied()
        .filter(|&c| c == '2')
        .count();

    Ok(ones * twos)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let line = crate::read_single_line(input)?;
    let chars: Vec<_> = line.chars().collect();

    let mut image = vec![vec![false; 25]; 6];
    for (row, image_row) in image.iter_mut().enumerate() {
        for (col, value) in image_row.iter_mut().enumerate() {
            let c = chars
                .iter()
                .copied()
                .skip(25 * row + col)
                .step_by(25 * 6)
                .find(|&c| c != '2')
                .ok_or_else(|| {
                    SimpleError::new(format!("no visible pixel at row={row}, col={col}"))
                })?;
            *value = c == '1';
        }
    }

    let mut image_str = String::new();
    for (i, row) in image.iter().enumerate() {
        if i > 0 {
            image_str.push('\n');
        }

        for &b in row {
            if b {
                image_str.push('█');
            } else {
                image_str.push(' ');
            }
        }
    }

    Ok(image_str)
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
