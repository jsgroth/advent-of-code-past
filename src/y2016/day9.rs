//! Day 9: Explosives in Cyberspace
//!
//! <https://adventofcode.com/2016/day/9>

use crate::SimpleError;
use std::error::Error;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let total_len = input
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();

            let mut len = 0;
            let mut i = 0;
            while i < line.len() {
                let next_open_paren = match chars[i..].iter().position(|&c| c == '(') {
                    Some(index) => index + i,
                    None => break,
                };

                let next_close_paren = match chars[i..].iter().position(|&c| c == ')') {
                    Some(index) => index + i,
                    None => {
                        return Err(SimpleError::new(format!(
                            "mismatched parentheses in line: {line}"
                        )))
                    }
                };

                let (to_repeat, repetitions) =
                    parse_from_parentheses(&line[next_open_paren + 1..next_close_paren])?;

                len += (next_open_paren - i) + repetitions * to_repeat;
                i = next_close_paren + 1 + to_repeat;
            }

            len += line.len() - i;

            Ok(len)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(total_len)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let total_len = input
        .lines()
        .map(find_len_recursive)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(total_len)
}

fn find_len_recursive(s: &str) -> Result<usize, SimpleError> {
    let next_open_paren = match s.chars().position(|c| c == '(') {
        Some(index) => index,
        None => return Ok(s.len()),
    };

    let next_close_paren = match s.chars().position(|c| c == ')') {
        Some(index) => index,
        None => {
            return Err(SimpleError::new(format!(
                "mismatched parentheses in string: {s}"
            )))
        }
    };

    let (to_repeat, repetitions) =
        parse_from_parentheses(&s[next_open_paren + 1..next_close_paren])?;

    let repeated_len = repetitions
        * find_len_recursive(&s[next_close_paren + 1..next_close_paren + 1 + to_repeat])?;

    Ok(
        next_open_paren
            + repeated_len
            + find_len_recursive(&s[next_close_paren + 1 + to_repeat..])?,
    )
}

fn parse_from_parentheses(s: &str) -> Result<(usize, usize), SimpleError> {
    let (l, r) = s
        .split_once('x')
        .ok_or_else(|| SimpleError::new(format!("no 'x' inside parentheses: {s}")))?;
    let l: usize = l.parse()?;
    let r: usize = r.parse()?;

    Ok((l, r))
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample9.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample9-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(57), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(242394), solve_part_2(SAMPLE_INPUT_2));
    }
}
