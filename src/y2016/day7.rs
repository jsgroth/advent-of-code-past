//! Day 7: Internet Protocol Version 7
//! https://adventofcode.com/2016/day/7

use std::error::Error;
use crate::SimpleError;

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let ip_addresses = parse_input(input)?;

    let valid = ip_addresses.into_iter()
        .filter(|(outer_strings, inner_strings)| {
            outer_strings.iter().any(|s| has_abba(*s)) && !inner_strings.iter().any(|s| has_abba(*s))
        })
        .count();

    Ok(valid)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let ip_addresses = parse_input(input)?;

    let valid: Vec<_> = ip_addresses.into_iter()
        .filter(|(outer_strings, inner_strings)| {
            outer_strings.iter().any(|outer_string| {
                let chars: Vec<_> = outer_string.chars().collect();
                chars.windows(3).any(|window| {
                    window[0] == window[2] && window[0] != window[1] && inner_strings.iter().any(|inner_string| {
                        let bab: String = [window[1], window[0], window[1]].into_iter().collect();
                        inner_string.contains(&bab)
                    })
                })
            })
        })
        .collect();

    Ok(valid.len())
}

fn partition_line(line: &str) -> Result<(Vec<&str>, Vec<&str>), SimpleError> {
    let mut outer_strings = Vec::new();
    let mut inner_strings = Vec::new();

    let mut i = 0;
    while i < line.len() {
        let open_bracket_index = match line.chars().skip(i).position(|c| c == '[') {
            Some(index) => index + i,
            None => break,
        };

        outer_strings.push(&line[i..open_bracket_index]);

        let close_bracket_index = match line.chars().skip(i).position(|c| c == ']') {
            Some(index) => index + i,
            None => return Err(SimpleError::new(format!("line has an unclosed bracket: {line}"))),
        };

        inner_strings.push(&line[open_bracket_index + 1..close_bracket_index]);

        i = close_bracket_index + 1;
    }

    if i < line.len() {
        outer_strings.push(&line[i..]);
    }

    Ok((outer_strings, inner_strings))
}

fn has_abba(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    for window in chars.windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>, SimpleError> {
    input.lines().map(|line| partition_line(line)).collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample7.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample_input/sample7-2.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(Ok(2), solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(Ok(3), solve_part_2(SAMPLE_INPUT_2));
    }
}