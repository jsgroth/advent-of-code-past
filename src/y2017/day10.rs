//! Day 10: Knot Hash
//! https://adventofcode.com/2017/day/10

use std::error::Error;
use crate::SimpleError;
use crate::y2017::knothash;

fn solve_part_1(input: &str, list_len: usize) -> Result<usize, SimpleError> {
    let lengths = parse_input(input)?;

    let list_end = (list_len - 1) as u8;
    let mut list: Vec<_> = (0..=list_end).into_iter().collect();

    let mut position = 0;
    for (skip_size, &length) in lengths.iter().enumerate() {
        knothash::circular_reverse(&mut list, position, length);
        position = (position + length + skip_size) % list.len();
    }

    Ok(list[0] as usize * list[1] as usize)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let line = crate::read_single_line(input)?;

    let knot_hash = knothash::compute_knot_hash(line);

    Ok(knothash::to_hex_string(&knot_hash))
}

fn parse_input(input: &str) -> Result<Vec<usize>, SimpleError> {
    let line = crate::read_single_line(input)?;
    line.split(',')
        .map(|n| n.parse::<usize>().map_err(SimpleError::from))
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 256)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(12), solve_part_1("3,4,1,5", 5));
    }
}