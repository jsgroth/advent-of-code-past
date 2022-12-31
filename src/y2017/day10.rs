//! Day 10: Knot Hash
//! https://adventofcode.com/2017/day/10

use std::error::Error;
use std::ops::BitXor;
use crate::SimpleError;

fn solve_part_1(input: &str, list_len: usize) -> Result<usize, SimpleError> {
    let lengths = parse_input(input)?;

    let mut list: Vec<_> = (0..list_len).into_iter().collect();

    let mut position = 0;
    let mut skip_size = 0;
    for &length in &lengths {
        reverse(&mut list, position, length);
        position = (position + length + skip_size) % list.len();
        skip_size += 1;
    }

    Ok(list[0] * list[1])
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let line = crate::read_single_line(input)?;

    let mut lengths: Vec<_> = line.chars().map(|c| c as usize).collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut list: Vec<_> = (0..256).into_iter().collect();

    let mut position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for &length in &lengths {
            reverse(&mut list, position, length);
            position = (position + length + skip_size) % list.len();
            skip_size += 1;
        }
    }

    let result: String = list.chunks(16)
        .map(|chunk| {
            let xor = chunk.iter().copied()
                .reduce(|a, b| a.bitxor(b))
                .unwrap();
            format!("{xor:02x}")
        })
        .collect();

    Ok(result)
}

fn reverse(list: &mut Vec<usize>, mut i: usize, length: usize) {
    let mut j = (i + length + list.len() - 1) % list.len();

    for _ in 0..(length / 2) {
        list.swap(i, j);
        i = (i + 1) % list.len();
        j = (j + list.len() - 1) % list.len();
    }
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

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(String::from("a2582a3a0e66e6e86e3812dcb672a272")), solve_part_2("\n"));
        assert_eq!(Ok(String::from("33efeb34ea91902bb2f59c9920caa6cd")), solve_part_2("AoC 2017"));
        assert_eq!(Ok(String::from("3efbe78a8d82f29979031a4aa0b16a9d")), solve_part_2("1,2,3"));
        assert_eq!(Ok(String::from("63960835bcdc130f0b66d7ff4f6a5a8e")), solve_part_2("1,2,4"));
    }
}