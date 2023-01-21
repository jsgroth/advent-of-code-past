//! Day 5: Alchemical Reduction
//! https://adventofcode.com/2018/day/5

use crate::SimpleError;
use std::cmp;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let polymer = crate::read_single_line(input)?;

    let reacted = react(polymer.chars());

    Ok(reacted.len())
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let polymer = crate::read_single_line(input)?;

    let mut min_length = usize::MAX;
    for c in 'a'..='z' {
        if !polymer.contains(c) {
            continue;
        }

        let polymer = polymer
            .chars()
            .filter(|&polymer_c| polymer_c != c && polymer_c != c.to_ascii_uppercase());
        let reacted = react(polymer);
        min_length = cmp::min(min_length, reacted.len());
    }

    Ok(min_length)
}

fn react(chars: impl Iterator<Item = char>) -> String {
    let (last_char, mut reacted) = chars.fold(
        (None, Vec::new()),
        |(last_char, mut result), next_char| match last_char {
            Some(last_char) => {
                if is_matching_pair(last_char, next_char) {
                    (result.pop(), result)
                } else {
                    result.push(last_char);
                    (Some(next_char), result)
                }
            }
            None => (Some(next_char), result),
        },
    );
    if let Some(c) = last_char {
        reacted.push(c);
    }
    reacted.into_iter().collect()
}

fn is_matching_pair(a: char, b: char) -> bool {
    (a.is_ascii_lowercase() && b == a.to_ascii_uppercase())
        || (a.is_ascii_uppercase() && b == a.to_ascii_lowercase())
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(10), solve_part_1("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(4), solve_part_2("dabAcCaCBAcCcaDA"));
    }
}
