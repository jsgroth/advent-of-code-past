//! Day 2: Password Philosophy
//!
//! <https://adventofcode.com/2020/day/2>

use crate::SimpleError;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Password {
    rule_char: char,
    min_allowed: u32,
    max_allowed: u32,
    text: String,
}

fn solve_part_1(input: &str) -> Result<usize, SimpleError> {
    let passwords = parse_input(input)?;

    let valid_count = passwords
        .iter()
        .filter(|password| {
            let rule_char_count = password
                .text
                .chars()
                .filter(|&c| c == password.rule_char)
                .count() as u32;

            rule_char_count >= password.min_allowed && rule_char_count <= password.max_allowed
        })
        .count();

    Ok(valid_count)
}

fn solve_part_2(input: &str) -> Result<usize, SimpleError> {
    let passwords = parse_input(input)?;

    let valid_count = passwords
        .iter()
        .filter(|password| {
            let password_chars: Vec<_> = password.text.chars().collect();

            let first_is_rule_char =
                password_chars[password.min_allowed as usize - 1] == password.rule_char;
            let last_is_rule_char =
                password_chars[password.max_allowed as usize - 1] == password.rule_char;

            first_is_rule_char ^ last_is_rule_char
        })
        .count();

    Ok(valid_count)
}

fn parse_input(input: &str) -> Result<Vec<Password>, SimpleError> {
    input
        .lines()
        .map(|line| {
            let (rule, password_text) = line
                .split_once(": ")
                .ok_or_else(|| SimpleError::new(format!("line contains no ': ': {line}")))?;

            let (rule_range, rule_char) = rule.split_once(' ').ok_or_else(|| {
                SimpleError::new(format!("rule in line contains no space: {line}"))
            })?;

            let (min_allowed, max_allowed) = rule_range.split_once('-').ok_or_else(|| {
                SimpleError::new(format!("rule range in line contains no dash: {line}"))
            })?;

            let rule_char = rule_char.parse()?;
            let min_allowed = min_allowed.parse()?;
            let max_allowed = max_allowed.parse()?;
            let text = String::from(password_text);

            Ok(Password {
                rule_char,
                min_allowed,
                max_allowed,
                text,
            })
        })
        .collect()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let solution1 = solve_part_1(input)?;
    let solution2 = solve_part_2(input)?;

    Ok((solution1, solution2))
}
