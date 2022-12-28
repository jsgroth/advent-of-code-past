//! Day 2: I Was Told There Would Be No Math
//! https://adventofcode.com/2015/day/2

use std::cmp;
use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<u32, SimpleError> {
    let result = parse_input(input)?.into_iter().map(|(l, w, h)| {
        2 * l * w + 2 * l * h + 2 * w * h + cmp::min(l * w, cmp::min(l * h, w * h))
    })
        .sum();

    Ok(result)
}

fn solve_part_2(input: &str) -> Result<u32, SimpleError> {
    let result = parse_input(input)?.into_iter().map(|(l, w, h)| {
        let smallest_perimeter = 2 * cmp::min(l + w, cmp::min(l + h, w + h));
        smallest_perimeter + l * w * h
    })
        .sum();

    Ok(result)
}

fn parse_input(input: &str) -> Result<Vec<(u32, u32, u32)>, SimpleError> {
    let dimensions: Result<Vec<_>, _> = input.lines().map(|line| {
        let split: Vec<_> = line.split("x").collect();
        match split.as_slice() {
            [l, w, h] => {
                Ok((l.parse::<u32>()?, w.parse::<u32>()?, h.parse::<u32>()?))
            },
            _ => Err(SimpleError::new(format!("unexpected line format: {line}")))
        }
    })
        .collect();

    if let Ok(dimensions) = &dimensions {
        if dimensions.is_empty() {
            return Err(SimpleError::new(String::from("input is empty")));
        }
    }

    dimensions
}

pub fn solve(input: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(58), solve_part_1("2x3x4"));
        assert_eq!(Ok(43), solve_part_1("1x1x10"));
        assert_eq!(Ok(101), solve_part_1("2x3x4\n1x1x10"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(34), solve_part_2("2x3x4"));
        assert_eq!(Ok(14), solve_part_2("1x1x10"));
        assert_eq!(Ok(48), solve_part_2("2x3x4\n1x1x10"));
    }

    #[test]
    fn test_invalid_input() {
        assert!(solve_part_1("").is_err());
        assert!(solve_part_1("asdf").is_err());
        assert!(solve_part_1("1x1").is_err());
        assert!(solve_part_1("1x1x1x1").is_err());
        assert!(solve_part_1("-1x-2x-3").is_err());
    }
}