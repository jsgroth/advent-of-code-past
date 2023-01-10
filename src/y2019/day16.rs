//! Day 16: Flawed Frequency Transmission
//! https://adventofcode.com/2019/day/16

use std::error::Error;
use std::iter;
use crate::SimpleError;

fn solve_part_1(input: &str, phases: usize) -> Result<String, SimpleError> {
    let numbers = crate::read_single_line(input)?;

    let mut digits = to_digits(numbers)?;

    for _ in 0..phases {
        digits = fft(&digits);
    }

    let first_digits = digits_to_string(&digits[..8]);

    Ok(first_digits)
}

fn solve_part_2(input: &str) -> Result<String, SimpleError> {
    let numbers = crate::read_single_line(input)?;

    let digits = to_digits(numbers)?;

    let offset: usize = numbers[..7].parse()?;

    if offset <= 10000 * digits.len() / 2 {
        return Err(SimpleError::new(
            format!("expected offset to be in second half of expanded list; offset={offset}, len={}", 10000 * digits.len())
        ));
    }

    let mut expanded_digits: Vec<_> = digits.iter().copied()
        .cycle()
        .skip(offset % digits.len())
        .take(10000 * digits.len() - offset)
        .collect();

    for _ in 0..100 {
        expanded_digits = cheat_fft(&expanded_digits);
    }

    let message = digits_to_string(&expanded_digits[..8]);

    Ok(message)
}

fn fft(digits: &[i32]) -> Vec<i32> {
    let mut transformed_digits = Vec::with_capacity(digits.len());

    for i in 1..=digits.len() {
        let sum: i32 = pattern_iter(i).zip(digits.iter().copied())
            .map(|(a, b)| a * b)
            .sum();
        let digit = sum.abs() % 10;
        transformed_digits.push(digit);
    }

    transformed_digits
}

// This function assumes that these digits are in the second half of a larger signal
fn cheat_fft(digits: &[i32]) -> Vec<i32> {
    let mut transformed_digits_rev = Vec::new();

    let mut running_sum = 0;
    for digit in digits.iter().copied().rev() {
        running_sum = (running_sum + digit) % 10;
        transformed_digits_rev.push(running_sum);
    }

    transformed_digits_rev.into_iter().rev().collect()
}

fn pattern_iter(position: usize) -> impl Iterator<Item = i32> {
    iter::repeat(0).take(position)
        .chain(iter::repeat(1).take(position))
        .chain(iter::repeat(0).take(position))
        .chain(iter::repeat(-1).take(position))
        .cycle()
        .skip(1)
}

fn to_digits(s: &str) -> Result<Vec<i32>, SimpleError> {
    s.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|digit| digit as i32)
                .ok_or(SimpleError::new(format!("{c} is not a digit")))
        })
        .collect()
}

fn digits_to_string(digits: &[i32]) -> String {
    digits.iter().copied()
        .map(|digit| char::from_digit(digit as u32, 10).unwrap())
        .collect()
}

pub fn solve(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let solution1 = solve_part_1(input, 100)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(String::from("48226158")), solve_part_1("12345678", 1));
        assert_eq!(Ok(String::from("34040438")), solve_part_1("12345678", 2));
        assert_eq!(Ok(String::from("03415518")), solve_part_1("12345678", 3));
        assert_eq!(Ok(String::from("01029498")), solve_part_1("12345678", 4));

        assert_eq!(Ok(String::from("24176176")), solve_part_1("80871224585914546619083218645595", 100));
        assert_eq!(Ok(String::from("73745418")), solve_part_1("19617804207202209144916044189917", 100));
        assert_eq!(Ok(String::from("52432133")), solve_part_1("69317163492948606335995924319873", 100));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(String::from("84462026")), solve_part_2("03036732577212944063491565474664"));
        assert_eq!(Ok(String::from("78725270")), solve_part_2("02935109699940807407585447034323"));
        assert_eq!(Ok(String::from("53553731")), solve_part_2("03081770884921959731165446850517"));
    }
}